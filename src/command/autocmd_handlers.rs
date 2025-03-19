//! Autocmd command handlers
//!
//! This module implements handlers for autocmd commands.

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
use std::io::{Read, Write, BufRead, BufReader};
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
// use std::fs::File;

/// Autocmd event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutocmdEvent {
    /// Buffer enter
    BufEnter,
    /// Buffer leave
    BufLeave,
    /// Buffer write
    BufWrite,
    /// Buffer write post
    BufWritePost,
    /// Buffer read
    BufRead,
    /// Buffer read post
    BufReadPost,
    /// File type
    FileType,
    /// File read
    FileRead,
    /// File read post
    FileReadPost,
    /// File write
    FileWrite,
    /// File write post
    FileWritePost,
    /// Window enter
    WinEnter,
    /// Window leave
    WinLeave,
    /// Cursor moved
    CursorMoved,
    /// Cursor moved in insert mode
    CursorMovedI,
    /// Insert enter
    InsertEnter,
    /// Insert leave
    InsertLeave,
    /// Visual enter
    VisualEnter,
    /// Visual leave
    VisualLeave,
    /// Command line enter
    CmdlineEnter,
    /// Command line leave
    CmdlineLeave,
    /// Command line changed
    CmdlineChanged,
    /// Terminal open
    TerminalOpen,
    /// Terminal close
    TerminalClose,
    /// Vim enter
    VimEnter,
    /// Vim leave
    VimLeave,
    /// Vim leave pre
    VimLeavePre,
    /// Quit
    QuitPre,
    /// Quit
    Quit,
}

impl AutocmdEvent {
    /// Parse an event name
    pub fn parse(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "bufenter" => Some(Self::BufEnter),
            "bufleave" => Some(Self::BufLeave),
            "bufwrite" => Some(Self::BufWrite),
            "bufwritepost" => Some(Self::BufWritePost),
            "bufread" => Some(Self::BufRead),
            "bufreadpost" => Some(Self::BufReadPost),
            "filetype" => Some(Self::FileType),
            "fileread" => Some(Self::FileRead),
            "filereadpost" => Some(Self::FileReadPost),
            "filewrite" => Some(Self::FileWrite),
            "filewritepost" => Some(Self::FileWritePost),
            "winenter" => Some(Self::WinEnter),
            "winleave" => Some(Self::WinLeave),
            "cursormoved" => Some(Self::CursorMoved),
            "cursormovedi" => Some(Self::CursorMovedI),
            "insertenter" => Some(Self::InsertEnter),
            "insertleave" => Some(Self::InsertLeave),
            "visualenter" => Some(Self::VisualEnter),
            "visualleave" => Some(Self::VisualLeave),
            "cmdlineenter" => Some(Self::CmdlineEnter),
            "cmdlineleave" => Some(Self::CmdlineLeave),
            "cmdlinechanged" => Some(Self::CmdlineChanged),
            "terminalopen" => Some(Self::TerminalOpen),
            "terminalclose" => Some(Self::TerminalClose),
            "vimenter" => Some(Self::VimEnter),
            "vimleave" => Some(Self::VimLeave),
            "vimleavepre" => Some(Self::VimLeavePre),
            "quitpre" => Some(Self::QuitPre),
            "quit" => Some(Self::Quit),
            _ => None,
        }
    }

    /// Get the event name
    pub fn name(&self) -> &'static str {
        match self {
            Self::BufEnter => "BufEnter",
            Self::BufLeave => "BufLeave",
            Self::BufWrite => "BufWrite",
            Self::BufWritePost => "BufWritePost",
            Self::BufRead => "BufRead",
            Self::BufReadPost => "BufReadPost",
            Self::FileType => "FileType",
            Self::FileRead => "FileRead",
            Self::FileReadPost => "FileReadPost",
            Self::FileWrite => "FileWrite",
            Self::FileWritePost => "FileWritePost",
            Self::WinEnter => "WinEnter",
            Self::WinLeave => "WinLeave",
            Self::CursorMoved => "CursorMoved",
            Self::CursorMovedI => "CursorMovedI",
            Self::InsertEnter => "InsertEnter",
            Self::InsertLeave => "InsertLeave",
            Self::VisualEnter => "VisualEnter",
            Self::VisualLeave => "VisualLeave",
            Self::CmdlineEnter => "CmdlineEnter",
            Self::CmdlineLeave => "CmdlineLeave",
            Self::CmdlineChanged => "CmdlineChanged",
            Self::TerminalOpen => "TerminalOpen",
            Self::TerminalClose => "TerminalClose",
            Self::VimEnter => "VimEnter",
            Self::VimLeave => "VimLeave",
            Self::VimLeavePre => "VimLeavePre",
            Self::QuitPre => "QuitPre",
            Self::Quit => "Quit",
        }
    }
}

