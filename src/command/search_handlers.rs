//! Search and replace command handlers
//!
//! This module implements handlers for search and replace commands.

use crate::command::{ExCommand, ExCommandError, ExCommandResult, ExCommandRegistry};
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use crate::editor::Editor;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use crate::cursor::CursorPosition;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::collections::HashMap;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::{self, Write};
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::time::SystemTime;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;

/// Search direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchDirection {
    /// Forward search
    Forward,
    /// Backward search
    Backward,
}

/// Search options
#[derive(Debug, Clone)]
pub struct SearchOptions {
    /// Case sensitivity
    pub case_sensitive: bool,
    /// Whole word search
    pub whole_word: bool,
    /// Regular expression search
    pub regex: bool,
    /// Incremental search
    pub incremental: bool,
    /// Highlight all matches
    pub highlight_all: bool,
    /// Wrap around
    pub wrap_around: bool,
    /// Search direction
    pub direction: SearchDirection,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            whole_word: false,
            regex: false,
            incremental: true,
            highlight_all: true,
            wrap_around: true,
            direction: SearchDirection::Forward,
        }
    }
}

/// Search history entry
#[derive(Debug, Clone)]
pub struct SearchHistoryEntry {
    /// Search pattern
    pub pattern: String,
    /// Search options
    pub options: SearchOptions,
    /// Timestamp
    pub timestamp: SystemTime,
}

/// Search history
#[derive(Debug)]
pub struct SearchHistory {
    /// Search history entries
    pub entries: Vec<SearchHistoryEntry>,
    /// Current position in the history
    pub current: usize,
    /// Maximum history size
    pub max_size: usize,
}

impl SearchHistory {
    /// Create a new search history
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            current: 0,
            max_size,
        }
    }

    /// Add a search pattern to the history
    pub fn add(&mut self, pattern: &str, options: &SearchOptions) {
        // Don't add empty patterns
        if pattern.is_empty() {
            return;
        }

        // Don't add duplicate patterns
        if let Some(entry) = self.entries.iter().find(|e| e.pattern == pattern) {
            // Update the timestamp
            let mut entry = entry.clone();
            entry.timestamp = SystemTime::now();
            
            // Remove the old entry
            self.entries.retain(|e| e.pattern != pattern);
            
            // Add the updated entry
            self.entries.push(entry);
            
            // Update the current position
            self.current = self.entries.len() - 1;
            
            return;
        }

        // Add the new entry
        self.entries.push(SearchHistoryEntry {
            pattern: pattern.to_string(),
            options: options.clone(),
            timestamp: SystemTime::now(),
        });

        // Limit the history size
        if self.entries.len() > self.max_size {
            self.entries.remove(0);
        }

        // Update the current position
        self.current = self.entries.len() - 1;
    }

    /// Get the previous search pattern
    pub fn prev(&mut self) -> Option<&SearchHistoryEntry> {
        if self.entries.is_empty() {
            return None;
        }

        if self.current > 0 {
            self.current -= 1;
        }

        self.entries.get(self.current)
    }

    /// Get the next search pattern
    pub fn next(&mut self) -> Option<&SearchHistoryEntry> {
        if self.entries.is_empty() {
            return None;
        }

        if self.current < self.entries.len() - 1 {
            self.current += 1;
        }

        self.entries.get(self.current)
    }

    /// Get the current search pattern
    pub fn current(&self) -> Option<&SearchHistoryEntry> {
        if self.entries.is_empty() {
            return None;
        }

        self.entries.get(self.current)
    }

    /// Clear the search history
    pub fn clear(&mut self) {
        self.entries.clear();
        self.current = 0;
    }
}

/// Search manager
#[derive(Debug)]
pub struct SearchManager {
    /// Search options
    pub options: SearchOptions,
    /// Last search pattern
    pub last_pattern: Option<String>,
    /// Last replacement pattern
    pub last_replacement: Option<String>,
    /// Search history
    pub history: SearchHistory,
    /// Highlighted matches
    pub highlights: HashMap<usize, Vec<(usize, usize)>>,
}

impl SearchManager {
    /// Create a new search manager
    pub fn new() -> Self {
        Self {
            options: SearchOptions::default(),
            last_pattern: None,
            last_replacement: None,
            history: SearchHistory::new(100),
            highlights: HashMap::new(),
        }
    }

    /// Set the search options
    pub fn set_options(&mut self, options: SearchOptions) {
        self.options = options;
    }

    /// Set the search pattern
    pub fn set_pattern(&mut self, pattern: &str) {
        self.last_pattern = Some(pattern.to_string());
        self.history.add(pattern, &self.options);
    }

    /// Set the replacement pattern
    pub fn set_replacement(&mut self, replacement: &str) {
        self.last_replacement = Some(replacement.to_string());
    }

    /// Get the last search pattern
    pub fn get_pattern(&self) -> Option<&str> {
        self.last_pattern.as_deref()
    }

    /// Get the last replacement pattern
    pub fn get_replacement(&self) -> Option<&str> {
        self.last_replacement.as_deref()
    }

    /// Search for a pattern
    pub fn search(&mut self, editor: &mut Editor, pattern: &str, direction: SearchDirection) -> ExCommandResult<Option<CursorPosition>> {
        // Set the search pattern
        self.set_pattern(pattern);
        
        // Set the search direction
        self.options.direction = direction;
        
        // Search for the pattern
        let result = editor.search(pattern, self.options.case_sensitive, self.options.whole_word);
        
        match result {
            Ok(Some(pos)) => {
                // Add the match to the highlights
                if self.options.highlight_all {
                    self.highlight_matches(editor, pattern)?;
                }
                
                Ok(Some(pos))
            },
            Ok(None) => Ok(None),
            Err(err) => Err(ExCommandError::Other(format!("Search error: {}", err))),
        }
    }

    /// Search for the next match
    pub fn search_next(&mut self, editor: &mut Editor) -> ExCommandResult<Option<CursorPosition>> {
        // Get the last search pattern
        let pattern = match self.get_pattern() {
            Some(pattern) => pattern,
            None => return Err(ExCommandError::Other("No previous search pattern".to_string())),
        };
        
        // Search for the pattern
        self.search(editor, pattern, SearchDirection::Forward)
    }

