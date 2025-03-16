//! Window management for the terminal UI
//!
//! This module provides functionality for managing multiple windows in the terminal UI.
//! Windows can be split horizontally or vertically, resized, and navigated between.

use std::cmp::{max, min};

use crate::buffer::Buffer;
use crate::cursor::CursorPosition;

/// Window split direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    /// Horizontal split (windows stacked vertically)
    Horizontal,
    /// Vertical split (windows side by side)
    Vertical,
}

/// Window position and size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowRect {
    /// X position (column)
    pub x: u16,
    /// Y position (row)
    pub y: u16,
    /// Width in columns
    pub width: u16,
    /// Height in rows
    pub height: u16,
}

impl WindowRect {
    /// Create a new window rectangle
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Split the window in the given direction
    pub fn split(&self, direction: SplitDirection, ratio: f32) -> (Self, Self) {
        let ratio = ratio.max(0.1).min(0.9); // Ensure ratio is between 0.1 and 0.9

        match direction {
            SplitDirection::Horizontal => {
                // Calculate the height of each window
                let top_height = max(1, (self.height as f32 * ratio) as u16);
                let bottom_height = max(1, self.height - top_height);

                // Create the two windows
                let top = Self::new(self.x, self.y, self.width, top_height);
                let bottom = Self::new(self.x, self.y + top_height, self.width, bottom_height);

                (top, bottom)
            }
            SplitDirection::Vertical => {
                // Calculate the width of each window
                let left_width = max(1, (self.width as f32 * ratio) as u16);
                let right_width = max(1, self.width - left_width);

                // Create the two windows
                let left = Self::new(self.x, self.y, left_width, self.height);
                let right = Self::new(self.x + left_width, self.y, right_width, self.height);

                (left, right)
            }
        }
    }

    /// Check if the window is large enough to be split
    pub fn can_split(&self, direction: SplitDirection) -> bool {
        match direction {
            SplitDirection::Horizontal => self.height >= 4, // Need at least 4 rows for a meaningful split
            SplitDirection::Vertical => self.width >= 10,   // Need at least 10 columns for a meaningful split
        }
    }

    /// Get the usable area for buffer content (excluding borders and status line)
    pub fn content_area(&self) -> Self {
        // Reserve 1 row for status line at the bottom
        Self::new(self.x, self.y, self.width, self.height.saturating_sub(1))
    }
}

/// A window displaying a buffer
#[derive(Debug, Clone)]
pub struct Window {
    /// Window ID
    pub id: usize,
    /// Buffer ID
    pub buffer_id: usize,
    /// Window position and size
    pub rect: WindowRect,
    /// Cursor position within the buffer
    pub cursor: CursorPosition,
    /// Top line of the visible portion of the buffer
    pub top_line: usize,
    /// Left column of the visible portion of the buffer
    pub left_col: usize,
}

impl Window {
    /// Create a new window
    pub fn new(id: usize, buffer_id: usize, rect: WindowRect) -> Self {
        Self {
            id,
            buffer_id,
            rect,
            cursor: CursorPosition::new(0, 0),
            top_line: 0,
            left_col: 0,
        }
    }

    /// Get the visible height (number of lines that can be displayed)
    pub fn visible_height(&self) -> usize {
        self.rect.content_area().height as usize
    }

    /// Get the visible width (number of columns that can be displayed)
    pub fn visible_width(&self) -> usize {
        self.rect.content_area().width as usize
    }

    /// Ensure the cursor is visible by scrolling if necessary
    pub fn ensure_cursor_visible(&mut self, buffer: &Buffer) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure cursor is within buffer bounds
        let line_count = buffer.line_count();
        if line_count == 0 {
            self.cursor = CursorPosition::new(0, 0);
            return Ok(());
        }

        let cursor_line = min(self.cursor.line, line_count - 1);
        let line_length = buffer.line_length(cursor_line)?;
        let cursor_col = min(self.cursor.column, line_length);

        self.cursor = CursorPosition::new(cursor_line, cursor_col);

        // Scroll vertically if necessary
        if self.cursor.line < self.top_line {
            self.top_line = self.cursor.line;
        } else if self.cursor.line >= self.top_line + self.visible_height() {
            self.top_line = self.cursor.line - self.visible_height() + 1;
        }

        // Scroll horizontally if necessary
        if self.cursor.column < self.left_col {
            self.left_col = self.cursor.column;
        } else if self.cursor.column >= self.left_col + self.visible_width() {
            self.left_col = self.cursor.column - self.visible_width() + 1;
        }

