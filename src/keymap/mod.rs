//! Key mapping module
//!
//! This module implements the key mapping system for xvim, which allows users to
//! define custom key bindings for various commands and operations.

use std::collections::HashMap;
use crossterm::event::KeyEvent;
use crate::mode::Mode;

/// A key sequence is a series of key events that can be mapped to a command
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeySequence {
    /// The key events in the sequence
    keys: Vec<KeyEvent>,
}

impl KeySequence {
    /// Create a new key sequence from a vector of key events
    pub fn new(keys: Vec<KeyEvent>) -> Self {
        Self { keys }
    }

    /// Create a key sequence from a single key event
    pub fn from_key(key: KeyEvent) -> Self {
        Self { keys: vec![key] }
    }

    /// Get the key events in the sequence
    pub fn keys(&self) -> &[KeyEvent] {
        &self.keys
    }

    /// Check if this sequence is a prefix of another sequence
    pub fn is_prefix_of(&self, other: &KeySequence) -> bool {
        if self.keys.len() > other.keys.len() {
            return false;
        }

        for (i, key) in self.keys.iter().enumerate() {
            if key != &other.keys[i] {
                return false;
            }
        }

        true
    }

    /// Add a key to the sequence
    pub fn push(&mut self, key: KeyEvent) {
        self.keys.push(key);
    }

    /// Get the length of the sequence
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Check if the sequence is empty
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}

/// A command that can be executed by the editor
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// A built-in command
    BuiltIn(String),
    /// A custom command (e.g., from a plugin)
    Custom(String),
    /// A sequence of keys to execute
    Keys(KeySequence),
}

/// A key mapping maps a key sequence to a command in a specific mode
#[derive(Debug, Clone)]
pub struct KeyMapping {
    /// The mode in which this mapping applies
    mode: Mode,
    /// The key sequence to match
    sequence: KeySequence,
    /// The command to execute
    command: Command,
    /// Whether this mapping is recursive
    recursive: bool,
}

impl KeyMapping {
    /// Create a new key mapping
    pub fn new(mode: Mode, sequence: KeySequence, command: Command, recursive: bool) -> Self {
        Self {
            mode,
            sequence,
            command,
            recursive,
        }
    }

    /// Get the mode for this mapping
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Get the key sequence for this mapping
    pub fn sequence(&self) -> &KeySequence {
        &self.sequence
    }

    /// Get the command for this mapping
    pub fn command(&self) -> &Command {
        &self.command
    }

    /// Check if this mapping is recursive
    pub fn is_recursive(&self) -> bool {
        self.recursive
    }
}

/// The key map stores all key mappings and provides methods to look them up
#[derive(Debug, Clone, Default)]
pub struct KeyMap {
    /// Mappings for each mode
    mappings: HashMap<Mode, Vec<KeyMapping>>,
}

impl KeyMap {
    /// Create a new key map
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }

    /// Add a key mapping
    pub fn add_mapping(&mut self, mapping: KeyMapping) {
        let mode = mapping.mode();
        let mappings = self.mappings.entry(mode).or_insert_with(Vec::new);
        mappings.push(mapping);
    }

    /// Find a mapping that matches the given key sequence in the specified mode
    pub fn find_mapping(&self, mode: Mode, sequence: &KeySequence) -> Option<&KeyMapping> {
        if let Some(mappings) = self.mappings.get(&mode) {
            for mapping in mappings {
                if mapping.sequence() == sequence {
                    return Some(mapping);
                }
            }
        }
        None
    }

    /// Find all mappings that have the given sequence as a prefix
    pub fn find_prefix_mappings(&self, mode: Mode, sequence: &KeySequence) -> Vec<&KeyMapping> {
        let mut result = Vec::new();
        if let Some(mappings) = self.mappings.get(&mode) {
            for mapping in mappings {
                if sequence.is_prefix_of(mapping.sequence()) {
                    result.push(mapping);
                }
            }
        }
        result
    }

    /// Remove a mapping
    pub fn remove_mapping(&mut self, mode: Mode, sequence: &KeySequence) -> bool {
        if let Some(mappings) = self.mappings.get_mut(&mode) {
            let len = mappings.len();
            mappings.retain(|m| m.sequence() != sequence);
            return mappings.len() < len;
        }
        false
    }

    /// Clear all mappings
    pub fn clear(&mut self) {
        self.mappings.clear();
    }

    /// Clear all mappings for a specific mode
    pub fn clear_mode(&mut self, mode: Mode) {
        self.mappings.remove(&mode);
    }
}

/// The key handler processes key events and executes commands
#[derive(Debug)]
pub struct KeyHandler {
    /// The key map
    key_map: KeyMap,
    /// The current key sequence being built
    current_sequence: KeySequence,
    /// The timeout for key sequences in milliseconds
    timeout: u64,
    /// The time of the last key press
    last_key_time: std::time::Instant,
}

impl KeyHandler {
    /// Create a new key handler
    pub fn new() -> Self {
        Self {
            key_map: KeyMap::new(),
            current_sequence: KeySequence::new(Vec::new()),
            timeout: 1000, // 1 second timeout by default
            last_key_time: std::time::Instant::now(),
        }
    }

    /// Set the timeout for key sequences
    pub fn set_timeout(&mut self, timeout: u64) {
        self.timeout = timeout;
    }

    /// Get the key map
    pub fn key_map(&self) -> &KeyMap {
        &self.key_map
    }

    /// Get a mutable reference to the key map
    pub fn key_map_mut(&mut self) -> &mut KeyMap {
        &mut self.key_map
    }