    /// Search for the previous match
    pub fn search_prev(&mut self, editor: &mut Editor) -> ExCommandResult<Option<CursorPosition>> {
        // Get the last search pattern
        let pattern = match self.get_pattern() {
            Some(pattern) => pattern,
            None => return Err(ExCommandError::Other("No previous search pattern".to_string())),
        };
        
        // Search for the pattern
        self.search(editor, pattern, SearchDirection::Backward)
    }

    /// Replace the current match
    pub fn replace(&mut self, editor: &mut Editor, pattern: &str, replacement: &str, global: bool, confirm: bool, case_insensitive: bool, range: Option<(usize, usize)>) -> ExCommandResult<usize> {
        // Set the search pattern
        self.set_pattern(pattern);
        
        // Set the replacement pattern
        self.set_replacement(replacement);
        
        // Get the current buffer ID
        let buffer_id = match editor.current_buffer_id() {
            Some(id) => id,
            None => return Err(ExCommandError::Other("No current buffer".to_string())),
        };
        
        // Get the buffer
        let buffer = match editor.get_buffer_manager().get_buffer_mut(buffer_id) {
            Ok(buffer) => buffer,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
        };
        
        // Get the range
        let (start_line, end_line) = match range {
            Some(range) => range,
            None => (0, buffer.line_count() - 1),
        };
        
        // Replace the pattern
        let mut count = 0;
        
        for line_idx in start_line..=end_line {
            // Get the line
            let line = match buffer.line(line_idx) {
                Ok(line) => line,
                Err(err) => return Err(ExCommandError::Other(format!("Failed to get line: {}", err))),
            };
            
            // Replace the pattern
            let new_line = if global {
                // Global replacement
                let mut new_line = line.to_string();
                let mut replaced = false;
                
                // Use regex for replacement if enabled
                if self.options.regex || case_insensitive {
                    // Create regex options
                    let regex_options = regex::RegexBuilder::new(pattern)
                        .case_insensitive(case_insensitive || !self.options.case_sensitive)
                        .build();
                    
                    match regex_options {
                        Ok(re) => {
                            // If confirmation is required, we need to handle each match individually
                            if confirm {
                                let mut result = String::new();
                                let mut last_end = 0;
                                
                                // Find all matches
                                for mat in re.find_iter(&new_line) {
                                    // Add text before the match
                                    result.push_str(&new_line[last_end..mat.start()]);
                                    
                                    // Get the matched text
                                    let matched = &new_line[mat.start()..mat.end()];
                                    
                                    // Prompt for confirmation
                                    println!("Replace '{}' with '{}'? (y/n/a/q)", matched, replacement);
                                    let mut input = String::new();
                                    std::io::stdin().read_line(&mut input).unwrap();
                                    
                                    match input.trim().to_lowercase().as_str() {
                                        "y" | "yes" => {
                                            // Replace this match
                                            result.push_str(&re.replace(matched, replacement));
                                            count += 1;
                                            replaced = true;
                                        },
                                        "n" | "no" => {
                                            // Don't replace this match
                                            result.push_str(matched);
                                        },
                                        "a" | "all" => {
                                            // Replace all remaining matches without confirmation
                                            result.push_str(&re.replace(matched, replacement));
                                            count += 1;
                                            replaced = true;
                                            
                                            // Replace all remaining matches
                                            let remaining = &new_line[mat.end()..];
                                            let remaining_replaced = re.replace_all(remaining, replacement);
                                            result.push_str(&remaining_replaced);
                                            
                                            // Count the remaining replacements
                                            count += re.find_iter(remaining).count();
                                            
                                            // Set the last end to the end of the string
                                            last_end = new_line.len();
                                            break;
                                        },
                                        "q" | "quit" => {
                                            // Quit replacing
                                            result.push_str(matched);
                                            result.push_str(&new_line[mat.end()..]);
                                            last_end = new_line.len();
                                            break;
                                        },
                                        _ => {
                                            // Invalid input, don't replace
                                            result.push_str(matched);
                                        }
                                    }
                                    
                                    last_end = mat.end();
                                }
                                
                                // Add any remaining text
                                if last_end < new_line.len() {
                                    result.push_str(&new_line[last_end..]);
                                }
                                
                                new_line = result;
                            } else {
                                // Replace all matches without confirmation
                                let count_before = count;
                                
                                // Use regex captures for replacement
                                let result = re.replace_all(&new_line, |caps: &regex::Captures| {
                                    // Process the replacement string to handle capture groups
                                    let mut result = String::new();
                                    let mut i = 0;
                                    
                                    while i < replacement.len() {
                                        if replacement[i..].starts_with('\\') && i + 1 < replacement.len() {
                                            let c = replacement.chars().nth(i + 1).unwrap();
                                            
                                            if c.is_digit(10) {
                                                // Replace \n with the nth capture group
                                                let group_idx = c.to_digit(10).unwrap() as usize;
                                                
                                                if let Some(group) = caps.get(group_idx) {
                                                    result.push_str(group.as_str());
                                                }
                                                
                                                i += 2;
                                            } else if c == '\\' {
                                                // Escaped backslash
                                                result.push('\\');
                                                i += 2;
                                            } else {
                                                // Other escaped character
                                                result.push(c);
                                                i += 2;
                                            }
                                        } else {
                                            // Regular character
                                            result.push(replacement.chars().nth(i).unwrap());
                                            i += 1;
                                        }
                                    }
                                    
                                    result
                                });
                                
                                // Count the number of replacements
                                count += re.find_iter(&new_line).count();
                                
                                // Check if any replacements were made
                                if count > count_before {
                                    replaced = true;
                                    new_line = result.to_string();
                                }
                            }
                        },
                        Err(err) => return Err(ExCommandError::Other(format!("Invalid regex pattern: {}", err))),
                    }
                } else {
                    // Simple string replacement
                    if confirm {
                        let mut result = String::new();
                        let mut last_end = 0;
                        
                        // Find all matches
                        let mut start = 0;
                        while let Some(idx) = new_line[start..].find(pattern) {
                            let match_start = start + idx;
                            let match_end = match_start + pattern.len();
                            
                            // Add text before the match
                            result.push_str(&new_line[last_end..match_start]);
                            
                            // Get the matched text
                            let matched = &new_line[match_start..match_end];
                            
                            // Prompt for confirmation
                            println!("Replace '{}' with '{}'? (y/n/a/q)", matched, replacement);
                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input).unwrap();
                            
                            match input.trim().to_lowercase().as_str() {
                                "y" | "yes" => {
                                    // Replace this match
                                    result.push_str(replacement);
                                    count += 1;
                                    replaced = true;
                                },
                                "n" | "no" => {
                                    // Don't replace this match
                                    result.push_str(matched);
                                },
                                "a" | "all" => {
                                    // Replace all remaining matches without confirmation
                                    result.push_str(replacement);
                                    count += 1;
                                    replaced = true;
                                    
                                    // Replace all remaining matches
                                    let remaining = &new_line[match_end..];
                                    let remaining_replaced = remaining.replace(pattern, replacement);
                                    result.push_str(&remaining_replaced);
                                    
                                    // Count the remaining replacements
                                    count += remaining.matches(pattern).count();
                                    
                                    // Set the last end to the end of the string
                                    last_end = new_line.len();
                                    break;
                                },
                                "q" | "quit" => {
                                    // Quit replacing
                                    result.push_str(matched);
                                    result.push_str(&new_line[match_end..]);
                                    last_end = new_line.len();
                                    break;
                                },
                                _ => {
                                    // Invalid input, don't replace
                                    result.push_str(matched);
                                }
                            }
                            
                            last_end = match_end;
                            start = match_end;
                        }
                        
                        // Add any remaining text
                        if last_end < new_line.len() {
                            result.push_str(&new_line[last_end..]);
                        }
                        
                        new_line = result;
                    } else {
                        // Simple string replacement without confirmation
                        let count_before = count;
                        count += new_line.matches(pattern).count();
                        
                        // Check if any replacements were made
                        if count > count_before {
                            replaced = true;
                            new_line = new_line.replace(pattern, replacement);
                        }
                    }
                }
                
                if replaced {
                    new_line
                } else {
                    line.to_string()
                }
            } else {
                // Replace first occurrence only
                let mut new_line = line.to_string();
                let mut replaced = false;
                
                // Use regex for replacement if enabled
                if self.options.regex || case_insensitive {
                    // Create regex options
                    let regex_options = regex::RegexBuilder::new(pattern)
                        .case_insensitive(case_insensitive || !self.options.case_sensitive)
                        .build();
                    
                    match regex_options {
                        Ok(re) => {
                            if let Some(mat) = re.find(&new_line) {
                                // If confirmation is required, prompt for confirmation
                                if confirm {
                                    // Get the matched text
                                    let matched = &new_line[mat.start()..mat.end()];
                                    
                                    // Prompt for confirmation
                                    println!("Replace '{}' with '{}'? (y/n)", matched, replacement);
                                    let mut input = String::new();
                                    std::io::stdin().read_line(&mut input).unwrap();
                                    
                                    if input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes" {
                                        // Replace the match
                                        count += 1;
                                        replaced = true;
                                        
                                        // Process the replacement string to handle capture groups
                                        let caps = re.captures(&new_line).unwrap();
                                        let mut result = String::new();
                                        let mut i = 0;
                                        
                                        while i < replacement.len() {
                                            if replacement[i..].starts_with('\\') && i + 1 < replacement.len() {
                                                let c = replacement.chars().nth(i + 1).unwrap();
                                                
                                                if c.is_digit(10) {
                                                    // Replace \n with the nth capture group
                                                    let group_idx = c.to_digit(10).unwrap() as usize;
                                                    
                                                    if let Some(group) = caps.get(group_idx) {
                                                        result.push_str(group.as_str());
                                                    }
                                                    
                                                    i += 2;
                                                } else if c == '\\' {
                                                    // Escaped backslash
                                                    result.push('\\');
                                                    i += 2;
                                                } else {
                                                    // Other escaped character
                                                    result.push(c);
                                                    i += 2;
                                                }
                                            } else {
                                                // Regular character
                                                result.push(replacement.chars().nth(i).unwrap());
                                                i += 1;
                                            }
                                        }
                                        
                                        // Replace the first match
                                        new_line = format!("{}{}{}", &new_line[..mat.start()], result, &new_line[mat.end()..]);
                                    }
                                } else {
                                    count += 1;
                                    replaced = true;
                                    
                                    // Process the replacement string to handle capture groups
                                    let caps = re.captures(&new_line).unwrap();
                                    let mut result = String::new();
                                    let mut i = 0;
                                    
                                    while i < replacement.len() {
                                        if replacement[i..].starts_with('\\') && i + 1 < replacement.len() {
                                            let c = replacement.chars().nth(i + 1).unwrap();
                                            
                                            if c.is_digit(10) {
                                                // Replace \n with the nth capture group
                                                let group_idx = c.to_digit(10).unwrap() as usize;
                                                
                                                if let Some(group) = caps.get(group_idx) {
                                                    result.push_str(group.as_str());
                                                }
                                                
                                                i += 2;
                                            } else if c == '\\' {
                                                // Escaped backslash
                                                result.push('\\');
                                                i += 2;
                                            } else {
                                                // Other escaped character
                                                result.push(c);
                                                i += 2;
                                            }
                                        } else {
                                            // Regular character
                                            result.push(replacement.chars().nth(i).unwrap());
                                            i += 1;
                                        }
                                    }
                                    
                                    // Replace the first match
                                    new_line = format!("{}{}{}", &new_line[..mat.start()], result, &new_line[mat.end()..]);
                                }
                            }
                        },
                        Err(err) => return Err(ExCommandError::Other(format!("Invalid regex pattern: {}", err))),
                    }
                } else {
                    // Simple string replacement
                    if let Some(idx) = new_line.find(pattern) {
                        // If confirmation is required, prompt for confirmation
                        if confirm {
                            // Get the matched text
                            let matched = &new_line[idx..idx + pattern.len()];
                            
                            // Prompt for confirmation
                            println!("Replace '{}' with '{}'? (y/n)", matched, replacement);
                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input).unwrap();
                            
                            if input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes" {
                                // Replace the match
                                count += 1;
                                replaced = true;
                                
                                // Replace the first match
                                new_line = format!("{}{}{}", &new_line[..idx], replacement, &new_line[idx + pattern.len()..]);
                            }
                        } else {
                            count += 1;
                            replaced = true;
                            
                            // Replace the first match
                            new_line = format!("{}{}{}", &new_line[..idx], replacement, &new_line[idx + pattern.len()..]);
                        }
                    }
                }
                
                if replaced {
                    new_line
                } else {
                    line.to_string()
                }
            };
            
            // Update the line
            if new_line != line {
                if let Err(err) = buffer.set_line(line_idx, &new_line) {
                    return Err(ExCommandError::Other(format!("Failed to update line: {}", err)));
                }
            }
        }
        
        Ok(count)
    }

    /// Highlight all matches
    fn highlight_matches(&mut self, editor: &mut Editor, pattern: &str) -> ExCommandResult<()> {
        // Get the current buffer ID
        let buffer_id = match editor.current_buffer_id() {
            Some(id) => id,
            None => return Err(ExCommandError::Other("No current buffer".to_string())),
        };
        
        // Get the buffer
        let buffer = match editor.get_buffer_manager().get_buffer(buffer_id) {
            Ok(buffer) => buffer,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
        };
        
        // Clear the highlights for this buffer
        self.highlights.remove(&buffer_id);
        
        // Find all matches
        let mut matches = Vec::new();
        
        for line_idx in 0..buffer.line_count() {
            // Get the line
            let line = match buffer.line(line_idx) {
                Ok(line) => line,
                Err(err) => return Err(ExCommandError::Other(format!("Failed to get line: {}", err))),
            };
            
            // Find all matches in the line
            if self.options.regex {
                match regex::RegexBuilder::new(pattern)
                    .case_insensitive(!self.options.case_sensitive)
                    .build() {
                    Ok(re) => {
                        for mat in re.find_iter(line) {
                            matches.push((line_idx, mat.start(), mat.end()));
                        }
                    },
                    Err(err) => return Err(ExCommandError::Other(format!("Invalid regex pattern: {}", err))),
                }
            } else {
                let mut start = 0;
                
                while let Some(idx) = line[start..].find(pattern) {
                    let match_start = start + idx;
                    let match_end = match_start + pattern.len();
                    
                    matches.push((line_idx, match_start, match_end));
                    
                    start = match_end;
                }
            }
        }
        
        // Add the matches to the highlights
        self.highlights.insert(buffer_id, matches.iter().map(|(line, start, _)| (*line, *start)).collect());
        
        Ok(())
    }

    /// Clear all highlights
    pub fn clear_highlights(&mut self) {
        self.highlights.clear();
    }
}

