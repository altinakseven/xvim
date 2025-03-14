//! Register module - Storage for yanked and deleted text
//!
//! This module implements Vim-like registers for storing text that has been
//! yanked (copied) or deleted. It supports named registers, numbered registers,
//! and special registers like the unnamed register.

use std::collections::HashMap;
use crossterm::event::KeyEvent;

/// Register types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegisterType {
    /// Named registers (a-z, A-Z)
    Named(char),
    /// Numbered registers (0-9)
    Numbered(u8),
    /// Unnamed register (")
    Unnamed,
    /// Small delete register (-)
    SmallDelete,
    /// Black hole register (_)
    BlackHole,
    /// System clipboard register (*)
    Clipboard,
    /// Selection register (+)
    Selection,
    /// Last search pattern register (/)
    SearchPattern,
    /// Command history register (:)
    CommandHistory,
    /// File name register (%)
    FileName,
    /// Alternate file name register (#)
    AlternateFileName,
    /// Last inserted text register (.)
    LastInserted,
    /// Last command register (:)
    LastCommand,
}

impl RegisterType {
    /// Convert a character to a register type
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'a'..='z' | 'A'..='Z' => Some(RegisterType::Named(c)),
            '0'..='9' => Some(RegisterType::Numbered(c.to_digit(10).unwrap() as u8)),
            '"' => Some(RegisterType::Unnamed),
            '-' => Some(RegisterType::SmallDelete),
            '_' => Some(RegisterType::BlackHole),
            '*' => Some(RegisterType::Clipboard),
            '+' => Some(RegisterType::Selection),
            '/' => Some(RegisterType::SearchPattern),
            ':' => Some(RegisterType::CommandHistory),
            '%' => Some(RegisterType::FileName),
            '#' => Some(RegisterType::AlternateFileName),
            '.' => Some(RegisterType::LastInserted),
            _ => None,
        }
    }

    /// Convert a register type to a character
    pub fn to_char(&self) -> char {
        match self {
            RegisterType::Named(c) => *c,
            RegisterType::Numbered(n) => char::from_digit(*n as u32, 10).unwrap(),
            RegisterType::Unnamed => '"',
            RegisterType::SmallDelete => '-',
            RegisterType::BlackHole => '_',
            RegisterType::Clipboard => '*',
            RegisterType::Selection => '+',
            RegisterType::SearchPattern => '/',
            RegisterType::CommandHistory => ':',
            RegisterType::FileName => '%',
            RegisterType::AlternateFileName => '#',
            RegisterType::LastInserted => '.',
            RegisterType::LastCommand => ':',
        }
    }
}

/// Register content types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterContent {
    /// Character-wise content (normal text)
    CharacterWise(String),
    /// Line-wise content (whole lines)
    LineWise(Vec<String>),
    /// Block-wise content (rectangular block)
    BlockWise(Vec<String>),
    /// Macro recording (sequence of key events)
    MacroRecording(Vec<KeyEvent>),
}

impl RegisterContent {
    /// Create character-wise content
    pub fn character_wise(text: &str) -> Self {
        RegisterContent::CharacterWise(text.to_string())
    }

    /// Create line-wise content
    pub fn line_wise(lines: &[&str]) -> Self {
        RegisterContent::LineWise(lines.iter().map(|s| s.to_string()).collect())
    }

    /// Create block-wise content
    pub fn block_wise(lines: &[&str]) -> Self {
        RegisterContent::BlockWise(lines.iter().map(|s| s.to_string()).collect())
    }
    
    /// Create macro recording content
    pub fn macro_recording(keys: &[KeyEvent]) -> Self {
        RegisterContent::MacroRecording(keys.to_vec())
    }

    /// Get the content as a string
    pub fn as_string(&self) -> String {
        match self {
            RegisterContent::CharacterWise(text) => text.clone(),
            RegisterContent::LineWise(lines) => lines.join("\n"),
            RegisterContent::BlockWise(lines) => lines.join("\n"),
            RegisterContent::MacroRecording(_) => "[Macro Recording]".to_string(),
        }
    }

    /// Get the content as lines
    pub fn as_lines(&self) -> Vec<String> {
        match self {
            RegisterContent::CharacterWise(text) => text.split('\n').map(|s| s.to_string()).collect(),
            RegisterContent::LineWise(lines) => lines.clone(),
            RegisterContent::BlockWise(lines) => lines.clone(),
            RegisterContent::MacroRecording(_) => vec!["[Macro Recording]".to_string()],
        }
    }
    
