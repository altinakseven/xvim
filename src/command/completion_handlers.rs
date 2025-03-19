//! Completion command handlers
//!
//! This module implements handlers for completion commands.

use crate::command::{ExCommand, ExCommandError, ExCommandResult, ExCommandRegistry};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
// use std::io::Read;
// use std::fs::File;
use std::sync::{Arc, Mutex};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
// use std::time::SystemTime;
// use std::env;
// use std::path::Path;

/// Completion type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionType {
    /// Keyword completion
    Keyword,
    /// Line completion
    Line,
    /// File completion
    File,
    /// Tag completion
    Tag,
    /// Dictionary completion
    Dictionary,
    /// Thesaurus completion
    Thesaurus,
    /// User defined completion
    UserDefined,
    /// Omni completion
    Omni,
    /// Spelling completion
    Spelling,
}

/// Completion item
#[derive(Debug, Clone)]
pub struct CompletionItem {
    /// Item text
    pub text: String,
    /// Item kind
    pub kind: String,
    /// Item menu
    pub menu: Option<String>,
    /// Item info
    pub info: Option<String>,
    /// Item source
    pub source: CompletionType,
}

/// Completion context
#[derive(Debug, Clone)]
pub struct CompletionContext {
    /// Base word
    pub base: String,
    /// Start position
    pub start_pos: usize,
    /// End position
    pub end_pos: usize,
    /// Completion type
    pub completion_type: CompletionType,
    /// Buffer ID
    pub buffer_id: usize,
    /// Line number
    pub line: usize,
    /// Column number
    pub column: usize,
}

/// Completion manager
#[derive(Debug)]
pub struct CompletionManager {
    /// Completion items
    pub items: Vec<CompletionItem>,
    /// Current completion index
    pub current_index: usize,
    /// Completion context
    pub context: Option<CompletionContext>,
    /// Completion history
    pub history: VecDeque<Vec<CompletionItem>>,
    /// Maximum history size
    pub max_history_size: usize,
    /// Keyword dictionaries
    pub keyword_dicts: HashMap<String, HashSet<String>>,
    /// User defined completions
    pub user_defined_completions: HashMap<String, Box<dyn Fn(&str) -> Vec<CompletionItem> + Send + Sync>>,
    /// Omni completions
    pub omni_completions: HashMap<String, Box<dyn Fn(&str) -> Vec<CompletionItem> + Send + Sync>>,
    /// Completion options
    pub options: CompletionOptions,
}

/// Completion options
#[derive(Debug, Clone)]
pub struct CompletionOptions {
    /// Ignore case
    pub ignore_case: bool,
    /// Match case
    pub match_case: bool,
    /// Menu height
    pub menu_height: usize,
    /// Preview
    pub preview: bool,
    /// Popup menu
    pub popup_menu: bool,
    /// Longest match
    pub longest: bool,
    /// Full menu
    pub full_menu: bool,
    /// Menu scroll
    pub menu_scroll: usize,
    /// Selection highlight
    pub selection_highlight: bool,
    /// Keyword pattern
    pub keyword_pattern: String,
}