/// Autocmd pattern
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AutocmdPattern {
    /// Pattern
    pub pattern: String,
    /// Is regex
    pub is_regex: bool,
}

impl AutocmdPattern {
    /// Create a new pattern
    pub fn new(pattern: &str, is_regex: bool) -> Self {
        Self {
            pattern: pattern.to_string(),
            is_regex,
        }
    }

    /// Check if the pattern matches a path
    pub fn matches(&self, path: &Path) -> bool {
        if self.pattern == "*" {
            return true;
        }
        
        let path_str = path.to_string_lossy();
        
        if self.is_regex {
            // Use regex matching
            match regex::Regex::new(&self.pattern) {
                Ok(re) => re.is_match(&path_str),
                Err(_) => false,
            }
        } else {
            // Use regex matching with a glob-like pattern
            // Convert glob pattern to regex pattern
            let regex_pattern = self.pattern.replace("*", ".*").replace("?", ".");
            match regex::Regex::new(&regex_pattern) {
                Ok(re) => re.is_match(&path_str),
                Err(_) => false,
            }
        }
    }
}

/// Autocmd command
#[derive(Debug, Clone)]
pub struct AutocmdCommand {
    /// Command
    pub command: String,
    /// Group
    pub group: Option<String>,
    /// Once
    pub once: bool,
    /// Nested
    pub nested: bool,
}

/// Autocmd
#[derive(Debug, Clone)]
pub struct Autocmd {
    /// Event
    pub event: AutocmdEvent,
    /// Pattern
    pub pattern: AutocmdPattern,
    /// Command
    pub command: AutocmdCommand,
}

/// Autocmd group
#[derive(Debug, Clone)]
pub struct AutocmdGroup {
    /// Name
    pub name: String,
    /// Autocmds
    pub autocmds: Vec<Autocmd>,
}

/// Autocmd manager
#[derive(Debug)]
pub struct AutocmdManager {
    /// Groups
    pub groups: HashMap<String, AutocmdGroup>,
    /// Default group
    pub default_group: String,
    /// Current group
    pub current_group: Option<String>,
    /// Autocmd history
    pub history: VecDeque<AutocmdAction>,
    /// Maximum history size
    pub max_history_size: usize,
    /// Nested level
    pub nested_level: usize,
    /// Maximum nested level
    pub max_nested_level: usize,
}

/// Autocmd action
#[derive(Debug, Clone)]
pub enum AutocmdAction {
    /// Add autocmd
    Add(Autocmd),
    /// Remove autocmd
    Remove(AutocmdEvent, AutocmdPattern, Option<String>),
    /// Clear autocmds
    Clear(Option<AutocmdEvent>, Option<AutocmdPattern>, Option<String>),
    /// Create group
    CreateGroup(String),
    /// Delete group
    DeleteGroup(String),
    /// Set current group
    SetCurrentGroup(Option<String>),
}

impl AutocmdManager {
    /// Create a new autocmd manager
    pub fn new() -> Self {
        let mut groups = HashMap::new();
        
        // Create the default group
        let default_group = "xvim".to_string();
        groups.insert(default_group.clone(), AutocmdGroup {
            name: default_group.clone(),
            autocmds: Vec::new(),
        });
        
        Self {
            groups,
            default_group,
            current_group: None,
            history: VecDeque::new(),
            max_history_size: 100,
            nested_level: 0,
            max_nested_level: 20,
        }
    }

    /// Add an autocmd
    pub fn add_autocmd(&mut self, autocmd: Autocmd) -> ExCommandResult<()> {
        // Get the group
        let group_name = match &autocmd.command.group {
            Some(name) => name.clone(),
            None => match &self.current_group {
                Some(name) => name.clone(),
                None => self.default_group.clone(),
            },
        };
        
        // Check if the group exists
        if !self.groups.contains_key(&group_name) {
            // Create the group
            self.groups.insert(group_name.clone(), AutocmdGroup {
                name: group_name.clone(),
                autocmds: Vec::new(),
            });
        }
        
        // Get the group
        let group = self.groups.get_mut(&group_name).unwrap();
        
        // Add the autocmd to the group
        group.autocmds.push(autocmd.clone());
        
        // Add to history
        self.add_to_history(AutocmdAction::Add(autocmd));
        
        Ok(())
    }