// Global search manager
pub static mut SEARCH_MANAGER: Option<SearchManager> = None;

/// Initialize the search manager
pub fn init_search_manager() {
    unsafe {
        if SEARCH_MANAGER.is_none() {
            SEARCH_MANAGER = Some(SearchManager::new());
        }
    }
}

/// Register search and replace command handlers
pub fn register_search_handlers(registry: &mut ExCommandRegistry) {
    // Initialize the search manager
    init_search_manager();
    
    // Register search commands
    registry.register("search", handle_search);
    registry.register("/", handle_search);
    registry.register("?", handle_search_backward);
    registry.register("n", handle_search_next);
    registry.register("N", handle_search_prev);
    
    // Register replace commands
    registry.register("substitute", handle_substitute);
    registry.register("s", handle_substitute);
    registry.register("global", handle_global);
    registry.register("g", handle_global);
    registry.register("vglobal", handle_vglobal);
    registry.register("v", handle_vglobal);
    
    // Register search option commands
    registry.register("set", crate::command::set_handlers::handle_set);
    
    // Register nohlsearch commands
    crate::command::nohlsearch_handlers::register_nohlsearch_handlers(registry);
}

/// Handle the :search command
fn handle_search(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the search manager
    let search_manager = unsafe {
        match &mut SEARCH_MANAGER {
            Some(manager) => manager,
            None => {
                init_search_manager();
                match &mut SEARCH_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize search manager".to_string())),
                }
            }
        }
    };
    
    // Get the search pattern from the command arguments
    let pattern = cmd.args_str();
    
    if pattern.is_empty() {
        // If no pattern is provided, use the last search pattern
        match search_manager.get_pattern() {
            Some(pattern) => {
                // Search for the pattern
                match search_manager.search(editor, pattern, SearchDirection::Forward)? {
                    Some(pos) => {
                        // Set the cursor position
                        if let Err(err) = editor.get_cursor_manager_mut().set_position(pos) {
                            return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
                        }
                        
                        println!("Pattern found: {}", pattern);
                        Ok(())
                    },
                    None => {
                        println!("Pattern not found: {}", pattern);
                        Ok(())
                    }
                }
            },
            None => Err(ExCommandError::MissingArgument("Search pattern required".to_string())),
        }
    } else {
        // Search for the pattern
        match search_manager.search(editor, pattern, SearchDirection::Forward)? {
            Some(pos) => {
                // Set the cursor position
                if let Err(err) = editor.get_cursor_manager_mut().set_position(pos) {
                    return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
                }
                
                println!("Pattern found: {}", pattern);
                Ok(())
            },
            None => {
                println!("Pattern not found: {}", pattern);
                Ok(())
            }
        }
    }
}

