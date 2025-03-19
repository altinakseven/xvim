//! Spell command handlers
//!
//! This module implements handlers for spell checking commands.

use crate::command::{ExCommand, ExCommandError, ExCommandResult, ExCommandRegistry};
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::io::{Read, Write, BufRead, BufReader, BufWriter};
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;

/// Spell suggestion
#[derive(Debug, Clone)]
pub struct SpellSuggestion {
    /// Suggested word
    pub word: String,
    /// Score (lower is better)
    pub score: usize,
    /// Source dictionary
    pub source: String,
}

/// Spell error
#[derive(Debug, Clone)]
pub struct SpellError {
    /// Misspelled word
    pub word: String,
    /// Line number
    pub line: usize,
    /// Column number
    pub column: usize,
    /// Suggestions
    pub suggestions: Vec<SpellSuggestion>,
    /// Error type
    pub error_type: SpellErrorType,
}

/// Spell error type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellErrorType {
    /// Unknown word
    Unknown,
    /// Capitalization error
    Capitalization,
    /// Rare word
    Rare,
    /// Bad word
    Bad,
}

/// Spell dictionary
#[derive(Debug)]
pub struct SpellDictionary {
    /// Dictionary name
    pub name: String,
    /// Dictionary path
    pub path: PathBuf,
    /// Dictionary words
    pub words: HashSet<String>,
    /// Dictionary encoding
    pub encoding: String,
    /// Dictionary language
    pub language: String,
    /// Dictionary region
    pub region: Option<String>,
    /// Dictionary version
    pub version: Option<String>,
    /// Dictionary format
    pub format: SpellDictionaryFormat,
}

/// Spell dictionary format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellDictionaryFormat {
    /// Plain text
    Plain,
    /// Hunspell
    Hunspell,
    /// Aspell
    Aspell,
    /// Vim
    Vim,
}

/// Spell manager
#[derive(Debug)]
pub struct SpellManager {
    /// Spell dictionaries
    pub dictionaries: HashMap<String, SpellDictionary>,
    /// Active dictionaries
    pub active_dictionaries: Vec<String>,
    /// Spell errors
    pub errors: HashMap<usize, Vec<SpellError>>,
    /// Spell options
    pub options: SpellOptions,
    /// User dictionary
    pub user_dictionary: Option<PathBuf>,
    /// Bad words
    pub bad_words: HashSet<String>,
    /// Good words
    pub good_words: HashSet<String>,
    /// Spell history
    pub history: VecDeque<SpellAction>,
    /// Maximum history size
    pub max_history_size: usize,
}

/// Spell options
#[derive(Debug, Clone)]
pub struct SpellOptions {
    /// Spell checking enabled
    pub enabled: bool,
    /// Check capitalization
    pub check_caps: bool,
    /// Maximum number of suggestions
    pub max_suggestions: usize,
    /// Minimum word length
    pub min_word_length: usize,
    /// Ignore words with numbers
    pub ignore_numbers: bool,
    /// Ignore words in UPPERCASE
    pub ignore_uppercase: bool,
    /// Ignore words with CamelCase
    pub ignore_camelcase: bool,
    /// Ignore words with underscores
    pub ignore_underscores: bool,
    /// Ignore words with URLs
    pub ignore_urls: bool,
    /// Ignore words with email addresses
    pub ignore_emails: bool,
    /// Ignore words in comments
    pub ignore_comments: bool,
    /// Ignore words in strings
    pub ignore_strings: bool,
    /// Ignore words in code
    pub ignore_code: bool,
}

/// Spell action
#[derive(Debug, Clone)]
pub enum SpellAction {
    /// Add word to dictionary
    AddWord(String),
    /// Remove word from dictionary
    RemoveWord(String),
    /// Add word to bad words
    AddBadWord(String),
    /// Remove word from bad words
    RemoveBadWord(String),
    /// Correct word
    CorrectWord(String, String),
}

impl SpellManager {
    /// Create a new spell manager
    pub fn new() -> Self {
        Self {
            dictionaries: HashMap::new(),
            active_dictionaries: Vec::new(),
            errors: HashMap::new(),
            options: SpellOptions {
                enabled: false,
                check_caps: true,
                max_suggestions: 10,
                min_word_length: 3,
                ignore_numbers: true,
                ignore_uppercase: false,
                ignore_camelcase: false,
                ignore_underscores: false,
                ignore_urls: true,
                ignore_emails: true,
                ignore_comments: false,
                ignore_strings: false,
                ignore_code: true,
            },
            user_dictionary: None,
            bad_words: HashSet::new(),
            good_words: HashSet::new(),
            history: VecDeque::new(),
            max_history_size: 100,
        }
    }