        Ok(())
    }

    /// Move the cursor to the specified position
    pub fn move_cursor(&mut self, position: CursorPosition, buffer: &Buffer) -> Result<(), Box<dyn std::error::Error>> {
        self.cursor = position;
        self.ensure_cursor_visible(buffer)
    }

    /// Move the cursor up by the specified number of lines
    pub fn move_cursor_up(&mut self, count: usize, buffer: &Buffer) -> Result<(), Box<dyn std::error::Error>> {
        let new_line = self.cursor.line.saturating_sub(count);
        let new_position = CursorPosition::new(new_line, self.cursor.column);
        self.move_cursor(new_position, buffer)
    }

    /// Move the cursor down by the specified number of lines
    pub fn move_cursor_down(&mut self, count: usize, buffer: &Buffer) -> Result<(), Box<dyn std::error::Error>> {
        let new_line = min(self.cursor.line + count, buffer.line_count().saturating_sub(1));
        let new_position = CursorPosition::new(new_line, self.cursor.column);
        self.move_cursor(new_position, buffer)
    }

    /// Move the cursor left by the specified number of columns
    pub fn move_cursor_left(&mut self, count: usize, buffer: &Buffer) -> Result<(), Box<dyn std::error::Error>> {
        let new_col = self.cursor.column.saturating_sub(count);
        let new_position = CursorPosition::new(self.cursor.line, new_col);
        self.move_cursor(new_position, buffer)
    }

    /// Move the cursor right by the specified number of columns
    pub fn move_cursor_right(&mut self, count: usize, buffer: &Buffer) -> Result<(), Box<dyn std::error::Error>> {
        let line_length = buffer.line_length(self.cursor.line)?;
        let new_col = min(self.cursor.column + count, line_length);
        let new_position = CursorPosition::new(self.cursor.line, new_col);
        self.move_cursor(new_position, buffer)
    }

    /// Resize the window
    pub fn resize(&mut self, rect: WindowRect, buffer: &Buffer) -> Result<(), Box<dyn std::error::Error>> {
        self.rect = rect;
        self.ensure_cursor_visible(buffer)
    }
}

/// Window manager
#[derive(Debug, Clone)]
pub struct WindowManager {
    /// Windows
    windows: Vec<Window>,
    /// Current window ID
    current_window_id: usize,
    /// Next window ID
    next_window_id: usize,
}

