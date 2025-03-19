//! Buffer manager - Manages multiple buffers and provides access to them
//!
//! This module implements the buffer management system for xvim, which is responsible
//! for creating, tracking, and managing multiple buffers.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
// use std::fs::File;
use std::error::Error;
use std::fmt;
use std::io::Write;

use super::{Buffer, BufferError, BufferResult};

/// Errors that can occur during buffer management operations
#[derive(Debug)]
pub enum BufferManagerError {
    /// Buffer with the specified ID not found
    BufferNotFound(usize),
    /// Buffer with the specified path not found
    FileNotFound(PathBuf),
    /// Error from buffer operations
    BufferError(BufferError),
    /// I/O error
    Io(std::io::Error),
    /// Other errors
    Other(String),
}

impl fmt::Display for BufferManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BufferManagerError::BufferNotFound(id) => write!(f, "Buffer {} not found", id),
            BufferManagerError::FileNotFound(path) => write!(f, "No buffer for file {:?}", path),
            BufferManagerError::BufferError(err) => write!(f, "Buffer error: {}", err),
            BufferManagerError::Io(err) => write!(f, "I/O error: {}", err),
            BufferManagerError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for BufferManagerError {}

impl From<BufferError> for BufferManagerError {
    fn from(err: BufferError) -> Self {
        BufferManagerError::BufferError(err)
    }
}

impl From<std::io::Error> for BufferManagerError {
    fn from(err: std::io::Error) -> Self {
        BufferManagerError::Io(err)
    }
}

/// Result type for buffer manager operations
pub type BufferManagerResult<T> = Result<T, BufferManagerError>;

/// Manages multiple buffers and provides access to them
#[derive(Clone)]
pub struct BufferManager {
    /// Map of buffer IDs to buffers
    buffers: HashMap<usize, Buffer>,
    /// Map of file paths to buffer IDs
    path_to_id: HashMap<PathBuf, usize>,
    /// The next buffer ID to assign
    next_id: usize,
    /// The current buffer ID
    current_id: Option<usize>,
}