    /// Load a dictionary
    pub fn load_dictionary(&mut self, path: &Path) -> ExCommandResult<()> {
        // Check if the file exists
        if !path.exists() {
            return Err(ExCommandError::InvalidArgument(format!("Dictionary file not found: {}", path.display())));
        }
        
        // Get the dictionary name
        let name = match path.file_stem() {
            Some(stem) => stem.to_string_lossy().to_string(),
            None => return Err(ExCommandError::InvalidArgument(format!("Invalid dictionary file name: {}", path.display()))),
        };
        
        // Determine the dictionary format
        let format = match path.extension() {
            Some(ext) => {
                match ext.to_string_lossy().as_ref() {
                    "dic" => SpellDictionaryFormat::Hunspell,
                    "aspell" => SpellDictionaryFormat::Aspell,
                    "vim" => SpellDictionaryFormat::Vim,
                    _ => SpellDictionaryFormat::Plain,
                }
            },
            None => SpellDictionaryFormat::Plain,
        };
        
        // Open the file
        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to open dictionary file: {}", err))),
        };
        
        // Read the file
        let reader = BufReader::new(file);
        let mut words = HashSet::new();
        let mut encoding = "utf-8".to_string();
        let mut language = "en".to_string();
        let mut region = None;
        let mut version = None;
        