    /// Remove autocmds
    pub fn remove_autocmds(&mut self, event: Option<AutocmdEvent>, pattern: Option<AutocmdPattern>, group: Option<String>) -> ExCommandResult<()> {
        // Get the group
        let group_name = match &group {
            Some(name) => name.clone(),
            None => match &self.current_group {
                Some(name) => name.clone(),
                None => self.default_group.clone(),
            },
        };
        
        // Check if the group exists
        if !self.groups.contains_key(&group_name) {
            return Ok(());
        }
        
        // Get the group
        let group = self.groups.get_mut(&group_name).unwrap();
        
        // Remove autocmds
        if let Some(event) = event {
            if let Some(pattern) = pattern {
                // Remove autocmds with the specified event and pattern
                group.autocmds.retain(|a| a.event != event || a.pattern != pattern);
                
                // Add to history
                self.add_to_history(AutocmdAction::Remove(event, pattern, Some(group_name)));
            } else {
                // Remove autocmds with the specified event
                group.autocmds.retain(|a| a.event != event);
                
                // Add to history
                self.add_to_history(AutocmdAction::Clear(Some(event), None, Some(group_name)));
            }
        } else if let Some(pattern) = pattern {
            // Remove autocmds with the specified pattern
            group.autocmds.retain(|a| a.pattern != pattern);
            
            // Add to history
            self.add_to_history(AutocmdAction::Clear(None, Some(pattern), Some(group_name)));
        } else {
            // Remove all autocmds
            group.autocmds.clear();
            
            // Add to history
            self.add_to_history(AutocmdAction::Clear(None, None, Some(group_name)));
        }
        
        Ok(())
    }

    /// Create a group
    pub fn create_group(&mut self, name: &str) -> ExCommandResult<()> {
        // Check if the group already exists
        if self.groups.contains_key(name) {
            return Ok(());
        }
        
        // Create the group
        self.groups.insert(name.to_string(), AutocmdGroup {
            name: name.to_string(),
            autocmds: Vec::new(),
        });
        
        // Add to history
        self.add_to_history(AutocmdAction::CreateGroup(name.to_string()));
        
        Ok(())
    }

    /// Delete a group
    pub fn delete_group(&mut self, name: &str) -> ExCommandResult<()> {
        // Check if the group exists
        if !self.groups.contains_key(name) {
            return Ok(());
        }
        
        // Check if the group is the default group
        if name == self.default_group {
            return Err(ExCommandError::InvalidArgument("Cannot delete the default group".to_string()));
        }
        
        // Remove the group
        self.groups.remove(name);
        
        // If the current group is the deleted group, set it to None
        if let Some(current_group) = &self.current_group {
            if current_group == name {
                self.current_group = None;
            }
        }
        
        // Add to history
        self.add_to_history(AutocmdAction::DeleteGroup(name.to_string()));
        
        Ok(())
    }

    /// Set the current group
    pub fn set_current_group(&mut self, name: Option<&str>) -> ExCommandResult<()> {
        // Set the current group
        self.current_group = name.map(|s| s.to_string());
        
        // Add to history
        self.add_to_history(AutocmdAction::SetCurrentGroup(self.current_group.clone()));
        
        Ok(())
    }