impl CompletionManager {
    /// Create a new completion manager
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            current_index: 0,
            context: None,
            history: VecDeque::new(),
            max_history_size: 10,
            keyword_dicts: HashMap::new(),
            user_defined_completions: HashMap::new(),
            omni_completions: HashMap::new(),
            options: CompletionOptions {
                ignore_case: true,
                match_case: false,
                menu_height: 10,
                preview: true,
                popup_menu: true,
                longest: false,
                full_menu: false,
                menu_scroll: 5,
                selection_highlight: true,
                keyword_pattern: r"\w+".to_string(),
            },
        }
    }

    /// Start completion
    pub fn start_completion(&mut self, context: CompletionContext, items: Vec<CompletionItem>) -> ExCommandResult<()> {
        // Set the context
        self.context = Some(context);
        
        // Set the items
        self.items = items;
        
        // Reset the current index
        self.current_index = 0;
        
        // Add to history
        if !self.items.is_empty() {
            self.history.push_front(self.items.clone());
            
            // Limit the history size
            if self.history.len() > self.max_history_size {
                self.history.pop_back();
            }
        }
        
        Ok(())
    }

    /// Next completion
    pub fn next_completion(&mut self) -> Option<&CompletionItem> {
        if self.items.is_empty() {
            return None;
        }
        
        // Increment the current index
        self.current_index = (self.current_index + 1) % self.items.len();
        
        // Get the current item
        self.items.get(self.current_index)
    }

    /// Previous completion
    pub fn prev_completion(&mut self) -> Option<&CompletionItem> {
        if self.items.is_empty() {
            return None;
        }
        
        // Decrement the current index
        self.current_index = if self.current_index == 0 {
            self.items.len() - 1
        } else {
            self.current_index - 1
        };
        
        // Get the current item
        self.items.get(self.current_index)
    }

    /// Get the current completion
    pub fn current_completion(&self) -> Option<&CompletionItem> {
        if self.items.is_empty() {
            return None;
        }
        
        // Get the current item
        self.items.get(self.current_index)
    }

    /// Cancel completion
    pub fn cancel_completion(&mut self) {
        // Clear the context
        self.context = None;
        
        // Clear the items
        self.items.clear();
        
        // Reset the current index
        self.current_index = 0;
    }

    /// Accept completion
    pub fn accept_completion(&mut self) -> Option<&CompletionItem> {
        // Get the current item
        let item = self.current_completion();
        
        // Clear the context
        self.context = None;
        
        item
    }

    /// Find completions
    pub fn find_completions(&mut self, base: &str, completion_type: CompletionType, buffer_id: usize, line: usize, column: usize) -> ExCommandResult<Vec<CompletionItem>> {
        // Create the context
        let context = CompletionContext {
            base: base.to_string(),
            start_pos: column - base.len(),
            end_pos: column,
            completion_type,
            buffer_id,
            line,
            column,
        };
        
        // Find completions based on the type
        let items = match completion_type {
            CompletionType::Keyword => self.find_keyword_completions(base, buffer_id)?,
            CompletionType::Line => self.find_line_completions(base, buffer_id)?,
            CompletionType::File => self.find_file_completions(base)?,
            CompletionType::Tag => self.find_tag_completions(base)?,
            CompletionType::Dictionary => self.find_dictionary_completions(base)?,
            CompletionType::Thesaurus => self.find_thesaurus_completions(base)?,
            CompletionType::UserDefined => self.find_user_defined_completions(base)?,
            CompletionType::Omni => self.find_omni_completions(base, buffer_id)?,
            CompletionType::Spelling => self.find_spelling_completions(base)?,
        };
        
        // Start completion
        self.start_completion(context, items.clone())?;
        
        Ok(items)
    }

    /// Find keyword completions
    pub fn find_keyword_completions(&self, base: &str, buffer_id: usize) -> ExCommandResult<Vec<CompletionItem>> {
        // Get the editor reference
        let editor = unsafe {
            match crate::command::handlers::EDITOR {
                Some(editor_ptr) => &*editor_ptr,
                None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
            }
        };
        
        // Get the buffer
        let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
        
        // Get all words in the buffer
        let mut words = HashSet::new();
        
        for line_idx in 0..buffer.line_count() {
            let line = buffer.line(line_idx)?;
            
            // Extract words from the line
            let line_words = extract_words(&line, &self.options.keyword_pattern);
            
            // Add words to the set
            for word in line_words {
                if word.starts_with(base) && word != base {
                    words.insert(word);
                }
            }
        }
        
        // Convert words to completion items
        let items: Vec<CompletionItem> = words.into_iter()
            .map(|word| CompletionItem {
                text: word.clone(),
                kind: "keyword".to_string(),
                menu: None,
                info: None,
                source: CompletionType::Keyword,
            })
            .collect();
        
        Ok(items)
    }

    /// Find line completions
    pub fn find_line_completions(&self, base: &str, buffer_id: usize) -> ExCommandResult<Vec<CompletionItem>> {
        // Get the editor reference
        let editor = unsafe {
            match crate::command::handlers::EDITOR {
                Some(editor_ptr) => &*editor_ptr,
                None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
            }
        };
        
        // Get the buffer
        let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
        
        // Get all lines in the buffer
        let mut lines = HashSet::new();
        
        for line_idx in 0..buffer.line_count() {
            let line = buffer.line(line_idx)?;
            
            if line.starts_with(base) && line != base {
                lines.insert(line);
            }
        }
        
        // Convert lines to completion items
        let items: Vec<CompletionItem> = lines.into_iter()
            .map(|line| CompletionItem {
                text: line.clone(),
                kind: "line".to_string(),
                menu: None,
                info: None,
                source: CompletionType::Line,
            })
            .collect();
        
        Ok(items)
    }

    /// Find file completions
    pub fn find_file_completions(&self, base: &str) -> ExCommandResult<Vec<CompletionItem>> {
        // Get the current directory
        let current_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to get current directory: {}", err))),
        };
        
        // Get the base directory and file prefix
        let (base_dir, file_prefix) = if base.contains('/') {
            let last_slash = base.rfind('/').unwrap();
            let dir = &base[..=last_slash];
            let prefix = &base[last_slash + 1..];
            (current_dir.join(dir), prefix.to_string())
        } else {
            (current_dir, base.to_string())
        };
        
        // Read the directory
        let entries = match std::fs::read_dir(&base_dir) {
            Ok(entries) => entries,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to read directory: {}", err))),
        };
        
        // Filter and convert entries to completion items
        let mut items = Vec::new();
        
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy().to_string();
                
                if file_name_str.starts_with(&file_prefix) {
                    let file_type = match entry.file_type() {
                        Ok(file_type) => {
                            if file_type.is_dir() {
                                "directory".to_string()
                            } else if file_type.is_file() {
                                "file".to_string()
                            } else {
                                "other".to_string()
                            }
                        },
                        Err(_) => "unknown".to_string(),
                    };
                    
                    let path = entry.path();
                    let path_str = path.to_string_lossy().to_string();
                    
                    items.push(CompletionItem {
                        text: file_name_str,
                        kind: file_type,
                        menu: Some(path_str),
                        info: None,
                        source: CompletionType::File,
                    });
                }
            }
        }
        
        Ok(items)
    }

    /// Find tag completions
    pub fn find_tag_completions(&self, base: &str) -> ExCommandResult<Vec<CompletionItem>> {
        // Get the tag database
        let tag_database = unsafe {
            match &crate::command::tag_handlers::TAG_DATABASE {
                Some(database) => database,
                None => {
                    crate::command::tag_handlers::init_tag_database();
                    match &crate::command::tag_handlers::TAG_DATABASE {
                        Some(database) => database,
                        None => return Err(ExCommandError::Other("Failed to initialize tag database".to_string())),
                    }
                }
            }
        };
        
        // Find tags that match the base
        let mut items = Vec::new();
        
        for (tag_name, tag_entries) in &tag_database.tags {
            if tag_name.starts_with(base) {
                for (i, entry) in tag_entries.iter().enumerate() {
                    items.push(CompletionItem {
                        text: tag_name.clone(),
                        kind: entry.kind.clone().unwrap_or_else(|| "tag".to_string()),
                        menu: Some(entry.file.to_string_lossy().to_string()),
                        info: Some(entry.pattern.clone()),
                        source: CompletionType::Tag,
                    });
                    
                    // Only add the first few entries for each tag
                    if i >= 5 {
                        break;
                    }
                }
            }
        }
        
        Ok(items)
    }

    /// Find dictionary completions
    pub fn find_dictionary_completions(&self, base: &str) -> ExCommandResult<Vec<CompletionItem>> {
        // Get the dictionary words
        let mut words = HashSet::new();
        
        for (_, dict) in &self.keyword_dicts {
            for word in dict {
                if word.starts_with(base) && word != base {
                    words.insert(word.clone());
                }
            }
        }
        
        // Convert words to completion items
        let items: Vec<CompletionItem> = words.into_iter()
            .map(|word| CompletionItem {
                text: word.clone(),
                kind: "dictionary".to_string(),
                menu: None,
                info: None,
                source: CompletionType::Dictionary,
            })
            .collect();
        
        Ok(items)
    }

    /// Find thesaurus completions
    pub fn find_thesaurus_completions(&self, base: &str) -> ExCommandResult<Vec<CompletionItem>> {
        // This is a stub implementation
        // In a real implementation, we would look up synonyms in a thesaurus
        Ok(Vec::new())
    }

    /// Find user defined completions
    pub fn find_user_defined_completions(&self, base: &str) -> ExCommandResult<Vec<CompletionItem>> {
        // Get the current filetype
        let editor = unsafe {
            match crate::command::handlers::EDITOR {
                Some(editor_ptr) => &*editor_ptr,
                None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
            }
        };
        
        // Get the current buffer ID
        let buffer_id = match editor.current_buffer_id() {
            Some(id) => id,
            None => return Err(ExCommandError::InvalidCommand("No buffer to complete in".to_string())),
        };
        
        // Get the buffer
        let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
        
        // Get the filetype
        let filetype = buffer.filetype().unwrap_or_else(|| "text".to_string());
        
        // Check if we have a user defined completion for this filetype
        if let Some(completion_fn) = self.user_defined_completions.get(&filetype) {
            // Call the completion function
            let items = completion_fn(base);
            
            Ok(items)
        } else {
            // No user defined completion for this filetype
            Ok(Vec::new())
        }
    }

    /// Find omni completions
    pub fn find_omni_completions(&self, base: &str, buffer_id: usize) -> ExCommandResult<Vec<CompletionItem>> {
        // Get the editor reference
        let editor = unsafe {
            match crate::command::handlers::EDITOR {
                Some(editor_ptr) => &*editor_ptr,
                None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
            }
        };
        
        // Get the buffer
        let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
        
        // Get the filetype
        let filetype = buffer.filetype().unwrap_or_else(|| "text".to_string());
        
        // Check if we have an omni completion for this filetype
        if let Some(completion_fn) = self.omni_completions.get(&filetype) {
            // Call the completion function
            let items = completion_fn(base);
            
            Ok(items)
        } else {
            // No omni completion for this filetype
            Ok(Vec::new())
        }
    }

    /// Find spelling completions
    pub fn find_spelling_completions(&self, base: &str) -> ExCommandResult<Vec<CompletionItem>> {
        // This is a stub implementation
        // In a real implementation, we would look up spelling suggestions
        Ok(Vec::new())
    }

    /// Register a user defined completion
    pub fn register_user_defined_completion(&mut self, filetype: &str, completion_fn: Box<dyn Fn(&str) -> Vec<CompletionItem> + Send + Sync>) {
        self.user_defined_completions.insert(filetype.to_string(), completion_fn);
    }

    /// Register an omni completion
    pub fn register_omni_completion(&mut self, filetype: &str, completion_fn: Box<dyn Fn(&str) -> Vec<CompletionItem> + Send + Sync>) {
        self.omni_completions.insert(filetype.to_string(), completion_fn);
    }

    /// Add a keyword dictionary
    pub fn add_keyword_dictionary(&mut self, name: &str, words: HashSet<String>) {
        self.keyword_dicts.insert(name.to_string(), words);
    }

    /// Remove a keyword dictionary
    pub fn remove_keyword_dictionary(&mut self, name: &str) {
        self.keyword_dicts.remove(name);
    }

    /// Set completion options
    pub fn set_options(&mut self, options: CompletionOptions) {
        self.options = options;
    }

    /// Get completion options
    pub fn get_options(&self) -> &CompletionOptions {
        &self.options
    }

    /// Get completion options (mutable)
    pub fn get_options_mut(&mut self) -> &mut CompletionOptions {
        &mut self.options
    }
}