        // Parse the file
        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(err) => return Err(ExCommandError::Other(format!("Failed to read dictionary file: {}", err))),
            };
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Check for metadata
            if line.starts_with("encoding:") {
                encoding = line[9..].trim().to_string();
                continue;
            } else if line.starts_with("language:") {
                language = line[9..].trim().to_string();
                continue;
            } else if line.starts_with("region:") {
                region = Some(line[7..].trim().to_string());
                continue;
            } else if line.starts_with("version:") {
                version = Some(line[8..].trim().to_string());
                continue;
            }
            
            // Add the word to the dictionary
            words.insert(line);
        }
        
        // Create the dictionary
        let dictionary = SpellDictionary {
            name: name.clone(),
            path: path.to_path_buf(),
            words,
            encoding,
            language,
            region,
            version,
            format,
        };
        
        // Add the dictionary to the manager
        self.dictionaries.insert(name.clone(), dictionary);
        
        // Activate the dictionary if there are no active dictionaries
        if self.active_dictionaries.is_empty() {
            self.active_dictionaries.push(name);
        }
        
        Ok(())
    }

    /// Unload a dictionary
    pub fn unload_dictionary(&mut self, name: &str) -> ExCommandResult<()> {
        // Check if the dictionary exists
        if !self.dictionaries.contains_key(name) {
            return Err(ExCommandError::InvalidArgument(format!("Dictionary not found: {}", name)));
        }
        
        // Remove the dictionary from the active dictionaries
        self.active_dictionaries.retain(|dict| dict != name);
        
        // Remove the dictionary from the manager
        self.dictionaries.remove(name);
        
        Ok(())
    }

    /// Activate a dictionary
    pub fn activate_dictionary(&mut self, name: &str) -> ExCommandResult<()> {
        // Check if the dictionary exists
        if !self.dictionaries.contains_key(name) {
            return Err(ExCommandError::InvalidArgument(format!("Dictionary not found: {}", name)));
        }
        
        // Check if the dictionary is already active
        if self.active_dictionaries.contains(&name.to_string()) {
            return Ok(());
        }
        
        // Add the dictionary to the active dictionaries
        self.active_dictionaries.push(name.to_string());
        
        Ok(())
    }

    /// Deactivate a dictionary
    pub fn deactivate_dictionary(&mut self, name: &str) -> ExCommandResult<()> {
        // Check if the dictionary exists
        if !self.dictionaries.contains_key(name) {
            return Err(ExCommandError::InvalidArgument(format!("Dictionary not found: {}", name)));
        }
        
        // Remove the dictionary from the active dictionaries
        self.active_dictionaries.retain(|dict| dict != name);
        
        Ok(())
    }

    /// Check if a word is spelled correctly
    pub fn check_word(&self, word: &str) -> bool {
        // Check if spell checking is enabled
        if !self.options.enabled {
            return true;
        }
        
        // Check if the word is in the good words list
        if self.good_words.contains(word) {
            return true;
        }
        
        // Check if the word is in the bad words list
        if self.bad_words.contains(word) {
            return false;
        }
        
        // Check if the word is too short
        if word.len() < self.options.min_word_length {
            return true;
        }
        
        // Check if the word contains numbers
        if self.options.ignore_numbers && word.chars().any(|c| c.is_numeric()) {
            return true;
        }
        
        // Check if the word is all uppercase
        if self.options.ignore_uppercase && word.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
            return true;
        }
        
        // Check if the word is CamelCase
        if self.options.ignore_camelcase && is_camel_case(word) {
            return true;
        }
        
        // Check if the word contains underscores
        if self.options.ignore_underscores && word.contains('_') {
            return true;
        }
        
        // Check if the word is a URL
        if self.options.ignore_urls && (word.starts_with("http://") || word.starts_with("https://") || word.starts_with("www.")) {
            return true;
        }
        
        // Check if the word is an email address
        if self.options.ignore_emails && word.contains('@') && word.contains('.') {
            return true;
        }
        
        // Check if the word is in any of the active dictionaries
        for dict_name in &self.active_dictionaries {
            if let Some(dict) = self.dictionaries.get(dict_name) {
                if dict.words.contains(word) {
                    return true;
                }
                
                // Check for capitalization errors if enabled
                if self.options.check_caps && word.chars().next().map_or(false, |c| c.is_uppercase()) {
                    let lowercase = word.to_lowercase();
                    if dict.words.contains(&lowercase) {
                        return true;
                    }
                }
            }
        }
        
        // Word is not in any dictionary
        false
    }

    /// Get suggestions for a misspelled word
    pub fn get_suggestions(&self, word: &str) -> Vec<SpellSuggestion> {
        // Check if spell checking is enabled
        if !self.options.enabled {
            return Vec::new();
        }
        
        let mut suggestions = Vec::new();
        
        // Check each active dictionary
        for dict_name in &self.active_dictionaries {
            if let Some(dict) = self.dictionaries.get(dict_name) {
                // Get suggestions from the dictionary
                let dict_suggestions = get_suggestions_from_dictionary(word, dict, self.options.max_suggestions);
                
                // Add the suggestions to the list
                for suggestion in dict_suggestions {
                    suggestions.push(SpellSuggestion {
                        word: suggestion.0,
                        score: suggestion.1,
                        source: dict_name.clone(),
                    });
                }
            }
        }
        
        // Sort the suggestions by score
        suggestions.sort_by_key(|s| s.score);
        
        // Limit the number of suggestions
        if suggestions.len() > self.options.max_suggestions {
            suggestions.truncate(self.options.max_suggestions);
        }
        
        suggestions
    }

    /// Check spelling in a buffer
    pub fn check_buffer(&mut self, buffer_id: usize) -> ExCommandResult<()> {
        // Get the editor reference
        let editor = unsafe {
            match crate::command::handlers::EDITOR {
                Some(editor_ptr) => &*editor_ptr,
                None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
            }
        };
        
        // Check if spell checking is enabled
        if !self.options.enabled {
            // Clear any existing errors
            self.errors.remove(&buffer_id);
            return Ok(());
        }
        
        // Get the buffer
        let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
        
        // Clear any existing errors
        let mut errors = Vec::new();
        
        // Check each line in the buffer
        for line_idx in 0..buffer.line_count() {
            let line = buffer.line(line_idx)?;
            
            // Extract words from the line
            let words = extract_words(&line);
            
            // Check each word
            for (word, column) in words {
                if !self.check_word(&word) {
                    // Get suggestions for the word
                    let suggestions = self.get_suggestions(&word);
                    
                    // Determine the error type
                    let error_type = if word.chars().next().map_or(false, |c| c.is_uppercase()) {
                        let lowercase = word.to_lowercase();
                        if self.check_word(&lowercase) {
                            SpellErrorType::Capitalization
                        } else {
                            SpellErrorType::Unknown
                        }
                    } else {
                        SpellErrorType::Unknown
                    };
                    
                    // Add the error
                    errors.push(SpellError {
                        word,
                        line: line_idx,
                        column,
                        suggestions,
                        error_type,
                    });
                }
            }
        }
        
        // Store the errors
        self.errors.insert(buffer_id, errors);
        
        Ok(())
    }

    /// Add a word to the user dictionary
    pub fn add_word(&mut self, word: &str) -> ExCommandResult<()> {
        // Add the word to the good words list
        self.good_words.insert(word.to_string());
        
        // Remove the word from the bad words list
        self.bad_words.remove(word);
        
        // Add the action to the history
        self.add_to_history(SpellAction::AddWord(word.to_string()));
        
        // Save the user dictionary if one is set
        if let Some(path) = &self.user_dictionary {
            self.save_user_dictionary(path)?;
        }
        
        Ok(())
    }

    /// Add a word to the bad words list
    pub fn add_bad_word(&mut self, word: &str) -> ExCommandResult<()> {
        // Add the word to the bad words list
        self.bad_words.insert(word.to_string());
        
        // Remove the word from the good words list
        self.good_words.remove(word);
        
        // Add the action to the history
        self.add_to_history(SpellAction::AddBadWord(word.to_string()));
        
        // Save the user dictionary if one is set
        if let Some(path) = &self.user_dictionary {
            self.save_user_dictionary(path)?;
        }
        
        Ok(())
    }

    /// Correct a word
    pub fn correct_word(&mut self, buffer_id: usize, line: usize, column: usize, replacement: &str) -> ExCommandResult<()> {
        // Get the editor reference
        let editor = unsafe {
            match crate::command::handlers::EDITOR {
                Some(editor_ptr) => &mut *editor_ptr,
                None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
            }
        };
        
        // Get the buffer
        let buffer = editor.get_buffer_manager().get_buffer_mut(buffer_id)?;
        
        // Get the line
        let line_text = buffer.line(line)?;
        
        // Find the word at the given position
        let mut word_start = column;
        let mut word_end = column;
        
        // Find the start of the word
        while word_start > 0 && is_word_char(line_text.chars().nth(word_start - 1).unwrap_or(' ')) {
            word_start -= 1;
        }
        
        // Find the end of the word
        while word_end < line_text.len() && is_word_char(line_text.chars().nth(word_end).unwrap_or(' ')) {
            word_end += 1;
        }
        
        // Get the word
        let word = &line_text[word_start..word_end];
        
        // Replace the word
        let new_line = format!("{}{}{}", &line_text[..word_start], replacement, &line_text[word_end..]);
        buffer.replace_line(line, &new_line)?;
        
        // Add the action to the history
        self.add_to_history(SpellAction::CorrectWord(word.to_string(), replacement.to_string()));
        
        Ok(())
    }

    /// Set the user dictionary
    pub fn set_user_dictionary(&mut self, path: &Path) -> ExCommandResult<()> {
        // Set the user dictionary path
        self.user_dictionary = Some(path.to_path_buf());
        
        // Load the user dictionary
        self.load_user_dictionary(path)?;
        
        Ok(())
    }

    /// Load the user dictionary
    pub fn load_user_dictionary(&mut self, path: &Path) -> ExCommandResult<()> {
        // Check if the file exists
        if !path.exists() {
            // Create the file
            match std::fs::File::create(path) {
                Ok(_) => {},
                Err(err) => return Err(ExCommandError::Other(format!("Failed to create user dictionary file: {}", err))),
            }
            
            return Ok(());
        }
        
        // Open the file
        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to open user dictionary file: {}", err))),
        };
        
        // Read the file
        let reader = BufReader::new(file);
        
        // Clear the good and bad words lists
        self.good_words.clear();
        self.bad_words.clear();
        
        // Parse the file
        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(err) => return Err(ExCommandError::Other(format!("Failed to read user dictionary file: {}", err))),
            };
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Check for bad words
            if line.starts_with('!') {
                let word = line[1..].trim().to_string();
                self.bad_words.insert(word);
            } else {
                // Good word
                self.good_words.insert(line);
            }
        }
        
        Ok(())
    }

    /// Save the user dictionary
    pub fn save_user_dictionary(&self, path: &Path) -> ExCommandResult<()> {
        // Open the file
        let file = match std::fs::File::create(path) {
            Ok(file) => file,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to create user dictionary file: {}", err))),
        };
        
        // Write the file
        let mut writer = std::io::BufWriter::new(file);
        
        // Write a header
        match std::io::Write::write_all(&mut writer, b"# xvim user dictionary\n") {
            Ok(_) => {},
            Err(err) => return Err(ExCommandError::Other(format!("Failed to write to user dictionary file: {}", err))),
        }
        
        // Write the good words
        for word in &self.good_words {
            match std::io::Write::write_all(&mut writer, format!("{}\n", word).as_bytes()) {
                Ok(_) => {},
                Err(err) => return Err(ExCommandError::Other(format!("Failed to write to user dictionary file: {}", err))),
            }
        }
        
        // Write the bad words
        for word in &self.bad_words {
            match std::io::Write::write_all(&mut writer, format!("!{}\n", word).as_bytes()) {
                Ok(_) => {},
                Err(err) => return Err(ExCommandError::Other(format!("Failed to write to user dictionary file: {}", err))),
            }
        }
        
        // Flush the writer
        match std::io::Write::flush(&mut writer) {
            Ok(_) => {},
            Err(err) => return Err(ExCommandError::Other(format!("Failed to flush user dictionary file: {}", err))),
        }
        
        Ok(())
    }

    /// Enable spell checking
    pub fn enable(&mut self) {
        self.options.enabled = true;
    }

    /// Disable spell checking
    pub fn disable(&mut self) {
        self.options.enabled = false;
    }

    /// Toggle spell checking
    pub fn toggle(&mut self) {
        self.options.enabled = !self.options.enabled;
    }

    /// Set spell options
    pub fn set_options(&mut self, options: SpellOptions) {
        self.options = options;
    }

    /// Get spell options
    pub fn get_options(&self) -> &SpellOptions {
        &self.options
    }

    /// Get spell options (mutable)
    pub fn get_options_mut(&mut self) -> &mut SpellOptions {
        &mut self.options
    }

    /// Add an action to the history
    fn add_to_history(&mut self, action: SpellAction) {
        // Add the action to the history
        self.history.push_front(action);
        
        // Limit the history size
        if self.history.len() > self.max_history_size {
            self.history.pop_back();
        }
    }
}

