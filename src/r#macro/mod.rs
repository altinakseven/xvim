//! Macro recording and playback module
//!
//! This module provides functionality for recording and playing back macros.
//! Macros are sequences of key presses that can be recorded and played back
//! to automate repetitive tasks.

use std::collections::VecDeque;
// use crossterm::event::KeyEvent;
use crate::register::{RegisterManager, RegisterType, RegisterContent};

/// Macro recorder state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacroRecorderState {
    /// Not recording
    NotRecording,
    /// Recording to a register
    Recording(char),
}

/// Macro recorder
pub struct MacroRecorder {
    /// Current state
    state: MacroRecorderState,
    /// Recorded keys
    keys: Vec<KeyEvent>,
    /// Register manager
    register_manager: RegisterManager,
}

impl MacroRecorder {
    /// Create a new macro recorder
    pub fn new(register_manager: RegisterManager) -> Self {
        Self {
            state: MacroRecorderState::NotRecording,
            keys: Vec::new(),
            register_manager,
        }
    }
    
    /// Start recording a macro to a register
    pub fn start_recording(&mut self, register: char) -> bool {
        // Check if we're already recording
        if let MacroRecorderState::Recording(_) = self.state {
            return false;
        }
        
        // Start recording
        self.state = MacroRecorderState::Recording(register);
        self.keys.clear();
        
        true
    }
    
    /// Stop recording a macro
    pub fn stop_recording(&mut self) -> bool {
        // Check if we're recording
        if let MacroRecorderState::Recording(register) = self.state {
            // Convert the keys to a string representation
            let keys_str = self.keys_to_string();
            
            // Store the macro in the register
            if let Some(register_type) = RegisterType::from_char(register) {
                let content = RegisterContent::macro_content(&keys_str);
                self.register_manager.set_register(register_type, content);
            }
            
            // Reset state
            self.state = MacroRecorderState::NotRecording;
            self.keys.clear();
            
            true
        } else {
            false
        }
    }
    
    /// Record a key
    pub fn record_key(&mut self, key: KeyEvent) {
        // Check if we're recording
        if let MacroRecorderState::Recording(_) = self.state {
            // Add the key to the recorded keys
            self.keys.push(key);
        }
    }
    
    /// Get the current state
    pub fn state(&self) -> MacroRecorderState {
        self.state
    }
    
    /// Convert the recorded keys to a string representation
    fn keys_to_string(&self) -> String {
        // This is a simple implementation that just stores the key codes
        // A more sophisticated implementation would encode the keys in a way
        // that can be parsed back into KeyEvent objects
        let mut result = String::new();
        
        for key in &self.keys {
            // Encode the key as a string
            let key_str = format!("{:?}:{:?}", key.code, key.modifiers);
            
            // Add to the result
            if !result.is_empty() {
                result.push(',');
            }
            result.push_str(&key_str);
        }
        
        result
    }
}

/// Macro player
pub struct MacroPlayer {
    /// Keys to play back
    keys: VecDeque<KeyEvent>,
    /// Register manager
    register_manager: RegisterManager,
}

impl MacroPlayer {
    /// Create a new macro player
    pub fn new(register_manager: RegisterManager) -> Self {
        Self {
            keys: VecDeque::new(),
            register_manager,
        }
    }
    
    /// Start playing back a macro from a register
    pub fn start_playback(&mut self, register: char) -> bool {
        // Check if we're already playing back
        if !self.keys.is_empty() {
            return false;
        }
        
        // Get the macro from the register
        if let Some(register_type) = RegisterType::from_char(register) {
            if let Some(content) = self.register_manager.get_register(register_type) {
                // Parse the keys from the register content
                if let Some(keys) = self.parse_keys(content.as_string()) {
                    // Set the keys to play back
                    self.keys = keys;
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Get the next key to play back
    pub fn next_key(&mut self) -> Option<KeyEvent> {
        self.keys.pop_front()
    }
    
    /// Check if we're currently playing back a macro
    pub fn is_playing(&self) -> bool {
        !self.keys.is_empty()
    }
    
    /// Parse keys from a string representation
    fn parse_keys(&self, keys_str: &str) -> Option<VecDeque<KeyEvent>> {
        // This is a simple implementation that parses the key codes
        // A more sophisticated implementation would decode the keys from
        // a more robust encoding
        
        // Split the string by commas
        let key_strs: Vec<&str> = keys_str.split(',').collect();
        
        // Parse each key
        let mut keys = VecDeque::new();
        
        for key_str in key_strs {
            // Split by colon to get code and modifiers
            let parts: Vec<&str> = key_str.split(':').collect();
            
            if parts.len() == 2 {
                // Parse the key code and modifiers
                // This is a simplified implementation
                // In a real implementation, we would parse the KeyCode and KeyModifiers
                // from their string representations
                
                // For now, just create a dummy key event
                // This is just a placeholder
                use crossterm::event::{KeyCode, KeyModifiers};
                let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE);
                
                keys.push_back(key);
            }
        }
        
        if keys.is_empty() {
            None
        } else {
            Some(keys)
        }
    }
}