    /// Process a key event in the given mode
    pub fn process_key(&mut self, key: KeyEvent, mode: Mode) -> Option<Command> {
        // Check if the timeout has expired
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_key_time).as_millis() as u64;
        if elapsed > self.timeout && !self.current_sequence.is_empty() {
            // Timeout expired, reset the sequence
            self.current_sequence = KeySequence::new(Vec::new());
        }
        self.last_key_time = now;

        // Add the key to the current sequence
        self.current_sequence.push(key);

        // Check if there's a mapping for the current sequence
        if let Some(mapping) = self.key_map.find_mapping(mode, &self.current_sequence) {
            // Found a mapping, reset the sequence and return the command
            let command = mapping.command().clone();
            self.current_sequence = KeySequence::new(Vec::new());
            return Some(command);
        }

        // Check if there are any mappings that have the current sequence as a prefix
        let prefix_mappings = self.key_map.find_prefix_mappings(mode, &self.current_sequence);
        if !prefix_mappings.is_empty() {
            // There are mappings that have the current sequence as a prefix,
            // so we need to wait for more keys
            return None;
        }

        // No mappings found, reset the sequence
        self.current_sequence = KeySequence::new(Vec::new());

        // Return a default command for the key
        self.default_command(key, mode)
    }

    /// Get the default command for a key in the given mode
    fn default_command(&self, _key: KeyEvent, _mode: Mode) -> Option<Command> {
        // This is where we would implement the default Vim key bindings
        // For now, just return None
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyModifiers};

    #[test]
    fn test_key_sequence() {
        let key1 = KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE);
        let key2 = KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE);
        let key3 = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE);

        let mut seq1 = KeySequence::new(vec![key1, key2]);
        let seq2 = KeySequence::new(vec![key1, key2, key3]);

        assert_eq!(seq1.len(), 2);
        assert!(!seq1.is_empty());
        assert!(seq1.is_prefix_of(&seq2));
        assert!(!seq2.is_prefix_of(&seq1));

        seq1.push(key3);
        assert_eq!(seq1.len(), 3);
        assert_eq!(seq1, seq2);
    }

    #[test]
    fn test_key_map() {
        let key1 = KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE);
        let key2 = KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE);
        let key3 = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE);

        let seq1 = KeySequence::new(vec![key1, key2]);
        let seq2 = KeySequence::new(vec![key1, key2, key3]);

        let cmd1 = Command::BuiltIn("first_line".to_string());
        let cmd2 = Command::BuiltIn("go_down".to_string());

        let mapping1 = KeyMapping::new(Mode::Normal, seq1.clone(), cmd1.clone(), false);
        let mapping2 = KeyMapping::new(Mode::Normal, seq2.clone(), cmd2.clone(), false);

        let mut key_map = KeyMap::new();
        key_map.add_mapping(mapping1);
        key_map.add_mapping(mapping2);

        let found1 = key_map.find_mapping(Mode::Normal, &seq1);
        assert!(found1.is_some());
        assert_eq!(found1.unwrap().command(), &cmd1);

        let found2 = key_map.find_mapping(Mode::Normal, &seq2);
        assert!(found2.is_some());
        assert_eq!(found2.unwrap().command(), &cmd2);

        let prefix_mappings = key_map.find_prefix_mappings(Mode::Normal, &KeySequence::new(vec![key1]));
        assert_eq!(prefix_mappings.len(), 2);

        assert!(key_map.remove_mapping(Mode::Normal, &seq1));
        assert!(key_map.find_mapping(Mode::Normal, &seq1).is_none());
        assert!(key_map.find_mapping(Mode::Normal, &seq2).is_some());

        key_map.clear_mode(Mode::Normal);
        assert!(key_map.find_mapping(Mode::Normal, &seq2).is_none());
    }

    #[test]
    fn test_key_handler() {
        let key1 = KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE);
        let key2 = KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE);
        let key3 = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE);

        let seq1 = KeySequence::new(vec![key1, key2]);
        let seq2 = KeySequence::new(vec![key1, key2, key3]);

        let cmd1 = Command::BuiltIn("first_line".to_string());
        let cmd2 = Command::BuiltIn("go_down".to_string());

        let mapping1 = KeyMapping::new(Mode::Normal, seq1, cmd1.clone(), false);
        let mapping2 = KeyMapping::new(Mode::Normal, seq2, cmd2.clone(), false);

        let mut handler = KeyHandler::new();
        handler.key_map_mut().add_mapping(mapping1);
        handler.key_map_mut().add_mapping(mapping2);

        // First key in the sequence
        let result1 = handler.process_key(key1, Mode::Normal);
        assert!(result1.is_none()); // No command yet, waiting for more keys
// Second key in the sequence
let result2 = handler.process_key(key2, Mode::Normal);
assert_eq!(result2, Some(cmd1.clone())); // Found a mapping for 'gg'

// First key in the sequence again
let result3 = handler.process_key(key1, Mode::Normal);
assert!(result3.is_none()); // No command yet, waiting for more keys

// Second key in the sequence
let result4 = handler.process_key(key2, Mode::Normal);
assert_eq!(result4, Some(cmd1.clone())); // Found a mapping for 'gg'

// First key in the sequence again
let result5 = handler.process_key(key1, Mode::Normal);
assert!(result5.is_none()); // No command yet, waiting for more keys

// Second key in the sequence
let result6 = handler.process_key(key2, Mode::Normal);
assert_eq!(result6, Some(cmd1.clone())); // Found a mapping for 'gg'
        assert_eq!(result6, Some(cmd1)); // Found a mapping for 'gg'

        // Third key in a new sequence
        let result7 = handler.process_key(key3, Mode::Normal);
        assert!(result7.is_none()); // No mapping for just 'j'
    }
}