impl WindowManager {
    /// Create a new window manager
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            current_window_id: 0,
            next_window_id: 1,
        }
    }

    /// Create the initial window
    pub fn create_initial_window(&mut self, buffer_id: usize, width: u16, height: u16) -> usize {
        // Start at y=1 to leave room for the tab bar at y=0
        let rect = WindowRect::new(0, 1, width, height - 1);
        let window = Window::new(self.next_window_id, buffer_id, rect);
        self.next_window_id += 1;
        let id = window.id;
        self.windows.push(window);
        self.current_window_id = id;
        id
    }

    /// Split the current window
    pub fn split_current_window(&mut self, direction: SplitDirection, buffer_id: usize) -> Option<usize> {
        // Find the current window
        let current_window_idx = self.windows.iter().position(|w| w.id == self.current_window_id)?;
        let current_window = &self.windows[current_window_idx];

        // Check if the window can be split
        if !current_window.rect.can_split(direction) {
            return None;
        }

        // Split the window
        let (rect1, rect2) = current_window.rect.split(direction, 0.5);

        // Update the current window
        self.windows[current_window_idx].rect = rect1;

        // Create a new window
        let new_window = Window::new(self.next_window_id, buffer_id, rect2);
        self.next_window_id += 1;
        let new_id = new_window.id;
        self.windows.push(new_window);

        // Make the new window the current window
        self.current_window_id = new_id;

        Some(new_id)
    }

    /// Get a reference to a window by ID
    pub fn get_window(&self, id: usize) -> Option<&Window> {
        self.windows.iter().find(|w| w.id == id)
    }

    /// Get a mutable reference to a window by ID
    pub fn get_window_mut(&mut self, id: usize) -> Option<&mut Window> {
        self.windows.iter_mut().find(|w| w.id == id)
    }

    /// Get a reference to the current window
    pub fn current_window(&self) -> Option<&Window> {
        self.get_window(self.current_window_id)
    }

    /// Get a mutable reference to the current window
    pub fn current_window_mut(&mut self) -> Option<&mut Window> {
        self.get_window_mut(self.current_window_id)
    }

    /// Set the current window
    pub fn set_current_window(&mut self, id: usize) -> bool {
        if self.get_window(id).is_some() {
            self.current_window_id = id;
            true
        } else {
            false
        }
    }

    /// Get the current window ID
    pub fn current_window_id(&self) -> usize {
        self.current_window_id
    }

    /// Get all windows
    pub fn windows(&self) -> &[Window] {
        &self.windows
    }

    /// Get all windows as mutable
    pub fn windows_mut(&mut self) -> &mut [Window] {
        &mut self.windows
    }

    /// Close a window by ID
    pub fn close_window(&mut self, id: usize) -> bool {
        let idx = match self.windows.iter().position(|w| w.id == id) {
            Some(idx) => idx,
            None => return false,
        };

        // Remove the window
        self.windows.remove(idx);

        // If we closed the current window, select another one
        if id == self.current_window_id && !self.windows.is_empty() {
            self.current_window_id = self.windows[0].id;
        }

        true
    }

    /// Navigate to the next window
    pub fn next_window(&mut self) -> bool {
        if self.windows.len() <= 1 {
            return false;
        }

        let current_idx = match self.windows.iter().position(|w| w.id == self.current_window_id) {
            Some(idx) => idx,
            None => return false,
        };

        let next_idx = (current_idx + 1) % self.windows.len();
        self.current_window_id = self.windows[next_idx].id;

        true
    }

    /// Navigate to the previous window
    pub fn prev_window(&mut self) -> bool {
        if self.windows.len() <= 1 {
            return false;
        }

        let current_idx = match self.windows.iter().position(|w| w.id == self.current_window_id) {
            Some(idx) => idx,
            None => return false,
        };

        let prev_idx = if current_idx == 0 {
            self.windows.len() - 1
        } else {
            current_idx - 1
        };

        self.current_window_id = self.windows[prev_idx].id;

        true
    }

    /// Resize all windows to fit the terminal size
    pub fn resize_all(&mut self, width: u16, height: u16, buffers: &[&Buffer]) -> Result<(), Box<dyn std::error::Error>> {
        if self.windows.is_empty() {
            return Ok(());
        }

        // For now, just resize the first window to fill the screen
        // In a more advanced implementation, we would maintain the window layout
        // Start at y=1 to leave room for the tab bar at y=0
        self.windows[0].rect = WindowRect::new(0, 1, width, height - 1);

        // Ensure cursor visibility for all windows
        for (i, window) in self.windows.iter_mut().enumerate() {
            if i < buffers.len() {
                window.ensure_cursor_visible(buffers[i])?;
            }
        }

        Ok(())
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_rect_split_horizontal() {
        let rect = WindowRect::new(0, 0, 80, 24);
        let (top, bottom) = rect.split(SplitDirection::Horizontal, 0.5);

        assert_eq!(top.x, 0);
        assert_eq!(top.y, 0);
        assert_eq!(top.width, 80);
        assert_eq!(top.height, 12);

        assert_eq!(bottom.x, 0);
        assert_eq!(bottom.y, 12);
        assert_eq!(bottom.width, 80);
        assert_eq!(bottom.height, 12);
    }

    #[test]
    fn test_window_rect_split_vertical() {
        let rect = WindowRect::new(0, 0, 80, 24);
        let (left, right) = rect.split(SplitDirection::Vertical, 0.5);

        assert_eq!(left.x, 0);
        assert_eq!(left.y, 0);
        assert_eq!(left.width, 40);
        assert_eq!(left.height, 24);

        assert_eq!(right.x, 40);
        assert_eq!(right.y, 0);
        assert_eq!(right.width, 40);
        assert_eq!(right.height, 24);
    }

    #[test]
    fn test_window_rect_can_split() {
        let large_rect = WindowRect::new(0, 0, 80, 24);
        assert!(large_rect.can_split(SplitDirection::Horizontal));
        assert!(large_rect.can_split(SplitDirection::Vertical));

        let small_rect = WindowRect::new(0, 0, 5, 3);
        assert!(!small_rect.can_split(SplitDirection::Horizontal));
        assert!(!small_rect.can_split(SplitDirection::Vertical));
    }

    #[test]
    fn test_window_manager_create_initial_window() {
        let mut manager = WindowManager::new();
        let id = manager.create_initial_window(1, 80, 24);

        assert_eq!(id, 1);
        assert_eq!(manager.current_window_id(), 1);
        assert_eq!(manager.windows().len(), 1);

        let window = manager.current_window().unwrap();
        assert_eq!(window.buffer_id, 1);
        assert_eq!(window.rect.width, 80);
        assert_eq!(window.rect.height, 23);
    }

    #[test]
    fn test_window_manager_split_window() {
        let mut manager = WindowManager::new();
        manager.create_initial_window(1, 80, 24);

        // Split horizontally
        let new_id = manager.split_current_window(SplitDirection::Horizontal, 2).unwrap();
        assert_eq!(manager.windows().len(), 2);
        assert_eq!(manager.current_window_id(), new_id);

        // Split vertically
        let new_id2 = manager.split_current_window(SplitDirection::Vertical, 3).unwrap();
        assert_eq!(manager.windows().len(), 3);
        assert_eq!(manager.current_window_id(), new_id2);
    }

    #[test]
    fn test_window_manager_navigation() {
        let mut manager = WindowManager::new();
        let id1 = manager.create_initial_window(1, 80, 24);
        let id2 = manager.split_current_window(SplitDirection::Horizontal, 2).unwrap();

        // Navigate to next window
        assert!(manager.next_window());
        assert_eq!(manager.current_window_id(), id1);

        // Navigate to previous window
        assert!(manager.prev_window());
        assert_eq!(manager.current_window_id(), id2);
    }
}