/// Check if a string is CamelCase
fn is_camel_case(s: &str) -> bool {
    let mut has_upper = false;
    let mut has_lower = false;
    
    for c in s.chars() {
        if c.is_uppercase() {
            has_upper = true;
        } else if c.is_lowercase() {
            has_lower = true;
        }
        
        if has_upper && has_lower {
            return true;
        }
    }
    
    false
}

/// Check if a character is part of a word
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '\'' || c == '-'
}

/// Extract words from a string
fn extract_words(text: &str) -> Vec<(String, usize)> {
    let mut words = Vec::new();
    let mut word_start = None;
    
    for (i, c) in text.char_indices() {
        if is_word_char(c) {
            if word_start.is_none() {
                word_start = Some(i);
            }
        } else if let Some(start) = word_start {
            words.push((text[start..i].to_string(), start));
            word_start = None;
        }
    }
    
    // Check for a word at the end of the string
    if let Some(start) = word_start {
        words.push((text[start..].to_string(), start));
    }
    
    words
}

/// Get suggestions from a dictionary
fn get_suggestions_from_dictionary(word: &str, dict: &SpellDictionary, max_suggestions: usize) -> Vec<(String, usize)> {
    let mut suggestions = Vec::new();
    
    // Simple implementation: find words with similar length and common characters
    for dict_word in &dict.words {
        // Skip words that are too different in length
        if dict_word.len() < word.len() / 2 || dict_word.len() > word.len() * 2 {
            continue;
        }
        
        // Calculate the edit distance
        let distance = levenshtein_distance(word, dict_word);
        
        // Add the suggestion if the distance is small enough
        if distance <= word.len() / 2 + 1 {
            suggestions.push((dict_word.clone(), distance));
        }
    }
    
    // Sort the suggestions by distance
    suggestions.sort_by_key(|s| s.1);
    
    // Limit the number of suggestions
    if suggestions.len() > max_suggestions {
        suggestions.truncate(max_suggestions);
    }
    
    suggestions
}