/// Extract words from a string
fn extract_words(text: &str, pattern: &str) -> Vec<String> {
    // Use regex to match the pattern
    let regex = match regex::Regex::new(pattern) {
        Ok(re) => re,
        Err(_) => {
            // Fallback to simple whitespace splitting if regex is invalid
            return text.split_whitespace().map(|s| s.to_string()).collect();
        }
    };
    
    // Find all matches
    let mut words = Vec::new();
    for mat in regex.find_iter(text) {
        words.push(mat.as_str().to_string());
    }
    
    words
}

// Global completion manager
static mut COMPLETION_MANAGER: Option<CompletionManager> = None;

/// Initialize the completion manager
pub fn init_completion_manager() {
    unsafe {
        if COMPLETION_MANAGER.is_none() {
            COMPLETION_MANAGER = Some(CompletionManager::new());
        }
    }
}

/// Register completion command handlers
pub fn register_completion_handlers(registry: &mut ExCommandRegistry) {
    // Initialize the completion manager
    init_completion_manager();
    
    // Register completion commands
    registry.register("complete", handle_complete);
    registry.register("completions", handle_completions);
    registry.register("setcomplete", handle_setcomplete);
    registry.register("setcompleteopt", handle_setcompleteopt);
}

/// Handle the :complete command
fn handle_complete(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the completion manager
    let completion_manager = unsafe {
        match &mut COMPLETION_MANAGER {
            Some(manager) => manager,
            None => {
                init_completion_manager();
                match &mut COMPLETION_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize completion manager".to_string())),
                }
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Completion type and base required".to_string()));
    }
    
    // Split the arguments
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    
    if parts.len() < 2 {
        return Err(ExCommandError::MissingArgument("Base required".to_string()));
    }
    
    // Parse the completion type
    let completion_type = match parts[0] {
        "keyword" => CompletionType::Keyword,
        "line" => CompletionType::Line,
        "file" => CompletionType::File,
        "tag" => CompletionType::Tag,
        "dictionary" => CompletionType::Dictionary,
        "thesaurus" => CompletionType::Thesaurus,
        "user" => CompletionType::UserDefined,
        "omni" => CompletionType::Omni,
        "spelling" => CompletionType::Spelling,
        _ => return Err(ExCommandError::InvalidArgument(format!("Invalid completion type: {}", parts[0]))),
    };
    
    // Get the base
    let base = parts[1];
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to complete in".to_string())),
    };
    
    // Get the current cursor position
    let cursor_pos = editor.cursor_position();
    
    // Find completions
    let items = completion_manager.find_completions(base, completion_type, buffer_id, cursor_pos.line, cursor_pos.column)?;
    
    // Display the completions
    println!("--- Completions ---");
    
    for (i, item) in items.iter().enumerate() {
        let current_marker = if i == completion_manager.current_index { ">" } else { " " };
        
        println!("{} {:3} {} [{}]{}{}",
            current_marker,
            i,
            item.text,
            item.kind,
            if let Some(menu) = &item.menu { format!(" {}", menu) } else { String::new() },
            if let Some(info) = &item.info { format!(" {}", info) } else { String::new() }
        );
    }
    
    Ok(())
}