    /// Get the content as a macro recording
    pub fn as_macro_recording(&self) -> Option<&Vec<KeyEvent>> {
        match self {
            RegisterContent::MacroRecording(keys) => Some(keys),
            _ => None,
        }
    }

    /// Check if the content is empty
    pub fn is_empty(&self) -> bool {
        match self {
            RegisterContent::CharacterWise(text) => text.is_empty(),
            RegisterContent::LineWise(lines) => lines.is_empty() || lines.iter().all(|s| s.is_empty()),
            RegisterContent::BlockWise(lines) => lines.is_empty() || lines.iter().all(|s| s.is_empty()),
            RegisterContent::MacroRecording(keys) => keys.is_empty(),
        }
    }
}

/// Register manager
#[derive(Debug, Clone)]
pub struct RegisterManager {
    /// Registers
    registers: HashMap<RegisterType, RegisterContent>,
    /// Default register
    default_register: RegisterType,
}

impl RegisterManager {
    /// Create a new register manager
    pub fn new() -> Self {
        let mut registers = HashMap::new();
        registers.insert(RegisterType::Unnamed, RegisterContent::CharacterWise(String::new()));
        
        Self {
            registers,
            default_register: RegisterType::Unnamed,
        }
    }

    /// Set the content of a register
    pub fn set_register(&mut self, register_type: RegisterType, content: RegisterContent) {
        // If setting a named register with an uppercase letter, append to the register
        if let RegisterType::Named(c) = register_type {
            if c.is_ascii_uppercase() {
                let lowercase_register = RegisterType::Named(c.to_ascii_lowercase());
                if let Some(existing_content) = self.registers.get(&lowercase_register) {
                    let new_content = match (existing_content, &content) {
                        (RegisterContent::CharacterWise(existing), RegisterContent::CharacterWise(new)) => {
                            RegisterContent::CharacterWise(format!("{}{}", existing, new))
                        },
                        (RegisterContent::LineWise(existing), RegisterContent::LineWise(new)) => {
                            let mut lines = existing.clone();
                            lines.extend(new.clone());
                            RegisterContent::LineWise(lines)
                        },
                        (RegisterContent::BlockWise(existing), RegisterContent::BlockWise(new)) => {
                            let mut lines = existing.clone();
                            lines.extend(new.clone());
                            RegisterContent::BlockWise(lines)
                        },
                        (RegisterContent::MacroRecording(existing), RegisterContent::MacroRecording(new)) => {
                            let mut keys = existing.clone();
                            keys.extend(new.clone());
                            RegisterContent::MacroRecording(keys)
                        },
                        _ => content.clone(),
                    };
                    self.registers.insert(lowercase_register, new_content);
                    return;
                }
            }
        }

        // For normal registers, just set the content
        self.registers.insert(register_type, content.clone());

        // If this is not the unnamed register or black hole register, also update the unnamed register
        if register_type != RegisterType::Unnamed && register_type != RegisterType::BlackHole {
            self.registers.insert(RegisterType::Unnamed, content.clone());
        }

        // If this is a named register, also update the corresponding numbered register
        if let RegisterType::Named(_) = register_type {
            // Shift numbered registers
            for i in (1..=9).rev() {
                if let Some(content) = self.registers.remove(&RegisterType::Numbered(i - 1)) {
                    self.registers.insert(RegisterType::Numbered(i), content);
                }
            }
            // Set register 0 with a clone of the content
            self.registers.insert(RegisterType::Numbered(0), content.clone());
        }
    }

    /// Get the content of a register
    pub fn get_register(&self, register_type: RegisterType) -> Option<&RegisterContent> {
        self.registers.get(&register_type)
    }

    /// Get the content of a register by character
    pub fn get_register_by_char(&self, c: char) -> Option<&RegisterContent> {
        RegisterType::from_char(c).and_then(|rt| self.get_register(rt))
    }

    /// Set the content of a register by character
    pub fn set_register_by_char(&mut self, c: char, content: RegisterContent) -> bool {
        if let Some(register_type) = RegisterType::from_char(c) {
            self.set_register(register_type, content);
            true
        } else {
            false
        }
    }

    /// Get the default register
    pub fn default_register(&self) -> RegisterType {
        self.default_register
    }

    /// Set the default register
    pub fn set_default_register(&mut self, register_type: RegisterType) {
        self.default_register = register_type;
    }

    /// Get the content of the default register
    pub fn get_default_register_content(&self) -> Option<&RegisterContent> {
        self.get_register(self.default_register)
    }

    /// Clear all registers
    pub fn clear(&mut self) {
        self.registers.clear();
        self.registers.insert(RegisterType::Unnamed, RegisterContent::CharacterWise(String::new()));
    }