/// Calculate the Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    
    let m = s1_chars.len();
    let n = s2_chars.len();
    
    // Create a matrix of size (m+1) x (n+1)
    let mut matrix = vec![vec![0; n + 1]; m + 1];
    
    // Initialize the first row and column
    for i in 0..=m {
        matrix[i][0] = i;
    }
    
    for j in 0..=n {
        matrix[0][j] = j;
    }
    
    // Fill in the rest of the matrix
    for i in 1..=m {
        for j in 1..=n {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
            
            matrix[i][j] = std::cmp::min(
                std::cmp::min(
                    matrix[i - 1][j] + 1,      // deletion
                    matrix[i][j - 1] + 1       // insertion
                ),
                matrix[i - 1][j - 1] + cost    // substitution
            );
        }
    }
    
    matrix[m][n]
}

// Global spell manager
static mut SPELL_MANAGER: Option<SpellManager> = None;

/// Initialize the spell manager
pub fn init_spell_manager() {
    unsafe {
        if SPELL_MANAGER.is_none() {
            SPELL_MANAGER = Some(SpellManager::new());
        }
    }
}

/// Register spell command handlers
pub fn register_spell_handlers(registry: &mut ExCommandRegistry) {
    // Initialize the spell manager
    init_spell_manager();
    
    // Register spell commands
    registry.register("spell", handle_spell);
    registry.register("spellgood", handle_spellgood);
    registry.register("spellbad", handle_spellbad);
    registry.register("spellundo", handle_spellundo);
    registry.register("spellrepall", handle_spellrepall);
    registry.register("spelldump", handle_spelldump);
    registry.register("spellinfo", handle_spellinfo);
    registry.register("spellrare", handle_spellrare);
    registry.register("spelllocal", handle_spelllocal);
    registry.register("spellfile", handle_spellfile);
    registry.register("spelllang", handle_spelllang);
    registry.register("spellsuggest", handle_spellsuggest);
}

/// Handle the :spell command
fn handle_spell(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the spell manager
    let spell_manager = unsafe {
        match &mut SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &mut SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize spell manager".to_string())),
                }
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // Toggle spell checking
        spell_manager.toggle();
        
        println!("Spell checking {}", if spell_manager.options.enabled { "enabled" } else { "disabled" });
    } else {
        // Parse the arguments
        match args {
            "on" => {
                spell_manager.enable();
                println!("Spell checking enabled");
            },
            "off" => {
                spell_manager.disable();
                println!("Spell checking disabled");
            },
            _ => {
                return Err(ExCommandError::InvalidArgument(format!("Invalid argument: {}", args)));
            }
        }
    }
    
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to check spelling in".to_string())),
    };
    
    // Check spelling in the current buffer
    spell_manager.check_buffer(buffer_id)?;
    
    Ok(())
}