impl BufferManager {
    /// Create a new buffer manager
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
            path_to_id: HashMap::new(),
            next_id: 1,
            current_id: None,
        }
    }
    
    /// Create a new empty buffer
    pub fn create_buffer(&mut self) -> BufferManagerResult<usize> {
        let id = self.next_id;
        self.next_id += 1;
        
        let buffer = Buffer::new(id);
        self.buffers.insert(id, buffer);
        
        if self.current_id.is_none() {
            self.current_id = Some(id);
        }
        
        Ok(id)
    }
    
    /// Open a file in a new buffer
    pub fn open_file<P: AsRef<Path>>(&mut self, path: P) -> BufferManagerResult<usize> {
        let path_buf = path.as_ref().to_path_buf();
        
        // Check if the file is already open
        if let Some(&id) = self.path_to_id.get(&path_buf) {
            self.current_id = Some(id);
            return Ok(id);
        }
        
        // Create a new buffer for the file
        let id = self.next_id;
        self.next_id += 1;
        
        // Try to open the file, but if it doesn't exist, create a new buffer with that name
        let buffer = match Buffer::from_file(id, &path_buf) {
            Ok(buffer) => buffer,
            Err(err) => {
                // If the file doesn't exist, create a new buffer with that name
                if let BufferError::Io(io_err) = &err {
                    if io_err.kind() == std::io::ErrorKind::NotFound {
                        // Create a new buffer
                        let mut buffer = Buffer::new(id);
                        
                        // Set the file path and name
                        if let Some(file_name) = path_buf.file_name().and_then(|n| n.to_str()) {
                            buffer.name = file_name.to_string();
                        }
                        buffer.file_path = Some(path_buf.clone());
                        buffer.flags.new_file = true;
                        
                        buffer
                    } else {
                        // Other I/O error
                        return Err(BufferManagerError::BufferError(err));
                    }
                } else {
                    // Other buffer error
                    return Err(BufferManagerError::BufferError(err));
                }
            }
        };
        
        self.buffers.insert(id, buffer);
        self.path_to_id.insert(path_buf, id);
        
        self.current_id = Some(id);
        
        Ok(id)
    }
    
    /// Get a reference to a buffer by ID
    pub fn get_buffer(&self, id: usize) -> BufferManagerResult<&Buffer> {
        self.buffers.get(&id).ok_or(BufferManagerError::BufferNotFound(id))
    }
    
    /// Get a mutable reference to a buffer by ID
    pub fn get_buffer_mut(&mut self, id: usize) -> BufferManagerResult<&mut Buffer> {
        self.buffers.get_mut(&id).ok_or(BufferManagerError::BufferNotFound(id))
    }
    
    /// Get a reference to the current buffer
    pub fn current_buffer(&self) -> BufferManagerResult<&Buffer> {
        match self.current_id {
            Some(id) => self.get_buffer(id),
            None => Err(BufferManagerError::Other("No current buffer".to_string())),
        }
    }
    
    /// Get a mutable reference to the current buffer
    pub fn current_buffer_mut(&mut self) -> BufferManagerResult<&mut Buffer> {
        match self.current_id {
            Some(id) => self.get_buffer_mut(id),
            None => Err(BufferManagerError::Other("No current buffer".to_string())),
        }
    }
    
    /// Set the current buffer by ID
    pub fn set_current_buffer(&mut self, id: usize) -> BufferManagerResult<()> {
        if !self.buffers.contains_key(&id) {
            return Err(BufferManagerError::BufferNotFound(id));
        }
        
        self.current_id = Some(id);
        Ok(())
    }
    
    /// Get the current buffer ID
    pub fn current_buffer_id(&self) -> Option<usize> {
        self.current_id
    }
    
    /// Get a buffer ID by file path
    pub fn get_buffer_id_by_path<P: AsRef<Path>>(&self, path: P) -> Option<usize> {
        self.path_to_id.get(path.as_ref()).copied()
    }
    
    /// Get a buffer name by ID
    pub fn get_buffer_name(&self, buffer_id: usize) -> Option<String> {
        self.buffers.get(&buffer_id).map(|buffer| buffer.name().to_string())
    }
    
    /// Set a buffer name by ID
    pub fn set_buffer_name(&mut self, buffer_id: usize, name: &str) -> BufferManagerResult<()> {
        let buffer = self.get_buffer_mut(buffer_id)?;
        buffer.set_name(name.to_string());
        Ok(())
    }
    
    /// Close a buffer by ID
    pub fn close_buffer(&mut self, id: usize) -> BufferManagerResult<()> {
        if !self.buffers.contains_key(&id) {
            return Err(BufferManagerError::BufferNotFound(id));
        }
        
        // Remove the buffer from the path map if it has a file path
        if let Some(path) = self.buffers[&id].file_path() {
            self.path_to_id.remove(&path.to_path_buf());
        }
        
        // Remove the buffer
        self.buffers.remove(&id);
        
        // Update the current buffer if necessary
        if self.current_id == Some(id) {
            self.current_id = self.buffers.keys().next().copied();
        }
        
        Ok(())
    }
    
    /// Get the number of buffers
    pub fn buffer_count(&self) -> usize {
        self.buffers.len()
    }
    
    /// Get a list of all buffer IDs
    pub fn buffer_ids(&self) -> Vec<usize> {
        self.buffers.keys().copied().collect()
    }
    
    /// Check if a buffer is modified
    pub fn is_buffer_modified(&self, id: usize) -> BufferManagerResult<bool> {
        Ok(self.get_buffer(id)?.is_modified())
    }
    
    /// Check if any buffer is modified
    pub fn any_buffer_modified(&self) -> bool {
        self.buffers.values().any(|b| b.is_modified())
    }
}

impl Default for BufferManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_create_buffer() {
        let mut manager = BufferManager::new();
        let id = manager.create_buffer().unwrap();
        assert_eq!(id, 1);
        assert_eq!(manager.buffer_count(), 1);
        assert_eq!(manager.current_buffer_id(), Some(1));
    }
    
    #[test]
    fn test_multiple_buffers() {
        let mut manager = BufferManager::new();
        let id1 = manager.create_buffer().unwrap();
        let id2 = manager.create_buffer().unwrap();
        
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(manager.buffer_count(), 2);
        assert_eq!(manager.current_buffer_id(), Some(1)); // First buffer created is current
        
        manager.set_current_buffer(id2).unwrap();
        assert_eq!(manager.current_buffer_id(), Some(2));
    }
    
    #[test]
    fn test_close_buffer() {
        let mut manager = BufferManager::new();
        let id1 = manager.create_buffer().unwrap();
        let id2 = manager.create_buffer().unwrap();
        
        manager.close_buffer(id1).unwrap();
        assert_eq!(manager.buffer_count(), 1);
        assert_eq!(manager.current_buffer_id(), Some(id2));
        
        assert!(manager.get_buffer(id1).is_err());
        assert!(manager.get_buffer(id2).is_ok());
    }
    
    #[test]
    fn test_open_file() {
        // Create a temporary file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Test content").unwrap();
        let file_path = temp_file.path();
        
        let mut manager = BufferManager::new();
        let id = manager.open_file(file_path).unwrap();
        
        assert_eq!(manager.buffer_count(), 1);
        assert_eq!(manager.current_buffer_id(), Some(id));
        
        let buffer = manager.get_buffer(id).unwrap();
        assert_eq!(buffer.content().trim(), "Test content");
        
        // Opening the same file again should return the same buffer
        let id2 = manager.open_file(file_path).unwrap();
        assert_eq!(id, id2);
        assert_eq!(manager.buffer_count(), 1);
    }
}