/// Handle the :? command (search backward)
fn handle_search_backward(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the search manager
    let search_manager = unsafe {
        match &mut SEARCH_MANAGER {
            Some(manager) => manager,
            None => {
                init_search_manager();
                match &mut SEARCH_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize search manager".to_string())),
                }
            }
        }
    };
    
    // Get the search pattern from the command arguments
    let pattern = cmd.args_str();
    
    if pattern.is_empty() {
        // If no pattern is provided, use the last search pattern
        match search_manager.get_pattern() {
            Some(pattern) => {
                // Search for the pattern
                match search_manager.search(editor, pattern, SearchDirection::Backward)? {
                    Some(pos) => {
                        // Set the cursor position
                        if let Err(err) = editor.get_cursor_manager_mut().set_position(pos) {
                            return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
                        }
                        
                        println!("Pattern found: {}", pattern);
                        Ok(())
                    },
                    None => {
                        println!("Pattern not found: {}", pattern);
                        Ok(())
                    }
                }
            },
            None => Err(ExCommandError::MissingArgument("Search pattern required".to_string())),
        }
    } else {
        // Search for the pattern
        match search_manager.search(editor, pattern, SearchDirection::Backward)? {
            Some(pos) => {
                // Set the cursor position
                if let Err(err) = editor.get_cursor_manager_mut().set_position(pos) {
                    return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
                }
                
                println!("Pattern found: {}", pattern);
                Ok(())
            },
            None => {
                println!("Pattern not found: {}", pattern);
                Ok(())
            }
        }
    }
}