/// Handle the :completions command
fn handle_completions(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the completion manager
    let completion_manager = unsafe {
        match &COMPLETION_MANAGER {
            Some(manager) => manager,
            None => {
                init_completion_manager();
                match &COMPLETION_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize completion manager".to_string())),
                }
            }
        }
    };
    
    // Display the current completions
    if let Some(context) = &completion_manager.context {
        println!("--- Completions ---");
        println!("Base: {}", context.base);
        println!("Type: {:?}", context.completion_type);
        println!("Buffer: {}", context.buffer_id);
        println!("Position: ({}, {})", context.line, context.column);
        
        for (i, item) in completion_manager.items.iter().enumerate() {
            let current_marker = if i == completion_manager.current_index { ">" } else { " " };
            
            println!("{} {:3} {} [{}]{}{}",
                current_marker,
                i,
                item.text,
                item.kind,
                if let Some(menu) = &item.menu { format!(" {}", menu) } else { String::new() },
                if let Some(info) = &item.info { format!(" {}", info) } else { String::new() }
            );
        }
    } else {
        println!("No active completion");
    }
    
    Ok(())
}

/// Handle the :setcomplete command
fn handle_setcomplete(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the completion manager
    let completion_manager = unsafe {
        match &mut COMPLETION_MANAGER {
            Some(manager) => manager,
            None => {
                init_completion_manager();
                match &mut COMPLETION_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize completion manager".to_string())),
                }
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // Display the current options
        println!("--- Completion Options ---");
        println!("ignore_case: {}", completion_manager.options.ignore_case);
        println!("match_case: {}", completion_manager.options.match_case);
        println!("menu_height: {}", completion_manager.options.menu_height);
        println!("preview: {}", completion_manager.options.preview);
        println!("popup_menu: {}", completion_manager.options.popup_menu);
        println!("longest: {}", completion_manager.options.longest);
        println!("full_menu: {}", completion_manager.options.full_menu);
        println!("menu_scroll: {}", completion_manager.options.menu_scroll);
        println!("selection_highlight: {}", completion_manager.options.selection_highlight);
        println!("keyword_pattern: {}", completion_manager.options.keyword_pattern);
        
        return Ok(());
    }
    
    // Split the arguments
    let parts: Vec<&str> = args.splitn(2, '=').collect();
    
    if parts.len() < 2 {
        return Err(ExCommandError::InvalidArgument("Invalid option format".to_string()));
    }
    
    // Parse the option name
    let option_name = parts[0].trim();
    let option_value = parts[1].trim();
    
    // Set the option
    match option_name {
        "ignore_case" => {
            completion_manager.options.ignore_case = match option_value {
                "true" | "1" | "yes" | "on" => true,
                "false" | "0" | "no" | "off" => false,
                _ => return Err(ExCommandError::InvalidArgument(format!("Invalid boolean value: {}", option_value))),
            };
        },
        "match_case" => {
            completion_manager.options.match_case = match option_value {
                "true" | "1" | "yes" | "on" => true,
                "false" | "0" | "no" | "off" => false,
                _ => return Err(ExCommandError::InvalidArgument(format!("Invalid boolean value: {}", option_value))),
            };
        },
        "menu_height" => {
            completion_manager.options.menu_height = match option_value.parse::<usize>() {
                Ok(value) => value,
                Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid number: {}", option_value))),
            };
        },
        "preview" => {
            completion_manager.options.preview = match option_value {
                "true" | "1" | "yes" | "on" => true,
                "false" | "0" | "no" | "off" => false,
                _ => return Err(ExCommandError::InvalidArgument(format!("Invalid boolean value: {}", option_value))),
            };
        },
        "popup_menu" => {
            completion_manager.options.popup_menu = match option_value {
                "true" | "1" | "yes" | "on" => true,
                "false" | "0" | "no" | "off" => false,
                _ => return Err(ExCommandError::InvalidArgument(format!("Invalid boolean value: {}", option_value))),
            };
        },
        "longest" => {
            completion_manager.options.longest = match option_value {
                "true" | "1" | "yes" | "on" => true,
                "false" | "0" | "no" | "off" => false,
                _ => return Err(ExCommandError::InvalidArgument(format!("Invalid boolean value: {}", option_value))),
            };
        },
        "full_menu" => {
            completion_manager.options.full_menu = match option_value {
                "true" | "1" | "yes" | "on" => true,
                "false" | "0" | "no" | "off" => false,
                _ => return Err(ExCommandError::InvalidArgument(format!("Invalid boolean value: {}", option_value))),
            };
        },
        "menu_scroll" => {
            completion_manager.options.menu_scroll = match option_value.parse::<usize>() {
                Ok(value) => value,
                Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid number: {}", option_value))),
            };
        },
        "selection_highlight" => {
            completion_manager.options.selection_highlight = match option_value {
                "true" | "1" | "yes" | "on" => true,
                "false" | "0" | "no" | "off" => false,
                _ => return Err(ExCommandError::InvalidArgument(format!("Invalid boolean value: {}", option_value))),
            };
        },
        "keyword_pattern" => {
            completion_manager.options.keyword_pattern = option_value.to_string();
        },
        _ => return Err(ExCommandError::InvalidArgument(format!("Unknown option: {}", option_name))),
    }
    
    println!("Option {} set to {}", option_name, option_value);
    Ok(())
}

/// Handle the :setcompleteopt command
fn handle_setcompleteopt(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the completion manager
    let completion_manager = unsafe {
        match &mut COMPLETION_MANAGER {
            Some(manager) => manager,
            None => {
                init_completion_manager();
                match &mut COMPLETION_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize completion manager".to_string())),
                }
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // Display the current options
        let mut options = Vec::new();
        
        if completion_manager.options.menu_height > 0 {
            options.push(format!("menu={}", completion_manager.options.menu_height));
        }
        
        if completion_manager.options.preview {
            options.push("preview".to_string());
        }
        
        if completion_manager.options.popup_menu {
            options.push("popup".to_string());
        }
        
        if completion_manager.options.longest {
            options.push("longest".to_string());
        }
        
        if completion_manager.options.full_menu {
            options.push("menuone".to_string());
        }
        
        println!("completeopt={}", options.join(","));
        
        return Ok(());
    }
    
    // Reset options to defaults
    completion_manager.options.menu_height = 0;
    completion_manager.options.preview = false;
    completion_manager.options.popup_menu = false;
    completion_manager.options.longest = false;
    completion_manager.options.full_menu = false;
    
    // Parse the options
    let options: Vec<&str> = args.split(',').collect();
    
    for option in options {
        let option = option.trim();
        
        if option.starts_with("menu=") {
            let value = &option[5..];
            
            completion_manager.options.menu_height = match value.parse::<usize>() {
                Ok(value) => value,
                Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid menu height: {}", value))),
            };
        } else if option == "preview" {
            completion_manager.options.preview = true;
        } else if option == "popup" {
            completion_manager.options.popup_menu = true;
        } else if option == "longest" {
            completion_manager.options.longest = true;
        } else if option == "menuone" {
            completion_manager.options.full_menu = true;
        } else {
            return Err(ExCommandError::InvalidArgument(format!("Unknown option: {}", option)));
        }
    }
    
    println!("completeopt set to {}", args);
    Ok(())
}

/// Start completion
pub fn start_completion(context: CompletionContext, items: Vec<CompletionItem>) -> ExCommandResult<()> {
    // Get the completion manager
    let completion_manager = unsafe {
        match &mut COMPLETION_MANAGER {
            Some(manager) => manager,
            None => {
                init_completion_manager();
                match &mut COMPLETION_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize completion manager".to_string())),
                }
            }
        }
    };
    
    // Start completion
    completion_manager.start_completion(context, items)
}

/// Next completion
pub fn next_completion() -> Option<CompletionItem> {
    // Get the completion manager
    let completion_manager = unsafe {
        match &mut COMPLETION_MANAGER {
            Some(manager) => manager,
            None => {
                init_completion_manager();
                match &mut COMPLETION_MANAGER {
                    Some(manager) => manager,
                    None => return None,
                }
            }
        }
    };
    
    // Get the next completion
    completion_manager.next_completion().cloned()
}

/// Previous completion
pub fn prev_completion() -> Option<CompletionItem> {
    // Get the completion manager
    let completion_manager = unsafe {
        match &mut COMPLETION_MANAGER {
            Some(manager) => manager,
            None => {
                init_completion_manager();
                match &mut COMPLETION_MANAGER {
                    Some(manager) => manager,
                    None => return None,
                }
            }
        }
    };
    
    // Get the previous completion
    completion_manager.prev_completion().cloned()
}

/// Current completion
pub fn current_completion() -> Option<CompletionItem> {
    // Get the completion manager
    let completion_manager = unsafe {
        match &COMPLETION_MANAGER {
            Some(manager) => manager,
            None => {
                init_completion_manager();
                match &COMPLETION_MANAGER {
                    Some(manager) => manager,
                    None => return None,
                }
            }
        }
    };
    
    // Get the current completion
    completion_manager.current_completion().cloned()
}

/// Cancel completion
pub fn cancel_completion() {
    // Get the completion manager
    let completion_manager = unsafe {
        match &mut COMPLETION_MANAGER {
            Some(manager) => manager,
            None => {
                init_completion_manager();
                match &mut COMPLETION_MANAGER {
                    Some(manager) => manager,
                    None => return,
                }
            }
        }
    };
    
    // Cancel completion
    completion_manager.cancel_completion();
}

/// Accept completion
pub fn accept_completion() -> Option<CompletionItem> {
    // Get the completion manager
    let completion_manager = unsafe {
        match &mut COMPLETION_MANAGER {
            Some(manager) => manager,
            None => {
                init_completion_manager();
                match &mut COMPLETION_MANAGER {
                    Some(manager) => manager,
                    None => return None,
                }
            }
        }
    };
    
    // Accept completion
    completion_manager.accept_completion().cloned()
}

/// Find completions
pub fn find_completions(base: &str, completion_type: CompletionType, buffer_id: usize, line: usize, column: usize) -> ExCommandResult<Vec<CompletionItem>> {
    // Get the completion manager
    let completion_manager = unsafe {
        match &mut COMPLETION_MANAGER {
            Some(manager) => manager,
            None => {
                init_completion_manager();
                match &mut COMPLETION_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize completion manager".to_string())),
                }
            }
        }
    };
    
    // Find completions
    completion_manager.find_completions(base, completion_type, buffer_id, line, column)
}