    /// Trigger an event
    pub fn trigger_event(&mut self, event: AutocmdEvent, path: &Path) -> ExCommandResult<()> {
        // Check if we've reached the maximum nested level
        if self.nested_level >= self.max_nested_level {
            return Err(ExCommandError::Other(format!("Autocmd nested too deeply (max {})", self.max_nested_level)));
        }
        
        // Increment the nested level
        self.nested_level += 1;
        
        // Get the editor reference
        let editor = unsafe {
            match crate::command::handlers::EDITOR {
                Some(editor_ptr) => &mut *editor_ptr,
                None => {
                    // Decrement the nested level
                    self.nested_level -= 1;
                    return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string()));
                }
            }
        };
        
        // Get matching autocmds
        let mut matching_autocmds = Vec::new();
        
        for group in self.groups.values() {
            for autocmd in &group.autocmds {
                if autocmd.event == event && autocmd.pattern.matches(path) {
                    matching_autocmds.push(autocmd.clone());
                }
            }
        }
        
        // Execute matching autocmds
        for autocmd in matching_autocmds {
            // Parse the command
            let command = match crate::command::ExCommandParser::parse(&autocmd.command.command) {
                Ok(cmd) => cmd,
                Err(err) => {
                    // Decrement the nested level
                    self.nested_level -= 1;
                    return Err(ExCommandError::Other(format!("Failed to parse autocmd command: {}", err)));
                }
            };
            
            // Execute the command
            match editor.execute_command(&command) {
                Ok(_) => {},
                Err(err) => {
                    // Decrement the nested level
                    self.nested_level -= 1;
                    return Err(ExCommandError::Other(format!("Failed to execute autocmd command: {}", err)));
                }
            }
            
            // If the autocmd is once, remove it
            if autocmd.command.once {
                self.remove_autocmds(Some(autocmd.event), Some(autocmd.pattern), autocmd.command.group)?;
            }
        }
        
        // Decrement the nested level
        self.nested_level -= 1;
        
        Ok(())
    }

    /// Get autocmds
    pub fn get_autocmds(&self, event: Option<AutocmdEvent>, pattern: Option<&str>, group: Option<&str>) -> Vec<Autocmd> {
        let mut autocmds = Vec::new();
        
        // Get the group
        let group_name = match group {
            Some(name) => name,
            None => match &self.current_group {
                Some(name) => name,
                None => &self.default_group,
            },
        };
        
        // Check if the group exists
        if !self.groups.contains_key(group_name) {
            return autocmds;
        }
        
        // Get the group
        let group = &self.groups[group_name];
        
        // Get autocmds
        for autocmd in &group.autocmds {
            if let Some(event) = event {
                if autocmd.event != event {
                    continue;
                }
            }
            
            if let Some(pattern) = pattern {
                if autocmd.pattern.pattern != pattern {
                    continue;
                }
            }
            
            autocmds.push(autocmd.clone());
        }
        
        autocmds
    }

    /// Get all autocmds
    pub fn get_all_autocmds(&self) -> Vec<Autocmd> {
        let mut autocmds = Vec::new();
        
        for group in self.groups.values() {
            for autocmd in &group.autocmds {
                autocmds.push(autocmd.clone());
            }
        }
        
        autocmds
    }

    /// Get groups
    pub fn get_groups(&self) -> Vec<String> {
        self.groups.keys().cloned().collect()
    }

    /// Get the current group
    pub fn get_current_group(&self) -> Option<&str> {
        self.current_group.as_deref()
    }

    /// Add an action to the history
    fn add_to_history(&mut self, action: AutocmdAction) {
        // Add the action to the history
        self.history.push_front(action);
        
        // Limit the history size
        if self.history.len() > self.max_history_size {
            self.history.pop_back();
        }
    }
}

// Global autocmd manager
static mut AUTOCMD_MANAGER: Option<AutocmdManager> = None;

/// Initialize the autocmd manager
pub fn init_autocmd_manager() {
    unsafe {
        if AUTOCMD_MANAGER.is_none() {
            AUTOCMD_MANAGER = Some(AutocmdManager::new());
        }
    }
}

/// Register autocmd command handlers
pub fn register_autocmd_handlers(registry: &mut ExCommandRegistry) {
    // Initialize the autocmd manager
    init_autocmd_manager();
    
    // Register autocmd commands
    registry.register("autocmd", handle_autocmd);
    registry.register("augroup", handle_augroup);
    registry.register("doautocmd", handle_doautocmd);
}