/// Handle the :n command (search next)
fn handle_search_next(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the search manager
    let search_manager = unsafe {
        match &mut SEARCH_MANAGER {
            Some(manager) => manager,
            None => {
                init_search_manager();
                match &mut SEARCH_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize search manager".to_string())),
                }
            }
        }
    };
    
    // Search for the next match
    match search_manager.search_next(editor)? {
        Some(pos) => {
            // Set the cursor position
            if let Err(err) = editor.get_cursor_manager_mut().set_position(pos) {
                return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
            }
            
            println!("Pattern found: {}", search_manager.get_pattern().unwrap_or(""));
            Ok(())
        },
        None => {
            println!("Pattern not found: {}", search_manager.get_pattern().unwrap_or(""));
            Ok(())
        }
    }
}

/// Handle the :N command (search previous)
fn handle_search_prev(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the search manager
    let search_manager = unsafe {
        match &mut SEARCH_MANAGER {
            Some(manager) => manager,
            None => {
                init_search_manager();
                match &mut SEARCH_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize search manager".to_string())),
                }
            }
        }
    };
    
    // Search for the previous match
    match search_manager.search_prev(editor)? {
        Some(pos) => {
            // Set the cursor position
            if let Err(err) = editor.get_cursor_manager_mut().set_position(pos) {
                return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
            }
            
            println!("Pattern found: {}", search_manager.get_pattern().unwrap_or(""));
            Ok(())
        },
        None => {
            println!("Pattern not found: {}", search_manager.get_pattern().unwrap_or(""));
            Ok(())
        }
    }
}