/// Register a user defined completion
pub fn register_user_defined_completion(filetype: &str, completion_fn: Box<dyn Fn(&str) -> Vec<CompletionItem> + Send + Sync>) {
    // Get the completion manager
    let completion_manager = unsafe {
        match &mut COMPLETION_MANAGER {
            Some(manager) => manager,
            None => {
                init_completion_manager();
                match &mut COMPLETION_MANAGER {
                    Some(manager) => manager,
                    None => return,
                }
            }
        }
    };
    
    // Register the completion
    completion_manager.register_user_defined_completion(filetype, completion_fn);
}

/// Register an omni completion
pub fn register_omni_completion(filetype: &str, completion_fn: Box<dyn Fn(&str) -> Vec<CompletionItem> + Send + Sync>) {
    // Get the completion manager
    let completion_manager = unsafe {
        match &mut COMPLETION_MANAGER {
            Some(manager) => manager,
            None => {
                init_completion_manager();
                match &mut COMPLETION_MANAGER {
                    Some(manager) => manager,
                    None => return,
                }
            }
        }
    };
    
    // Register the completion
    completion_manager.register_omni_completion(filetype, completion_fn);
}

/// Add a keyword dictionary
pub fn add_keyword_dictionary(name: &str, words: HashSet<String>) {
    // Get the completion manager
    let completion_manager = unsafe {
        match &mut COMPLETION_MANAGER {
            Some(manager) => manager,
            None => {
                init_completion_manager();
                match &mut COMPLETION_MANAGER {
                    Some(manager) => manager,
                    None => return,
                }
            }
        }
    };
    
    // Add the dictionary
    completion_manager.add_keyword_dictionary(name, words);
}