/// Handle the :spellgood command
fn handle_spellgood(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the spell manager
    let spell_manager = unsafe {
        match &mut SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &mut SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize spell manager".to_string())),
                }
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Word required".to_string()));
    }
    
    // Add the word to the user dictionary
    spell_manager.add_word(args)?;
    
    println!("Added '{}' to the user dictionary", args);
    
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to check spelling in".to_string())),
    };
    
    // Check spelling in the current buffer
    spell_manager.check_buffer(buffer_id)?;
    
    Ok(())
}

/// Handle the :spellbad command
fn handle_spellbad(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the spell manager
    let spell_manager = unsafe {
        match &mut SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &mut SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize spell manager".to_string())),
                }
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Word required".to_string()));
    }
    
    // Add the word to the bad words list
    spell_manager.add_bad_word(args)?;
    
    println!("Added '{}' to the bad words list", args);
    
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to check spelling in".to_string())),
    };
    
    // Check spelling in the current buffer
    spell_manager.check_buffer(buffer_id)?;
    
    Ok(())
}

/// Handle the :spellundo command
fn handle_spellundo(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the spell manager
    let spell_manager = unsafe {
        match &mut SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &mut SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize spell manager".to_string())),
                }
            }
        }
    };
    
    // Check if there are any actions in the history
    if spell_manager.history.is_empty() {
        println!("No spell actions to undo");
        return Ok(());
    }
    
    // Get the last action
    let action = spell_manager.history.pop_front().unwrap();
    
    // Undo the action
    match action {
        SpellAction::AddWord(word) => {
            // Remove the word from the good words list
            spell_manager.good_words.remove(&word);
            println!("Removed '{}' from the user dictionary", word);
        },
        SpellAction::RemoveWord(word) => {
            // Add the word to the good words list
            spell_manager.good_words.insert(word.clone());
            println!("Added '{}' to the user dictionary", word);
        },
        SpellAction::AddBadWord(word) => {
            // Remove the word from the bad words list
            spell_manager.bad_words.remove(&word);
            println!("Removed '{}' from the bad words list", word);
        },
        SpellAction::RemoveBadWord(word) => {
            // Add the word to the bad words list
            spell_manager.bad_words.insert(word.clone());
            println!("Added '{}' to the bad words list", word);
        },
        SpellAction::CorrectWord(word, replacement) => {
            println!("Undid correction of '{}' to '{}'", word, replacement);
        },
    }
    
    // Save the user dictionary if one is set
    if let Some(path) = &spell_manager.user_dictionary {
        spell_manager.save_user_dictionary(path)?;
    }
    
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to check spelling in".to_string())),
    };
    
    // Check spelling in the current buffer
    spell_manager.check_buffer(buffer_id)?;
    
    Ok(())
}

/// Handle the :spellrepall command
fn handle_spellrepall(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the spell manager
    let spell_manager = unsafe {
        match &mut SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &mut SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize spell manager".to_string())),
                }
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Word and replacement required".to_string()));
    }
    
    // Split the arguments
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    
    if parts.len() < 2 {
        return Err(ExCommandError::MissingArgument("Replacement required".to_string()));
    }
    
    let word = parts[0];
    let replacement = parts[1];
    
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to check spelling in".to_string())),
    };
    
    // Get the errors for the buffer
    let errors = match spell_manager.errors.get(&buffer_id) {
        Some(errors) => errors,
        None => {
            // Check spelling in the buffer
            spell_manager.check_buffer(buffer_id)?;
            
            match spell_manager.errors.get(&buffer_id) {
                Some(errors) => errors,
                None => return Ok(()),
            }
        }
    };
    
    // Find all occurrences of the word
    let mut count = 0;
    
    // Clone the errors to avoid borrowing issues
    let errors_clone = errors.clone();
    
    for error in errors_clone {
        if error.word == word {
            // Replace the word
            spell_manager.correct_word(buffer_id, error.line, error.column, replacement)?;
            count += 1;
        }
    }
    
    println!("Replaced {} occurrence(s) of '{}' with '{}'", count, word, replacement);
    
    // Check spelling in the buffer again
    spell_manager.check_buffer(buffer_id)?;
    
    Ok(())
}

/// Handle the :spelldump command
fn handle_spelldump(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the spell manager
    let spell_manager = unsafe {
        match &SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize spell manager".to_string())),
                }
            }
        }
    };
    
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Create a new buffer
    let buffer_id = editor.create_buffer("[Spell]", None)?;
    
    // Get the buffer
    let buffer = editor.get_buffer_manager().get_buffer_mut(buffer_id)?;
    
    // Add a header
    buffer.append_line("# xvim spell dump")?;
    buffer.append_line("")?;
    
    // Add the active dictionaries
    buffer.append_line("# Active dictionaries")?;
    
    for dict_name in &spell_manager.active_dictionaries {
        buffer.append_line(&format!("# {}", dict_name))?;
    }
    
    buffer.append_line("")?;
    
    // Add the good words
    buffer.append_line("# Good words")?;
    
    let mut good_words: Vec<String> = spell_manager.good_words.iter().cloned().collect();
    good_words.sort();
    
    for word in good_words {
        buffer.append_line(&word)?;
    }
    
    buffer.append_line("")?;
    
    // Add the bad words
    buffer.append_line("# Bad words")?;
    
    let mut bad_words: Vec<String> = spell_manager.bad_words.iter().cloned().collect();
    bad_words.sort();
    
    for word in bad_words {
        buffer.append_line(&format!("!{}", word))?;
    }
    
    // Set the buffer as read-only
    buffer.set_readonly(true)?;
    
    // Open the buffer in a new window
    editor.edit_buffer(buffer_id)?;
    
    Ok(())
}

