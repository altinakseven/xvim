//! Macro module - Recording and playback of command sequences
//!
//! This module implements the macro recording and playback functionality,
//! allowing users to record a sequence of commands and play them back.

use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
// // use std::collections::HashMap;
use crate::register::{RegisterManager, RegisterType, RegisterContent};

/// Macro recorder state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacroRecorderState {
    /// Not recording
    Idle,
    /// Recording to a register
    Recording(char),
}

/// Macro recorder
#[derive(Debug)]
pub struct MacroRecorder {
    /// Current state
    state: MacroRecorderState,
    /// Current recording
    current_recording: Vec<KeyEvent>,
    /// Register manager
    register_manager: RegisterManager,
}

impl MacroRecorder {
    /// Create a new macro recorder
    pub fn new(register_manager: RegisterManager) -> Self {
        Self {
            state: MacroRecorderState::Idle,
            current_recording: Vec::new(),
            register_manager,
        }
    }
    
    /// Start recording to a register
    pub fn start_recording(&mut self, register: char) -> bool {
        // Only allow recording to a-z registers
        if !('a'..='z').contains(&register) && !('A'..='Z').contains(&register) {
            return false;
        }
        
        // Clear any existing recording
        self.current_recording.clear();
        
        // Set the state to recording
        self.state = MacroRecorderState::Recording(register);
        
        true
    }
    
    /// Stop recording and save to the register
    pub fn stop_recording(&mut self) -> bool {
        if let MacroRecorderState::Recording(register) = self.state {
            // Convert the recording to a register content
            let content = RegisterContent::macro_recording(&self.current_recording);
            
            // Save to the register
            if let Some(register_type) = RegisterType::from_char(register) {
                self.register_manager.set_register(register_type, content);
            } else {
                // Fallback to unnamed register
                self.register_manager.set_register(RegisterType::Unnamed, content);
            }
            
            // Clear the recording
            self.current_recording.clear();
            
            // Set the state to idle
            self.state = MacroRecorderState::Idle;
            
            true
        } else {
            false
        }
    }
    
    /// Record a key event
    pub fn record_key(&mut self, key: KeyEvent) -> bool {
        if let MacroRecorderState::Recording(_) = self.state {
            // Don't record the q key that starts/stops recording
            if key.code == KeyCode::Char('q') && key.modifiers == KeyModifiers::NONE {
                return false;
            }
            
            // Add the key to the recording
            self.current_recording.push(key);
            
            true
        } else {
            false
        }
    }
    
    /// Get the current state
    pub fn state(&self) -> MacroRecorderState {
        self.state
    }
    
    /// Get the current recording
    pub fn current_recording(&self) -> &[KeyEvent] {
        &self.current_recording
    }
    
    /// Get a macro from a register
    pub fn get_macro(&self, register: char) -> Option<Vec<KeyEvent>> {
        if let Some(register_type) = RegisterType::from_char(register) {
            if let Some(content) = self.register_manager.get_register(register_type) {
                content.as_macro_recording().cloned()
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Play back a macro from a register
    pub fn play_macro(&self, register: char) -> Option<Vec<KeyEvent>> {
        self.get_macro(register)
    }
}

/// Macro player
#[derive(Debug)]
pub struct MacroPlayer {
    /// Macros being played
    playback_stack: Vec<(char, Vec<KeyEvent>, usize)>,
    /// Register manager
    register_manager: RegisterManager,
}

impl MacroPlayer {
    /// Create a new macro player
    pub fn new(register_manager: RegisterManager) -> Self {
        Self {
            playback_stack: Vec::new(),
            register_manager,
        }
    }
    
    /// Start playing a macro from a register
    pub fn start_playback(&mut self, register: char) -> bool {
        // Get the macro from the register
        if let Some(register_type) = RegisterType::from_char(register) {
            if let Some(content) = self.register_manager.get_register(register_type) {
                if let Some(macro_keys) = content.as_macro_recording() {
                    // Push the macro onto the playback stack
                    self.playback_stack.push((register, macro_keys.clone(), 0));
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Get the next key event from the current macro
    pub fn next_key(&mut self) -> Option<KeyEvent> {
        if let Some((register, keys, index)) = self.playback_stack.last_mut() {
            if *index < keys.len() {
                let key = keys[*index];
                *index += 1;
                
                // If we've reached the end of the macro, pop it from the stack
                if *index >= keys.len() {
                    self.playback_stack.pop();
                }
                
                Some(key)
            } else {
                // Pop the empty macro from the stack
                self.playback_stack.pop();
                None
            }
        } else {
            None
        }
    }
    
    /// Check if there are any macros being played
    pub fn is_playing(&self) -> bool {
        !self.playback_stack.is_empty()
    }
    
    /// Get the current playback stack
    pub fn playback_stack(&self) -> &[(char, Vec<KeyEvent>, usize)] {
        &self.playback_stack
    }
    
    /// Clear the playback stack
    pub fn clear(&mut self) {
        self.playback_stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_macro_recorder() {
        let register_manager = RegisterManager::new();
        let mut recorder = MacroRecorder::new(register_manager);
        
        // Start recording to register a
        assert!(recorder.start_recording('a'));
        assert_eq!(recorder.state(), MacroRecorderState::Recording('a'));
        
        // Record some keys
        let key1 = KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE);
        let key2 = KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE);
        let key3 = KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE);
        let key4 = KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE);
        let key5 = KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE);
        let key6 = KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE);
        let key7 = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        
        assert!(recorder.record_key(key1));
        assert!(recorder.record_key(key2));
        assert!(recorder.record_key(key3));
        assert!(recorder.record_key(key4));
        assert!(recorder.record_key(key5));
        assert!(recorder.record_key(key6));
        assert!(recorder.record_key(key7));
        
        // Check the recording
        assert_eq!(recorder.current_recording().len(), 7);
        
        // Stop recording
        assert!(recorder.stop_recording());
        assert_eq!(recorder.state(), MacroRecorderState::Idle);
        
        // Check that the recording was saved to the register
        let register_content = recorder.register_manager.get_register_by_char('a').unwrap();
        let macro_keys = register_content.as_macro_recording().unwrap();
        assert_eq!(macro_keys.len(), 7);
    }
    
    #[test]
    fn test_macro_player() {
        let mut register_manager = RegisterManager::new();
        
        // Create a macro recording
        let key1 = KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE);
        let key2 = KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE);
        let key3 = KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE);
        let key4 = KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE);
        let key5 = KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE);
        let key6 = KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE);
        let key7 = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        
        let keys = vec![key1, key2, key3, key4, key5, key6, key7];
        let content = RegisterContent::macro_recording(&keys);
        
        // Save it to register a
        register_manager.set_register_by_char('a', content);
        
        // Create a macro player
        let mut player = MacroPlayer::new(register_manager);
        
        // Start playback
        assert!(player.start_playback('a'));
        assert!(player.is_playing());
        
        // Get the keys one by one
        assert_eq!(player.next_key(), Some(key1));
        assert_eq!(player.next_key(), Some(key2));
        assert_eq!(player.next_key(), Some(key3));
        assert_eq!(player.next_key(), Some(key4));
        assert_eq!(player.next_key(), Some(key5));
        assert_eq!(player.next_key(), Some(key6));
        assert_eq!(player.next_key(), Some(key7));
        
        // Should be done now
        assert_eq!(player.next_key(), None);
        assert!(!player.is_playing());
    }
}