/// Handle the :substitute command
fn handle_substitute(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the search manager
    let search_manager = unsafe {
        match &mut SEARCH_MANAGER {
            Some(manager) => manager,
            None => {
                init_search_manager();
                match &mut SEARCH_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize search manager".to_string())),
                }
            }
        }
    };
    
    // Parse the substitute command
    // Format: :[range]s[ubstitute]/{pattern}/{replacement}/[flags]
    let args = cmd.args_str();
    
    if args.is_empty() {
        // If no arguments are provided, use the last search pattern and replacement
        match (search_manager.get_pattern(), search_manager.get_replacement()) {
            (Some(pattern), Some(replacement)) => {
                // Get the range from the command
                let range = if let Some(start) = cmd.range.start {
                    let start_line = match start {
                        crate::command::RangeSpec::CurrentLine => editor.cursor_position().line,
                        crate::command::RangeSpec::LastLine => {
                            let buffer_id = match editor.current_buffer_id() {
                                Some(id) => id,
                                None => return Err(ExCommandError::Other("No current buffer".to_string())),
                            };
                            
                            let buffer = match editor.get_buffer_manager().get_buffer(buffer_id) {
                                Ok(buffer) => buffer,
                                Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
                            };
                            
                            buffer.line_count() - 1
                        },
                        crate::command::RangeSpec::LineNumber(n) => n - 1, // Convert to 0-based
                        crate::command::RangeSpec::Mark(mark) => {
                            match editor.get_mark_position(mark) {
                                Some(pos) => pos.line,
                                None => return Err(ExCommandError::InvalidRange(format!("Mark '{}' not set", mark))),
                            }
                        },
                        crate::command::RangeSpec::Search(pattern) => {
                            // Search for the pattern
                            match editor.search(pattern.as_str(), true, false) {
                                Ok(Some(pos)) => pos.line,
                                Ok(None) => return Err(ExCommandError::InvalidRange(format!("Pattern not found: {}", pattern))),
                                Err(err) => return Err(ExCommandError::InvalidRange(format!("Search error: {}", err))),
                            }
                        },
                        crate::command::RangeSpec::Offset(offset) => {
                            let current_line = editor.cursor_position().line;
                            if offset >= 0 {
                                current_line + offset as usize
                            } else {
                                current_line.saturating_sub((-offset) as usize)
                            }
                        },
                    };
                    
                    let end_line = if let Some(end) = cmd.range.end {
                        match end {
                            crate::command::RangeSpec::CurrentLine => editor.cursor_position().line,
                            crate::command::RangeSpec::LastLine => {
                                let buffer_id = match editor.current_buffer_id() {
                                    Some(id) => id,
                                    None => return Err(ExCommandError::Other("No current buffer".to_string())),
                                };
                                
                                let buffer = match editor.get_buffer_manager().get_buffer(buffer_id) {
                                    Ok(buffer) => buffer,
                                    Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
                                };
                                
                                buffer.line_count() - 1
                            },
                            crate::command::RangeSpec::LineNumber(n) => n - 1, // Convert to 0-based
                            crate::command::RangeSpec::Mark(mark) => {
                                match editor.get_mark_position(mark) {
                                    Some(pos) => pos.line,
                                    None => return Err(ExCommandError::InvalidRange(format!("Mark '{}' not set", mark))),
                                }
                            },
                            crate::command::RangeSpec::Search(pattern) => {
                                // Search for the pattern
                                match editor.search(pattern.as_str(), true, false) {
                                    Ok(Some(pos)) => pos.line,
                                    Ok(None) => return Err(ExCommandError::InvalidRange(format!("Pattern not found: {}", pattern))),
                                    Err(err) => return Err(ExCommandError::InvalidRange(format!("Search error: {}", err))),
                                }
                            },
                            crate::command::RangeSpec::Offset(offset) => {
                                let current_line = editor.cursor_position().line;
                                if offset >= 0 {
                                    current_line + offset as usize
                                } else {
                                    current_line.saturating_sub((-offset) as usize)
                                }
                            },
                        }
                    } else {
                        start_line
                    };
                    
                    Some((start_line, end_line))
                } else {
                    None
                };
                
                // Replace the pattern
                let count = search_manager.replace(editor, pattern, replacement, true, false, false, range)?;
                
                println!("{} substitutions on {} lines", count, if let Some((start, end)) = range {
                    end - start + 1
                } else {
                    let buffer_id = match editor.current_buffer_id() {
                        Some(id) => id,
                        None => return Err(ExCommandError::Other("No current buffer".to_string())),
                    };
                    
                    let buffer = match editor.get_buffer_manager().get_buffer(buffer_id) {
                        Ok(buffer) => buffer,
                        Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
                    };
                    
                    buffer.line_count()
                });
                
                Ok(())
            },
            (None, _) => Err(ExCommandError::Other("No previous search pattern".to_string())),
            (_, None) => Err(ExCommandError::Other("No previous replacement pattern".to_string())),
        }
    } else {
        // Parse the substitute command
        // We need to handle the case where the pattern or replacement might contain slashes
        // For example: s/\/usr\/local/\/opt\/local/g
        
        // Find the first delimiter
        let first_char = args.chars().next().unwrap_or('/');
        
        // Split the command using the delimiter, but handle escaped delimiters
        let mut parts = Vec::new();
        let mut current_part = String::new();
        let mut escaped = false;
        let mut in_part = false;
        
        for c in args.chars() {
            if escaped {
                // Add the escaped character
                current_part.push(c);
                escaped = false;
            } else if c == '\\' {
                // Escape the next character
                escaped = true;
                current_part.push(c);
            } else if c == first_char {
                if !in_part {
                    // Start of the first part
                    in_part = true;
                } else {
                    // End of a part
                    parts.push(current_part);
                    current_part = String::new();
                }
            } else {
                // Add the character to the current part
                current_part.push(c);
            }
        }
        
        // Add the last part if there is one
        if !current_part.is_empty() {
            parts.push(current_part);
        }
        
        if parts.len() < 2 {
            return Err(ExCommandError::InvalidArgument(format!("Invalid substitute command: {}", args)));
        }
        
        let pattern = &parts[0];
        let replacement = &parts[1];
        let flags = if parts.len() > 2 { &parts[2] } else { "" };
        
        // Parse the flags
        let global = flags.contains('g');
        let confirm = flags.contains('c');
        let case_insensitive = flags.contains('i');
        
        // Get the range from the command
        let range = if let Some(start) = cmd.range.start {
            let start_line = match start {
                crate::command::RangeSpec::CurrentLine => editor.cursor_position().line,
                crate::command::RangeSpec::LastLine => {
                    let buffer_id = match editor.current_buffer_id() {
                        Some(id) => id,
                        None => return Err(ExCommandError::Other("No current buffer".to_string())),
                    };
                    
                    let buffer = match editor.get_buffer_manager().get_buffer(buffer_id) {
                        Ok(buffer) => buffer,
                        Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
                    };
                    
                    buffer.line_count() - 1
                },
                crate::command::RangeSpec::LineNumber(n) => n - 1, // Convert to 0-based
                crate::command::RangeSpec::Mark(mark) => {
                    match editor.get_mark_position(mark) {
                        Some(pos) => pos.line,
                        None => return Err(ExCommandError::InvalidRange(format!("Mark '{}' not set", mark))),
                    }
                },
                crate::command::RangeSpec::Search(pattern) => {
                    // Search for the pattern
                    match editor.search(pattern.as_str(), true, false) {
                        Ok(Some(pos)) => pos.line,
                        Ok(None) => return Err(ExCommandError::InvalidRange(format!("Pattern not found: {}", pattern))),
                        Err(err) => return Err(ExCommandError::InvalidRange(format!("Search error: {}", err))),
                    }
                },
                crate::command::RangeSpec::Offset(offset) => {
                    let current_line = editor.cursor_position().line;
                    if offset >= 0 {
                        current_line + offset as usize
                    } else {
                        current_line.saturating_sub((-offset) as usize)
                    }
                },
            };
            
            let end_line = if let Some(end) = cmd.range.end {
                match end {
                    crate::command::RangeSpec::CurrentLine => editor.cursor_position().line,
                    crate::command::RangeSpec::LastLine => {
                        let buffer_id = match editor.current_buffer_id() {
                            Some(id) => id,
                            None => return Err(ExCommandError::Other("No current buffer".to_string())),
                        };
                        
                        let buffer = match editor.get_buffer_manager().get_buffer(buffer_id) {
                            Ok(buffer) => buffer,
                            Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
                        };
                        
                        buffer.line_count() - 1
                    },
                    crate::command::RangeSpec::LineNumber(n) => n - 1, // Convert to 0-based
                    crate::command::RangeSpec::Mark(mark) => {
                        match editor.get_mark_position(mark) {
                            Some(pos) => pos.line,
                            None => return Err(ExCommandError::InvalidRange(format!("Mark '{}' not set", mark))),
                        }
                    },
                    crate::command::RangeSpec::Search(pattern) => {
                        // Search for the pattern
                        match editor.search(pattern.as_str(), true, false) {
                            Ok(Some(pos)) => pos.line,
                            Ok(None) => return Err(ExCommandError::InvalidRange(format!("Pattern not found: {}", pattern))),
                            Err(err) => return Err(ExCommandError::InvalidRange(format!("Search error: {}", err))),
                        }
                    },
                    crate::command::RangeSpec::Offset(offset) => {
                        let current_line = editor.cursor_position().line;
                        if offset >= 0 {
                            current_line + offset as usize
                        } else {
                            current_line.saturating_sub((-offset) as usize)
                        }
                    },
                }
            } else {
                start_line
            };
            
            Some((start_line, end_line))
        } else {
            None
        };
        
        // Replace the pattern
        let count = search_manager.replace(editor, pattern, replacement, global, confirm, case_insensitive, range)?;
        
        println!("{} substitutions on {} lines", count, if let Some((start, end)) = range {
            end - start + 1
        } else {
            let buffer_id = match editor.current_buffer_id() {
                Some(id) => id,
                None => return Err(ExCommandError::Other("No current buffer".to_string())),
            };
            
            let buffer = match editor.get_buffer_manager().get_buffer(buffer_id) {
                Ok(buffer) => buffer,
                Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
            };
            
            buffer.line_count()
        });
        
        Ok(())
    }
}