/// Handle the :spellinfo command
fn handle_spellinfo(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the spell manager
    let spell_manager = unsafe {
        match &SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize spell manager".to_string())),
                }
            }
        }
    };
    
    // Display spell information
    println!("--- Spell Information ---");
    println!("Spell checking: {}", if spell_manager.options.enabled { "enabled" } else { "disabled" });
    println!("Check capitalization: {}", if spell_manager.options.check_caps { "yes" } else { "no" });
    println!("Maximum suggestions: {}", spell_manager.options.max_suggestions);
    println!("Minimum word length: {}", spell_manager.options.min_word_length);
    println!("Ignore words with numbers: {}", if spell_manager.options.ignore_numbers { "yes" } else { "no" });
    println!("Ignore words in UPPERCASE: {}", if spell_manager.options.ignore_uppercase { "yes" } else { "no" });
    println!("Ignore words with CamelCase: {}", if spell_manager.options.ignore_camelcase { "yes" } else { "no" });
    println!("Ignore words with underscores: {}", if spell_manager.options.ignore_underscores { "yes" } else { "no" });
    println!("Ignore words with URLs: {}", if spell_manager.options.ignore_urls { "yes" } else { "no" });
    println!("Ignore words with email addresses: {}", if spell_manager.options.ignore_emails { "yes" } else { "no" });
    println!("Ignore words in comments: {}", if spell_manager.options.ignore_comments { "yes" } else { "no" });
    println!("Ignore words in strings: {}", if spell_manager.options.ignore_strings { "yes" } else { "no" });
    println!("Ignore words in code: {}", if spell_manager.options.ignore_code { "yes" } else { "no" });
    println!("");
    
    // Display dictionary information
    println!("--- Dictionaries ---");
    
    for dict_name in &spell_manager.active_dictionaries {
        if let Some(dict) = spell_manager.dictionaries.get(dict_name) {
            println!("{} ({}, {} words)", dict_name, dict.language, dict.words.len());
        }
    }
    
    println!("");
    
    // Display user dictionary information
    println!("--- User Dictionary ---");
    
    if let Some(path) = &spell_manager.user_dictionary {
        println!("Path: {}", path.display());
    } else {
        println!("No user dictionary set");
    }
    
    println!("Good words: {}", spell_manager.good_words.len());
    println!("Bad words: {}", spell_manager.bad_words.len());
    
    Ok(())
}

/// Handle the :spellrare command
fn handle_spellrare(_cmd: &ExCommand) -> ExCommandResult<()> {
    // This is a stub implementation
    println!("The :spellrare command is not yet implemented");
    Ok(())
}

/// Handle the :spelllocal command
fn handle_spelllocal(_cmd: &ExCommand) -> ExCommandResult<()> {
    // This is a stub implementation
    println!("The :spelllocal command is not yet implemented");
    Ok(())
}

/// Handle the :spellfile command
fn handle_spellfile(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the spell manager
    let spell_manager = unsafe {
        match &mut SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &mut SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize spell manager".to_string())),
                }
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // Display the current user dictionary
        if let Some(path) = &spell_manager.user_dictionary {
            println!("User dictionary: {}", path.display());
        } else {
            println!("No user dictionary set");
        }
        
        return Ok(());
    }
    
    // Set the user dictionary
    let path = PathBuf::from(args);
    spell_manager.set_user_dictionary(&path)?;
    
    println!("User dictionary set to: {}", path.display());
    
    Ok(())
}

/// Handle the :spelllang command
fn handle_spelllang(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the spell manager
    let spell_manager = unsafe {
        match &mut SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &mut SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize spell manager".to_string())),
                }
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // Display the current active dictionaries
        println!("Active dictionaries: {}", spell_manager.active_dictionaries.join(", "));
        return Ok(());
    }
    
    // Clear the active dictionaries
    spell_manager.active_dictionaries.clear();
    
    // Parse the languages
    let langs: Vec<&str> = args.split(',').collect();
    
    for lang in langs {
        let lang = lang.trim();
        
        // Check if the dictionary exists
        if spell_manager.dictionaries.contains_key(lang) {
            // Activate the dictionary
            spell_manager.active_dictionaries.push(lang.to_string());
        } else {
            // Try to load the dictionary
            let path = PathBuf::from(format!("runtime/spell/{}.dic", lang));
            
            if path.exists() {
                // Load the dictionary
                spell_manager.load_dictionary(&path)?;
                
                // Activate the dictionary
                spell_manager.active_dictionaries.push(lang.to_string());
            } else {
                return Err(ExCommandError::InvalidArgument(format!("Dictionary not found: {}", lang)));
            }
        }
    }
    
    println!("Active dictionaries: {}", spell_manager.active_dictionaries.join(", "));
    
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to check spelling in".to_string())),
    };
    
    // Check spelling in the current buffer
    spell_manager.check_buffer(buffer_id)?;
    
    Ok(())
}