/// Handle the :autocmd command
fn handle_autocmd(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the autocmd manager
    let autocmd_manager = unsafe {
        match &mut AUTOCMD_MANAGER {
            Some(manager) => manager,
            None => {
                init_autocmd_manager();
                match &mut AUTOCMD_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize autocmd manager".to_string())),
                }
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // Display all autocmds
        let autocmds = autocmd_manager.get_all_autocmds();
        
        if autocmds.is_empty() {
            println!("No autocmds defined");
            return Ok(());
        }
        
        // Group autocmds by event
        let mut event_groups: HashMap<AutocmdEvent, Vec<Autocmd>> = HashMap::new();
        
        for autocmd in autocmds {
            event_groups.entry(autocmd.event).or_insert_with(Vec::new).push(autocmd);
        }
        
        // Display autocmds by event
        for (event, autocmds) in event_groups {
            println!("--- {} ---", event.name());
            
            for autocmd in autocmds {
                let group = match &autocmd.command.group {
                    Some(name) => name,
                    None => match &autocmd_manager.current_group {
                        Some(name) => name,
                        None => &autocmd_manager.default_group,
                    },
                };
                
                println!("{}  {}  {}", group, autocmd.pattern.pattern, autocmd.command.command);
            }
            
            println!();
        }
        
        return Ok(());
    }
    
    // Parse the arguments
    let parts: Vec<&str> = args.split_whitespace().collect();
    
    if parts.is_empty() {
        return Err(ExCommandError::InvalidArgument("Invalid autocmd command".to_string()));
    }
    
    // Check if the first part is an event
    if let Some(event) = AutocmdEvent::parse(parts[0]) {
        // Check if there are more parts
        if parts.len() == 1 {
            // Display autocmds for the event
            let autocmds = autocmd_manager.get_autocmds(Some(event), None, None);
            
            if autocmds.is_empty() {
                println!("No autocmds for {}", event.name());
                return Ok(());
            }
            
            println!("--- {} ---", event.name());
            
            for autocmd in autocmds {
                let group = match &autocmd.command.group {
                    Some(name) => name,
                    None => match &autocmd_manager.current_group {
                        Some(name) => name,
                        None => &autocmd_manager.default_group,
                    },
                };
                
                println!("{}  {}  {}", group, autocmd.pattern.pattern, autocmd.command.command);
            }
            
            return Ok(());
        }
        
        // Get the pattern
        let pattern = parts[1];
        
        // Check if there are more parts
        if parts.len() == 2 {
            // Display autocmds for the event and pattern
            let autocmds = autocmd_manager.get_autocmds(Some(event), Some(pattern), None);
            
            if autocmds.is_empty() {
                println!("No autocmds for {} {}", event.name(), pattern);
                return Ok(());
            }
            
            println!("--- {} {} ---", event.name(), pattern);
            
            for autocmd in autocmds {
                let group = match &autocmd.command.group {
                    Some(name) => name,
                    None => match &autocmd_manager.current_group {
                        Some(name) => name,
                        None => &autocmd_manager.default_group,
                    },
                };
                
                println!("{}  {}  {}", group, autocmd.pattern.pattern, autocmd.command.command);
            }
            
            return Ok(());
        }
        
        // Get the command
        let command = parts[2..].join(" ");
        
        // Check if the command is empty
        if command.is_empty() {
            // Remove autocmds for the event and pattern
            autocmd_manager.remove_autocmds(Some(event), Some(AutocmdPattern::new(pattern, false)), None)?;
            
            println!("Removed autocmds for {} {}", event.name(), pattern);
            
            return Ok(());
        }
        
        // Parse options
        let mut group = None;
        let mut once = false;
        let mut nested = false;
        let mut is_regex = false;
        
        let command = if command.contains("|") {
            let parts: Vec<&str> = command.splitn(2, "|").collect();
            
            // Parse options
            for option in parts[0].split_whitespace() {
                match option {
                    "++once" => once = true,
                    "++nested" => nested = true,
                    "++group" => {
                        // Get the group name
                        let group_parts: Vec<&str> = option.splitn(2, "=").collect();
                        
                        if group_parts.len() == 2 {
                            group = Some(group_parts[1].to_string());
                        }
                    },
                    "++regex" => is_regex = true,
                    _ => {},
                }
            }
            
            parts[1].to_string()
        } else {
            command
        };
        
        // Add the autocmd
        let autocmd = Autocmd {
            event,
            pattern: AutocmdPattern::new(pattern, is_regex),
            command: AutocmdCommand {
                command,
                group,
                once,
                nested,
            },
        };
        
        autocmd_manager.add_autocmd(autocmd)?;
        
        println!("Added autocmd for {} {}", event.name(), pattern);
        
        return Ok(());
    }
    
    // Check if the first part is a special command
    match parts[0] {
        "!" => {
            // Remove all autocmds
            autocmd_manager.remove_autocmds(None, None, None)?;
            
            println!("Removed all autocmds");
            
            return Ok(());
        },
        _ => {
            return Err(ExCommandError::InvalidArgument(format!("Invalid autocmd command: {}", parts[0])));
        }
    }
}

/// Handle the :augroup command
fn handle_augroup(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the autocmd manager
    let autocmd_manager = unsafe {
        match &mut AUTOCMD_MANAGER {
            Some(manager) => manager,
            None => {
                init_autocmd_manager();
                match &mut AUTOCMD_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize autocmd manager".to_string())),
                }
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // Display all groups
        let groups = autocmd_manager.get_groups();
        
        if groups.is_empty() {
            println!("No augroups defined");
            return Ok(());
        }
        
        println!("--- Augroups ---");
        
        for group in groups {
            println!("{}", group);
        }
        
        return Ok(());
    }
    
    // Parse the arguments
    let parts: Vec<&str> = args.split_whitespace().collect();
    
    if parts.is_empty() {
        return Err(ExCommandError::InvalidArgument("Invalid augroup command".to_string()));
    }
    
    // Check if the first part is a special command
    if parts[0] == "END" {
        // End the current group
        autocmd_manager.set_current_group(None)?;
        
        println!("Ended augroup");
        
        return Ok(());
    }
    
    // Check if the first part is a group name
    let group_name = parts[0];
    
    // Check if there are more parts
    if parts.len() > 1 && parts[1] == "!" {
        // Delete the group
        autocmd_manager.delete_group(group_name)?;
        
        // Create the group
        autocmd_manager.create_group(group_name)?;
        
        // Set the current group
        autocmd_manager.set_current_group(Some(group_name))?;
        
        println!("Recreated augroup {}", group_name);
        
        return Ok(());
    }
    
    // Create the group
    autocmd_manager.create_group(group_name)?;
    
    // Set the current group
    autocmd_manager.set_current_group(Some(group_name))?;
    
    println!("Created augroup {}", group_name);
    
    Ok(())
}

/// Handle the :doautocmd command
fn handle_doautocmd(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the autocmd manager
    let autocmd_manager = unsafe {
        match &mut AUTOCMD_MANAGER {
            Some(manager) => manager,
            None => {
                init_autocmd_manager();
                match &mut AUTOCMD_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize autocmd manager".to_string())),
                }
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Event name required".to_string()));
    }
    
    // Parse the arguments
    let parts: Vec<&str> = args.split_whitespace().collect();
    
    if parts.is_empty() {
        return Err(ExCommandError::InvalidArgument("Invalid doautocmd command".to_string()));
    }
    
    // Check if the first part is an event
    if let Some(event) = AutocmdEvent::parse(parts[0]) {
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
            None => return Err(ExCommandError::InvalidCommand("No buffer to trigger event for".to_string())),
        };
        
        // Get the buffer
        let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
        
        // Get the buffer path
        let buffer_path = match buffer.get_path() {
            Some(path) => path,
            None => PathBuf::from(""),
        };
        
        // Trigger the event
        autocmd_manager.trigger_event(event, &buffer_path)?;
        
        println!("Triggered {} event", event.name());
        
        return Ok(());
    }
    
    Err(ExCommandError::InvalidArgument(format!("Invalid event name: {}", parts[0])))
}

/// Trigger an event
pub fn trigger_event(event: AutocmdEvent, path: &Path) -> ExCommandResult<()> {
    // Get the autocmd manager
    let autocmd_manager = unsafe {
        match &mut AUTOCMD_MANAGER {
            Some(manager) => manager,
            None => {
                init_autocmd_manager();
                match &mut AUTOCMD_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize autocmd manager".to_string())),
                }
            }
        }
    };
    
    // Trigger the event
    autocmd_manager.trigger_event(event, path)
}

/// Add an autocmd
pub fn add_autocmd(event: AutocmdEvent, pattern: &str, command: &str, group: Option<&str>, once: bool, nested: bool, is_regex: bool) -> ExCommandResult<()> {
    // Get the autocmd manager
    let autocmd_manager = unsafe {
        match &mut AUTOCMD_MANAGER {
            Some(manager) => manager,
            None => {
                init_autocmd_manager();
                match &mut AUTOCMD_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize autocmd manager".to_string())),
                }
            }
        }
    };
    
    // Add the autocmd
    let autocmd = Autocmd {
        event,
        pattern: AutocmdPattern::new(pattern, is_regex),
        command: AutocmdCommand {
            command: command.to_string(),
            group: group.map(|s| s.to_string()),
            once,
            nested,
        },
    };
    
    autocmd_manager.add_autocmd(autocmd)
}