/// Handle the :global command
fn handle_global(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Parse the global command
    // Format: :[range]g[lobal]/{pattern}/{command}
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Global pattern and command required".to_string()));
    }
    
    // Parse the global command
    // We need to handle the case where the pattern or command might contain the delimiter
    // For example: g/\/usr\/local/d
    
    // Find the first delimiter
    let first_char = args.chars().next().unwrap_or('/');
    
    // Split the command using the delimiter, but handle escaped delimiters
    let mut parts = Vec::new();
    let mut current_part = String::new();
    let mut escaped = false;
    let mut in_part = false;
    
    for c in args.chars() {
        if escaped {
            // Add the escaped character
            current_part.push(c);
            escaped = false;
        } else if c == '\\' {
            // Escape the next character
            escaped = true;
            current_part.push(c);
        } else if c == first_char {
            if !in_part {
                // Start of the first part
                in_part = true;
            } else {
                // End of a part
                parts.push(current_part);
                current_part = String::new();
            }
        } else {
            // Add the character to the current part
            current_part.push(c);
        }
    }
    
    // Add the last part if there is one
    if !current_part.is_empty() {
        parts.push(current_part);
    }
    
    if parts.len() < 2 {
        return Err(ExCommandError::InvalidArgument(format!("Invalid global command: {}", args)));
    }
    
    let pattern = &parts[0];
    let command = &parts[1];
    
    // Get the range from the command
    let range = if let Some(start) = cmd.range.start {
        let start_line = match start {
            crate::command::RangeSpec::CurrentLine => editor.cursor_position().line,
            crate::command::RangeSpec::LastLine => {
                let buffer_id = match editor.current_buffer_id() {
                    Some(id) => id,
                    None => return Err(ExCommandError::Other("No current buffer".to_string())),
                };
                
                let buffer = match editor.get_buffer_manager().get_buffer(buffer_id) {
                    Ok(buffer) => buffer,
                    Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
                };
                
                buffer.line_count() - 1
            },
            crate::command::RangeSpec::LineNumber(n) => n - 1, // Convert to 0-based
            crate::command::RangeSpec::Mark(mark) => {
                match editor.get_mark_position(mark) {
                    Some(pos) => pos.line,
                    None => return Err(ExCommandError::InvalidRange(format!("Mark '{}' not set", mark))),
                }
            },
            crate::command::RangeSpec::Search(pattern) => {
                // Search for the pattern
                match editor.search(pattern.as_str(), true, false) {
                    Ok(Some(pos)) => pos.line,
                    Ok(None) => return Err(ExCommandError::InvalidRange(format!("Pattern not found: {}", pattern))),
                    Err(err) => return Err(ExCommandError::InvalidRange(format!("Search error: {}", err))),
                }
            },
            crate::command::RangeSpec::Offset(offset) => {
                let current_line = editor.cursor_position().line;
                if offset >= 0 {
                    current_line + offset as usize
                } else {
                    current_line.saturating_sub((-offset) as usize)
                }
            },
        };
        
        let end_line = if let Some(end) = cmd.range.end {
            match end {
                crate::command::RangeSpec::CurrentLine => editor.cursor_position().line,
                crate::command::RangeSpec::LastLine => {
                    let buffer_id = match editor.current_buffer_id() {
                        Some(id) => id,
                        None => return Err(ExCommandError::Other("No current buffer".to_string())),
                    };
                    
                    let buffer = match editor.get_buffer_manager().get_buffer(buffer_id) {
                        Ok(buffer) => buffer,
                        Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
                    };
                    
                    buffer.line_count() - 1
                },
                crate::command::RangeSpec::LineNumber(n) => n - 1, // Convert to 0-based
                crate::command::RangeSpec::Mark(mark) => {
                    match editor.get_mark_position(mark) {
                        Some(pos) => pos.line,
                        None => return Err(ExCommandError::InvalidRange(format!("Mark '{}' not set", mark))),
                    }
                },
                crate::command::RangeSpec::Search(pattern) => {
                    // Search for the pattern
                    match editor.search(pattern.as_str(), true, false) {
                        Ok(Some(pos)) => pos.line,
                        Ok(None) => return Err(ExCommandError::InvalidRange(format!("Pattern not found: {}", pattern))),
                        Err(err) => return Err(ExCommandError::InvalidRange(format!("Search error: {}", err))),
                    }
                },
                crate::command::RangeSpec::Offset(offset) => {
                    let current_line = editor.cursor_position().line;
                    if offset >= 0 {
                        current_line + offset as usize
                    } else {
                        current_line.saturating_sub((-offset) as usize)
                    }
                },
            }
        } else {
            start_line
        };
        
        (start_line, end_line)
    } else {
        // If no range is provided, use the entire buffer
        let buffer_id = match editor.current_buffer_id() {
            Some(id) => id,
            None => return Err(ExCommandError::Other("No current buffer".to_string())),
        };
        
        let buffer = match editor.get_buffer_manager().get_buffer(buffer_id) {
            Ok(buffer) => buffer,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
        };
        
        (0, buffer.line_count() - 1)
    };
    
    // Get the buffer
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::Other("No current buffer".to_string())),
    };
    
    let buffer = match editor.get_buffer_manager().get_buffer(buffer_id) {
        Ok(buffer) => buffer,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
    };
    
    // Create a command parser
    let parser = crate::command::ExCommandParser::new();
    
    // Create a command registry
    let mut registry = crate::command::ExCommandRegistry::new();
    crate::command::handlers::register_handlers(&mut registry, None);
    
    // Find all lines matching the pattern
    let mut matching_lines = Vec::new();
    
    for line_idx in range.0..=range.1 {
        // Get the line
        let line = match buffer.line(line_idx) {
            Ok(line) => line,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to get line: {}", err))),
        };
        
        // Check if the line matches the pattern
        if line.contains(pattern) {
            matching_lines.push(line_idx);
        }
    }
    
    // Execute the command on each matching line
    for line_idx in matching_lines {
        // Set the cursor position to the matching line
        if let Err(err) = editor.get_cursor_manager_mut().set_position(CursorPosition::new(line_idx, 0)) {
            return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
        }
        
        // Parse and execute the command
        match parser.parse(command) {
            Ok(ex_cmd) => {
                if let Err(err) = registry.execute(&ex_cmd) {
                    return Err(ExCommandError::Other(format!("Failed to execute command on line {}: {}", line_idx + 1, err)));
                }
            },
            Err(err) => {
                return Err(ExCommandError::Other(format!("Failed to parse command: {}", err)));
            }
        }
    }
    
    println!("Global command executed on {} matching lines", matching_lines.len());
    Ok(())
}