    /// Get all registers
    pub fn registers(&self) -> &HashMap<RegisterType, RegisterContent> {
        &self.registers
    }
}

impl Default for RegisterManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyModifiers};

    #[test]
    fn test_register_type_conversion() {
        // Test named registers
        assert_eq!(RegisterType::from_char('a'), Some(RegisterType::Named('a')));
        assert_eq!(RegisterType::from_char('Z'), Some(RegisterType::Named('Z')));
        
        // Test numbered registers
        assert_eq!(RegisterType::from_char('0'), Some(RegisterType::Numbered(0)));
        assert_eq!(RegisterType::from_char('9'), Some(RegisterType::Numbered(9)));
        
        // Test special registers
        assert_eq!(RegisterType::from_char('"'), Some(RegisterType::Unnamed));
        assert_eq!(RegisterType::from_char('-'), Some(RegisterType::SmallDelete));
        assert_eq!(RegisterType::from_char('_'), Some(RegisterType::BlackHole));
        
        // Test invalid register
        assert_eq!(RegisterType::from_char('!'), None);
        
        // Test round-trip conversion
        let register_type = RegisterType::Named('a');
        assert_eq!(RegisterType::from_char(register_type.to_char()), Some(register_type));
    }
    
    #[test]
    fn test_register_content() {
        // Test character-wise content
        let content = RegisterContent::character_wise("Hello, world!");
        assert_eq!(content.as_string(), "Hello, world!");
        assert_eq!(content.as_lines(), vec!["Hello, world!"]);
        assert!(!content.is_empty());
        
        // Test line-wise content
        let content = RegisterContent::line_wise(&["Line 1", "Line 2", "Line 3"]);
        assert_eq!(content.as_string(), "Line 1\nLine 2\nLine 3");
        assert_eq!(content.as_lines(), vec!["Line 1", "Line 2", "Line 3"]);
        assert!(!content.is_empty());
        
        // Test block-wise content
        let content = RegisterContent::block_wise(&["Block 1", "Block 2", "Block 3"]);
        assert_eq!(content.as_string(), "Block 1\nBlock 2\nBlock 3");
        assert_eq!(content.as_lines(), vec!["Block 1", "Block 2", "Block 3"]);
        assert!(!content.is_empty());
        
        // Test macro recording content
        let keys = vec![
            KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
        ];
        let content = RegisterContent::macro_recording(&keys);
        assert_eq!(content.as_string(), "[Macro Recording]");
        assert_eq!(content.as_lines(), vec!["[Macro Recording]"]);
        assert_eq!(content.as_macro_recording(), Some(&keys));
        assert!(!content.is_empty());
        
        // Test empty content
        let content = RegisterContent::character_wise("");
        assert_eq!(content.as_string(), "");
        assert_eq!(content.as_lines(), vec![""]);
        assert!(content.is_empty());
    }
    
    #[test]
    fn test_register_manager() {
        let mut manager = RegisterManager::new();
        
        // Test setting and getting registers
        let content = RegisterContent::character_wise("Hello, world!");
        manager.set_register(RegisterType::Named('a'), content.clone());
        
        assert_eq!(manager.get_register(RegisterType::Named('a')), Some(&content));
        assert_eq!(manager.get_register(RegisterType::Unnamed), Some(&content));
        assert_eq!(manager.get_register(RegisterType::Numbered(0)), Some(&content));
        
        // Test setting and getting registers by character
        let content = RegisterContent::character_wise("Another test");
        assert!(manager.set_register_by_char('b', content.clone()));
        
        assert_eq!(manager.get_register_by_char('b'), Some(&content));
        assert_eq!(manager.get_register_by_char('"'), Some(&content));
        assert_eq!(manager.get_register_by_char('0'), Some(&content));
        
        // Test default register
        assert_eq!(manager.default_register(), RegisterType::Unnamed);
        assert_eq!(manager.get_default_register_content(), Some(&content));
        
        manager.set_default_register(RegisterType::Named('a'));
        assert_eq!(manager.default_register(), RegisterType::Named('a'));
        assert_eq!(manager.get_default_register_content(), Some(&RegisterContent::character_wise("Hello, world!")));
        
        // Test uppercase named registers (append)
        let content1 = RegisterContent::character_wise("First part");
        let content2 = RegisterContent::character_wise(" - Second part");
        
        manager.set_register(RegisterType::Named('c'), content1);
        manager.set_register(RegisterType::Named('C'), content2);
        
        assert_eq!(
            manager.get_register(RegisterType::Named('c')),
            Some(&RegisterContent::character_wise("First part - Second part"))
        );
    }
}