/// Remove autocmds
pub fn remove_autocmds(event: Option<AutocmdEvent>, pattern: Option<&str>, group: Option<&str>) -> ExCommandResult<()> {
    // Get the autocmd manager
    let autocmd_manager = unsafe {
        match &mut AUTOCMD_MANAGER {
            Some(manager) => manager,
            None => {
                init_autocmd_manager();
                match &mut AUTOCMD_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize autocmd manager".to_string())),
                }
            }
        }
    };
    
    // Remove autocmds
    autocmd_manager.remove_autocmds(
        event,
        pattern.map(|p| AutocmdPattern::new(p, false)),
        group.map(|s| s.to_string())
    )
}

/// Create a group
pub fn create_group(name: &str) -> ExCommandResult<()> {
    // Get the autocmd manager
    let autocmd_manager = unsafe {
        match &mut AUTOCMD_MANAGER {
            Some(manager) => manager,
            None => {
                init_autocmd_manager();
                match &mut AUTOCMD_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize autocmd manager".to_string())),
                }
            }
        }
    };
    
    // Create the group
    autocmd_manager.create_group(name)
}

/// Delete a group
pub fn delete_group(name: &str) -> ExCommandResult<()> {
    // Get the autocmd manager
    let autocmd_manager = unsafe {
        match &mut AUTOCMD_MANAGER {
            Some(manager) => manager,
            None => {
                init_autocmd_manager();
                match &mut AUTOCMD_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize autocmd manager".to_string())),
                }
            }
        }
    };
    
    // Delete the group
    autocmd_manager.delete_group(name)
}