/// Remove a keyword dictionary
pub fn remove_keyword_dictionary(name: &str) {
    // Get the completion manager
    let completion_manager = unsafe {
        match &mut COMPLETION_MANAGER {
            Some(manager) => manager,
            None => {
                init_completion_manager();
                match &mut COMPLETION_MANAGER {
                    Some(manager) => manager,
                    None => return,
                }
            }
        }
    };
    
    // Remove the dictionary
    completion_manager.remove_keyword_dictionary(name);
}

/// Set completion options
pub fn set_completion_options(options: CompletionOptions) {
    // Get the completion manager
    let completion_manager = unsafe {
        match &mut COMPLETION_MANAGER {
            Some(manager) => manager,
            None => {
                init_completion_manager();
                match &mut COMPLETION_MANAGER {
                    Some(manager) => manager,
                    None => return,
                }
            }
        }
    };
    
    // Set the options
    completion_manager.set_options(options);
}

/// Get completion options
pub fn get_completion_options() -> Option<CompletionOptions> {
    // Get the completion manager
    let completion_manager = unsafe {
        match &COMPLETION_MANAGER {
            Some(manager) => manager,
            None => {
                init_completion_manager();
                match &COMPLETION_MANAGER {
                    Some(manager) => manager,
                    None => return None,
                }
            }
        }
    };
    
    // Get the options
    Some(completion_manager.get_options().clone())
}