/// Handle the :vglobal command
fn handle_vglobal(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Parse the vglobal command
    // Format: :[range]v[global]/{pattern}/{command}
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Vglobal pattern and command required".to_string()));
    }
    
    // Parse the vglobal command
    // We need to handle the case where the pattern or command might contain the delimiter
    // For example: v/\/usr\/local/d
    
    // Find the first delimiter
    let first_char = args.chars().next().unwrap_or('/');
    
    // Split the command using the delimiter, but handle escaped delimiters
    let mut parts = Vec::new();
    let mut current_part = String::new();
    let mut escaped = false;
    let mut in_part = false;
    
    for c in args.chars() {
        if escaped {
            // Add the escaped character
            current_part.push(c);
            escaped = false;
        } else if c == '\\' {
            // Escape the next character
            escaped = true;
            current_part.push(c);
        } else if c == first_char {
            if !in_part {
                // Start of the first part
                in_part = true;
            } else {
                // End of a part
                parts.push(current_part);
                current_part = String::new();
            }
        } else {
            // Add the character to the current part
            current_part.push(c);
        }
    }
    
    // Add the last part if there is one
    if !current_part.is_empty() {
        parts.push(current_part);
    }
    
    if parts.len() < 2 {
        return Err(ExCommandError::InvalidArgument(format!("Invalid vglobal command: {}", args)));
    }
    
    let pattern = &parts[0];
    let command = &parts[1];
    
    // Get the range from the command
    let range = if let Some(start) = cmd.range.start {
        let start_line = match start {
            crate::command::RangeSpec::CurrentLine => editor.cursor_position().line,
            crate::command::RangeSpec::LastLine => {
                let buffer_id = match editor.current_buffer_id() {
                    Some(id) => id,
                    None => return Err(ExCommandError::Other("No current buffer".to_string())),
                };
                
                let buffer = match editor.get_buffer_manager().get_buffer(buffer_id) {
                    Ok(buffer) => buffer,
                    Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
                };
                
                buffer.line_count() - 1
            },
            crate::command::RangeSpec::LineNumber(n) => n - 1, // Convert to 0-based
            crate::command::RangeSpec::Mark(mark) => {
                match editor.get_mark_position(mark) {
                    Some(pos) => pos.line,
                    None => return Err(ExCommandError::InvalidRange(format!("Mark '{}' not set", mark))),
                }
            },
            crate::command::RangeSpec::Search(pattern) => {
                // Search for the pattern
                match editor.search(pattern.as_str(), true, false) {
                    Ok(Some(pos)) => pos.line,
                    Ok(None) => return Err(ExCommandError::InvalidRange(format!("Pattern not found: {}", pattern))),
                    Err(err) => return Err(ExCommandError::InvalidRange(format!("Search error: {}", err))),
                }
            },
            crate::command::RangeSpec::Offset(offset) => {
                let current_line = editor.cursor_position().line;
                if offset >= 0 {
                    current_line + offset as usize
                } else {
                    current_line.saturating_sub((-offset) as usize)
                }
            },
        };
        
        let end_line = if let Some(end) = cmd.range.end {
            match end {
                crate::command::RangeSpec::CurrentLine => editor.cursor_position().line,
                crate::command::RangeSpec::LastLine => {
                    let buffer_id = match editor.current_buffer_id() {
                        Some(id) => id,
                        None => return Err(ExCommandError::Other("No current buffer".to_string())),
                    };
                    
                    let buffer = match editor.get_buffer_manager().get_buffer(buffer_id) {
                        Ok(buffer) => buffer,
                        Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
                    };
                    
                    buffer.line_count() - 1
                },
                crate::command::RangeSpec::LineNumber(n) => n - 1, // Convert to 0-based
                crate::command::RangeSpec::Mark(mark) => {
                    match editor.get_mark_position(mark) {
                        Some(pos) => pos.line,
                        None => return Err(ExCommandError::InvalidRange(format!("Mark '{}' not set", mark))),
                    }
                },
                crate::command::RangeSpec::Search(pattern) => {
                    // Search for the pattern
                    match editor.search(pattern.as_str(), true, false) {
                        Ok(Some(pos)) => pos.line,
                        Ok(None) => return Err(ExCommandError::InvalidRange(format!("Pattern not found: {}", pattern))),
                        Err(err) => return Err(ExCommandError::InvalidRange(format!("Search error: {}", err))),
                    }
                },
                crate::command::RangeSpec::Offset(offset) => {
                    let current_line = editor.cursor_position().line;
                    if offset >= 0 {
                        current_line + offset as usize
                    } else {
                        current_line.saturating_sub((-offset) as usize)
                    }
                },
            }
        } else {
            start_line
        };
        
        (start_line, end_line)
    } else {
        // If no range is provided, use the entire buffer
        let buffer_id = match editor.current_buffer_id() {
            Some(id) => id,
            None => return Err(ExCommandError::Other("No current buffer".to_string())),
        };
        
        let buffer = match editor.get_buffer_manager().get_buffer(buffer_id) {
            Ok(buffer) => buffer,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
        };
        
        (0, buffer.line_count() - 1)
    };
    
    // Get the buffer
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::Other("No current buffer".to_string())),
    };
    
    let buffer = match editor.get_buffer_manager().get_buffer(buffer_id) {
        Ok(buffer) => buffer,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
    };
    
    // Create a command parser
    let parser = crate::command::ExCommandParser::new();
    
    // Create a command registry
    let mut registry = crate::command::ExCommandRegistry::new();
    crate::command::handlers::register_handlers(&mut registry, None);
    
    // Find lines that don't match the pattern
    let mut non_matching_lines = Vec::new();
    
    for line_idx in range.0..=range.1 {
        // Get the line
        let line = match buffer.line(line_idx) {
            Ok(line) => line,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to get line: {}", err))),
        };
        
        // Check if the line doesn't match the pattern
        if !line.contains(pattern) {
            non_matching_lines.push(line_idx);
        }
    }
    
    // Execute the command on each non-matching line
    for line_idx in non_matching_lines {
        // Set the cursor position to the non-matching line
        if let Err(err) = editor.get_cursor_manager_mut().set_position(CursorPosition::new(line_idx, 0)) {
            return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
        }
        
        // Parse and execute the command
        match parser.parse(command) {
            Ok(ex_cmd) => {
                if let Err(err) = registry.execute(&ex_cmd) {
                    return Err(ExCommandError::Other(format!("Failed to execute command on line {}: {}", line_idx + 1, err)));
                }
            },
            Err(err) => {
                return Err(ExCommandError::Other(format!("Failed to parse command: {}", err)));
            }
        }
    }
    
    println!("Vglobal command executed on {} non-matching lines", non_matching_lines.len());
    Ok(())
}