/// Set the current group
pub fn set_current_group(name: Option<&str>) -> ExCommandResult<()> {
    // Get the autocmd manager
    let autocmd_manager = unsafe {
        match &mut AUTOCMD_MANAGER {
            Some(manager) => manager,
            None => {
                init_autocmd_manager();
                match &mut AUTOCMD_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize autocmd manager".to_string())),
                }
            }
        }
    };
    
    // Set the current group
    autocmd_manager.set_current_group(name)
}

/// Get autocmds
pub fn get_autocmds(event: Option<AutocmdEvent>, pattern: Option<&str>, group: Option<&str>) -> Vec<Autocmd> {
    // Get the autocmd manager
    let autocmd_manager = unsafe {
        match &AUTOCMD_MANAGER {
            Some(manager) => manager,
            None => {
                init_autocmd_manager();
                match &AUTOCMD_MANAGER {
                    Some(manager) => manager,
                    None => return Vec::new(),
                }
            }
        }
    };
    
    // Get autocmds
    autocmd_manager.get_autocmds(event, pattern, group)
}

/// Get all autocmds
pub fn get_all_autocmds() -> Vec<Autocmd> {
    // Get the autocmd manager
    let autocmd_manager = unsafe {
        match &AUTOCMD_MANAGER {
            Some(manager) => manager,
            None => {
                init_autocmd_manager();
                match &AUTOCMD_MANAGER {
                    Some(manager) => manager,
                    None => return Vec::new(),
                }
            }
        }
    };
    
    // Get all autocmds
    autocmd_manager.get_all_autocmds()
}

/// Get groups
pub fn get_groups() -> Vec<String> {
    // Get the autocmd manager
    let autocmd_manager = unsafe {
        match &AUTOCMD_MANAGER {
            Some(manager) => manager,
            None => {
                init_autocmd_manager();
                match &AUTOCMD_MANAGER {
                    Some(manager) => manager,
                    None => return Vec::new(),
                }
            }
        }
    };
    
    // Get groups
    autocmd_manager.get_groups()
}

/// Get the current group
pub fn get_current_group() -> Option<String> {
    // Get the autocmd manager
    let autocmd_manager = unsafe {
        match &AUTOCMD_MANAGER {
            Some(manager) => manager,
            None => {
                init_autocmd_manager();
                match &AUTOCMD_MANAGER {
                    Some(manager) => manager,
                    None => return None,
                }
            }
        }
    };
    
    // Get the current group
    autocmd_manager.get_current_group().map(|s| s.to_string())
}