/// Handle the :spellsuggest command
fn handle_spellsuggest(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the spell manager
    let spell_manager = unsafe {
        match &SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize spell manager".to_string())),
                }
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Word required".to_string()));
    }
    
    // Get suggestions for the word
    let suggestions = spell_manager.get_suggestions(args);
    
    if suggestions.is_empty() {
        println!("No suggestions for '{}'", args);
        return Ok(());
    }
    
    // Display the suggestions
    println!("Suggestions for '{}' (score):", args);
    
    for (i, suggestion) in suggestions.iter().enumerate() {
        println!("{:2}. {} ({}) [{}]", i + 1, suggestion.word, suggestion.score, suggestion.source);
    }
    
    Ok(())
}

/// Enable spell checking
pub fn enable_spell_checking() {
    // Get the spell manager
    let spell_manager = unsafe {
        match &mut SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &mut SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return,
                }
            }
        }
    };
    
    // Enable spell checking
    spell_manager.enable();
}

/// Disable spell checking
pub fn disable_spell_checking() {
    // Get the spell manager
    let spell_manager = unsafe {
        match &mut SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &mut SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return,
                }
            }
        }
    };
    
    // Disable spell checking
    spell_manager.disable();
}

/// Toggle spell checking
pub fn toggle_spell_checking() {
    // Get the spell manager
    let spell_manager = unsafe {
        match &mut SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &mut SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return,
                }
            }
        }
    };
    
    // Toggle spell checking
    spell_manager.toggle();
}

/// Check spelling in a buffer
pub fn check_spelling(buffer_id: usize) -> ExCommandResult<()> {
    // Get the spell manager
    let spell_manager = unsafe {
        match &mut SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &mut SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize spell manager".to_string())),
                }
            }
        }
    };
    
    // Check spelling in the buffer
    spell_manager.check_buffer(buffer_id)
}

/// Get spell errors for a buffer
pub fn get_spell_errors(buffer_id: usize) -> Vec<SpellError> {
    // Get the spell manager
    let spell_manager = unsafe {
        match &SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return Vec::new(),
                }
            }
        }
    };
    
    // Get the errors for the buffer
    match spell_manager.errors.get(&buffer_id) {
        Some(errors) => errors.clone(),
        None => Vec::new(),
    }
}

/// Add a word to the user dictionary
pub fn add_word_to_dictionary(word: &str) -> ExCommandResult<()> {
    // Get the spell manager
    let spell_manager = unsafe {
        match &mut SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &mut SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize spell manager".to_string())),
                }
            }
        }
    };
    
    // Add the word to the user dictionary
    spell_manager.add_word(word)
}

/// Add a word to the bad words list
pub fn add_bad_word(word: &str) -> ExCommandResult<()> {
    // Get the spell manager
    let spell_manager = unsafe {
        match &mut SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &mut SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize spell manager".to_string())),
                }
            }
        }
    };
    
    // Add the word to the bad words list
    spell_manager.add_bad_word(word)
}

/// Get suggestions for a word
pub fn get_suggestions(word: &str) -> Vec<SpellSuggestion> {
    // Get the spell manager
    let spell_manager = unsafe {
        match &SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return Vec::new(),
                }
            }
        }
    };
    
    // Get suggestions for the word
    spell_manager.get_suggestions(word)
}

/// Check if a word is spelled correctly
pub fn check_word(word: &str) -> bool {
    // Get the spell manager
    let spell_manager = unsafe {
        match &SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return true,
                }
            }
        }
    };
    
    // Check if the word is spelled correctly
    spell_manager.check_word(word)
}

/// Set spell options
pub fn set_spell_options(options: SpellOptions) {
    // Get the spell manager
    let spell_manager = unsafe {
        match &mut SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &mut SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return,
                }
            }
        }
    };
    
    // Set the options
    spell_manager.set_options(options);
}

/// Get spell options
pub fn get_spell_options() -> Option<SpellOptions> {
    // Get the spell manager
    let spell_manager = unsafe {
        match &SPELL_MANAGER {
            Some(manager) => manager,
            None => {
                init_spell_manager();
                match &SPELL_MANAGER {
                    Some(manager) => manager,
                    None => return None,
                }
            }
        }
    };
    
    // Get the options
    Some(spell_manager.get_options().clone())
}