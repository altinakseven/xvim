//! Ex command implementation
//!
//! This module implements the ex command system for xvim, which handles
//! commands that start with a colon (e.g., :w, :q, :set, etc.).

use std::str::FromStr;
use std::fmt;
use std::collections::HashMap;
use std::sync::Arc;

/// Error type for ex command parsing
#[derive(Debug, Clone, PartialEq)]
pub enum ExCommandError {
    /// Invalid command
    InvalidCommand(String),
    /// Invalid range
    InvalidRange(String),
    /// Invalid argument
    InvalidArgument(String),
    /// Missing required argument
    MissingArgument(String),
    /// Unknown command
    UnknownCommand(String),
}

impl fmt::Display for ExCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExCommandError::InvalidCommand(msg) => write!(f, "Invalid command: {}", msg),
            ExCommandError::InvalidRange(msg) => write!(f, "Invalid range: {}", msg),
            ExCommandError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            ExCommandError::MissingArgument(msg) => write!(f, "Missing argument: {}", msg),
            ExCommandError::UnknownCommand(cmd) => write!(f, "Unknown command: {}", cmd),
        }
    }
}

impl std::error::Error for ExCommandError {}

/// Result type for ex command operations
pub type ExCommandResult<T> = Result<T, ExCommandError>;

/// Command range specification
#[derive(Debug, Clone, PartialEq)]
pub enum RangeSpec {
    /// Current line (.)
    CurrentLine,
    /// Last line ($)
    LastLine,
    /// Specific line number
    LineNumber(usize),
    /// Line with mark
    Mark(char),
    /// Search pattern
    Search(String),
    /// Relative offset from current line
    Offset(isize),
}

impl FromStr for RangeSpec {
    type Err = ExCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ExCommandError::InvalidRange("Empty range specification".to_string()));
        }

        match s {
            "." => Ok(RangeSpec::CurrentLine),
            "$" => Ok(RangeSpec::LastLine),
            _ if s.starts_with('\'') && s.len() == 2 => {
                let mark = s.chars().nth(1).unwrap();
                Ok(RangeSpec::Mark(mark))
            },
            _ if s.starts_with('/') && s.len() > 1 => {
                let pattern = s[1..].to_string();
                Ok(RangeSpec::Search(pattern))
            },
            _ if s.starts_with('+') => {
                let offset = s[1..].parse::<isize>().map_err(|_| {
                    ExCommandError::InvalidRange(format!("Invalid offset: {}", s))
                })?;
                Ok(RangeSpec::Offset(offset))
            },
            _ if s.starts_with('-') => {
                let offset = -s[1..].parse::<isize>().map_err(|_| {
                    ExCommandError::InvalidRange(format!("Invalid offset: {}", s))
                })?;
                Ok(RangeSpec::Offset(offset))
            },
            _ => {
                // Try to parse as a line number
                match s.parse::<usize>() {
                    Ok(num) => Ok(RangeSpec::LineNumber(num)),
                    Err(_) => Err(ExCommandError::InvalidRange(format!("Invalid range: {}", s))),
                }
            }
        }
    }
}

/// Command range
#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    /// Start of range
    pub start: Option<RangeSpec>,
    /// End of range
    pub end: Option<RangeSpec>,
}

impl Range {
    /// Create a new range
    pub fn new(start: Option<RangeSpec>, end: Option<RangeSpec>) -> Self {
        Self { start, end }
    }

    /// Create a range for the current line
    pub fn current_line() -> Self {
        Self {
            start: Some(RangeSpec::CurrentLine),
            end: None,
        }
    }

    /// Create a range for the entire buffer
    pub fn entire_buffer() -> Self {
        Self {
            start: Some(RangeSpec::LineNumber(1)),
            end: Some(RangeSpec::LastLine),
        }
    }

    /// Check if the range is empty (no start or end)
    pub fn is_empty(&self) -> bool {
        self.start.is_none() && self.end.is_none()
    }

    /// Check if the range is a single line
    pub fn is_single_line(&self) -> bool {
        self.start.is_some() && self.end.is_none()
    }
}

/// Command flags
#[derive(Debug, Clone, PartialEq)]
pub struct CommandFlags {
    /// Force flag (!)
    pub force: bool,
    /// Print flag (p)
    pub print: bool,
    /// List flag (l)
    pub list: bool,
    /// Number flag (#)
    pub number: bool,
}

impl Default for CommandFlags {
    fn default() -> Self {
        Self {
            force: false,
            print: false,
            list: false,
            number: false,
        }
    }
}

/// Ex command
#[derive(Debug, Clone, PartialEq)]
pub struct ExCommand {
    /// Command name
    pub name: String,
    /// Command range
    pub range: Range,
    /// Command flags
    pub flags: CommandFlags,
    /// Command arguments
    pub args: Vec<String>,
    /// Raw command string
    pub raw: String,
}

impl ExCommand {
    /// Create a new ex command
    pub fn new(name: &str, range: Range, flags: CommandFlags, args: Vec<String>, raw: &str) -> Self {
        Self {
            name: name.to_string(),
            range,
            flags,
            args,
            raw: raw.to_string(),
        }
    }

    /// Get the first argument as a string
    pub fn first_arg(&self) -> Option<&str> {
        self.args.first().map(|s| s.as_str())
    }

    /// Get the arguments as a single string
    pub fn args_str(&self) -> String {
        self.args.join(" ")
    }
}

/// Ex command parser
pub struct ExCommandParser {
    /// Command aliases
    aliases: HashMap<String, String>,
}

impl ExCommandParser {
    /// Create a new ex command parser
    pub fn new() -> Self {
        let mut parser = Self {
            aliases: HashMap::new(),
        };
        parser.init_aliases();
        parser
    }

    /// Initialize command aliases
    fn init_aliases(&mut self) {
        // File operations
        self.aliases.insert("w".to_string(), "write".to_string());
        self.aliases.insert("wq".to_string(), "wquit".to_string());
        self.aliases.insert("x".to_string(), "xit".to_string());
        self.aliases.insert("q".to_string(), "quit".to_string());
        self.aliases.insert("e".to_string(), "edit".to_string());
        self.aliases.insert("r".to_string(), "read".to_string());
        
        // Window operations
        self.aliases.insert("sp".to_string(), "split".to_string());
        self.aliases.insert("vs".to_string(), "vsplit".to_string());
        self.aliases.insert("clo".to_string(), "close".to_string());
        self.aliases.insert("wn".to_string(), "wnext".to_string());
        self.aliases.insert("wp".to_string(), "wprevious".to_string());
        
        // Tab operations
        self.aliases.insert("tabe".to_string(), "tabedit".to_string());
        self.aliases.insert("tabc".to_string(), "tabclose".to_string());
        self.aliases.insert("tabn".to_string(), "tabnext".to_string());
        self.aliases.insert("tabp".to_string(), "tabprevious".to_string());
        
        // Editing operations
        self.aliases.insert("d".to_string(), "delete".to_string());
        self.aliases.insert("y".to_string(), "yank".to_string());
        self.aliases.insert("m".to_string(), "move".to_string());
        self.aliases.insert("co".to_string(), "copy".to_string());
        self.aliases.insert("t".to_string(), "copy".to_string());
        self.aliases.insert("s".to_string(), "substitute".to_string());
        
        // Other operations
        self.aliases.insert("u".to_string(), "undo".to_string());
        self.aliases.insert("red".to_string(), "redo".to_string());
        self.aliases.insert("se".to_string(), "set".to_string());
    }

    /// Parse a command string
    pub fn parse(&self, input: &str) -> ExCommandResult<ExCommand> {
        let input = input.trim();
        
        if input.is_empty() {
            return Err(ExCommandError::InvalidCommand("Empty command".to_string()));
        }
        
        // Parse range
        let (range, rest) = self.parse_range(input)?;
        
        // Parse command name
        let (name, rest) = self.parse_command_name(rest)?;
        
        // Parse flags
        let (flags, rest) = self.parse_flags(rest)?;
        
        // Parse arguments
        let args = self.parse_args(rest)?;
        
        // Resolve command alias
        let resolved_name = self.resolve_alias(&name);
        
        Ok(ExCommand::new(&resolved_name, range, flags, args, input))
    }
    
    /// Parse a range specification
    fn parse_range<'a>(&self, input: &'a str) -> ExCommandResult<(Range, &'a str)> {
        let mut chars = input.chars().peekable();
        let mut range_str = String::new();
        
        // Check for range characters
        while let Some(&c) = chars.peek() {
            match c {
                '.' | '$' | '\'' | '/' | '?' | '+' | '-' | '0'..='9' | ',' | ';' => {
                    range_str.push(c);
                    chars.next();
                },
                _ => break,
            }
        }
        
        let rest = &input[range_str.len()..];
        
        if range_str.is_empty() {
            return Ok((Range::new(None, None), rest));
        }
        
        // Parse the range
        if range_str.contains(',') {
            // Range with comma separator (e.g., 1,5)
            let parts: Vec<&str> = range_str.split(',').collect();
            if parts.len() != 2 {
                return Err(ExCommandError::InvalidRange(format!("Invalid range: {}", range_str)));
            }
            
            let start = if parts[0].is_empty() {
                None
            } else {
                Some(RangeSpec::from_str(parts[0])?)
            };
            
            let end = if parts[1].is_empty() {
                None
            } else {
                Some(RangeSpec::from_str(parts[1])?)
            };
            
            Ok((Range::new(start, end), rest))
        } else if range_str.contains(';') {
            // Range with semicolon separator (e.g., 1;5)
            let parts: Vec<&str> = range_str.split(';').collect();
            if parts.len() != 2 {
                return Err(ExCommandError::InvalidRange(format!("Invalid range: {}", range_str)));
            }
            
            let start = if parts[0].is_empty() {
                None
            } else {
                Some(RangeSpec::from_str(parts[0])?)
            };
            
            let end = if parts[1].is_empty() {
                None
            } else {
                Some(RangeSpec::from_str(parts[1])?)
            };
            
            // Note: semicolon means the second address is relative to the first
            Ok((Range::new(start, end), rest))
        } else {
            // Single line range
            let spec = RangeSpec::from_str(&range_str)?;
            Ok((Range::new(Some(spec), None), rest))
        }
    }
    
    /// Parse a command name
    fn parse_command_name<'a>(&self, input: &'a str) -> ExCommandResult<(String, &'a str)> {
        let input = input.trim_start();
        
        if input.is_empty() {
            return Err(ExCommandError::InvalidCommand("Missing command name".to_string()));
        }
        
        let mut chars = input.chars();
        let mut name = String::new();
        
        // Get the command name (letters only)
        while let Some(c) = chars.next() {
            if c.is_alphabetic() {
                name.push(c);
            } else {
                break;
            }
        }
        
        if name.is_empty() {
            return Err(ExCommandError::InvalidCommand("Invalid command name".to_string()));
        }
        
        let rest = &input[name.len()..];
        Ok((name, rest))
    }
    
    /// Parse command flags
    fn parse_flags<'a>(&self, input: &'a str) -> ExCommandResult<(CommandFlags, &'a str)> {
        let input = input.trim_start();
        let mut flags = CommandFlags::default();
        let mut pos = 0;
        
        for (i, c) in input.chars().enumerate() {
            match c {
                '!' => flags.force = true,
                'p' => flags.print = true,
                'l' => flags.list = true,
                '#' => flags.number = true,
                _ => {
                    pos = i;
                    break;
                }
            }
            pos = i + 1;
        }
        
        let rest = &input[pos..];
        Ok((flags, rest))
    }
    
    /// Parse command arguments
    fn parse_args(&self, input: &str) -> ExCommandResult<Vec<String>> {
        let input = input.trim_start();
        
        if input.is_empty() {
            return Ok(Vec::new());
        }
        
        // Split by whitespace, but respect quotes
        let mut args = Vec::new();
        let mut current_arg = String::new();
        let mut in_quotes = false;
        let mut escape_next = false;
        
        for c in input.chars() {
            if escape_next {
                current_arg.push(c);
                escape_next = false;
            } else if c == '\\' {
                escape_next = true;
            } else if c == '"' {
                in_quotes = !in_quotes;
            } else if c.is_whitespace() && !in_quotes {
                if !current_arg.is_empty() {
                    args.push(current_arg);
                    current_arg = String::new();
                }
            } else {
                current_arg.push(c);
            }
        }
        
        if !current_arg.is_empty() {
            args.push(current_arg);
        }
        
        Ok(args)
    }
    
    /// Resolve a command alias
    fn resolve_alias(&self, name: &str) -> String {
        self.aliases.get(name).cloned().unwrap_or_else(|| name.to_string())
    }
}

/// Ex command registry
#[derive(Clone)]
pub struct ExCommandRegistry {
    /// Command handlers
    handlers: HashMap<String, Arc<dyn Fn(&ExCommand) -> ExCommandResult<()> + Send + Sync>>,
}

impl ExCommandRegistry {
    /// Create a new ex command registry
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
    
    /// Register a command handler
    pub fn register<F>(&mut self, name: &str, handler: F)
    where
        F: Fn(&ExCommand) -> ExCommandResult<()> + Send + Sync + 'static,
    {
        self.handlers.insert(name.to_string(), Arc::new(handler));
    }
    
    /// Execute a command
    pub fn execute(&self, cmd: &ExCommand) -> ExCommandResult<()> {
        if let Some(handler) = self.handlers.get(&cmd.name) {
            handler(cmd)
        } else {
            Err(ExCommandError::UnknownCommand(cmd.name.clone()))
        }
    }
    
    /// Check if a command is registered
    pub fn has_command(&self, name: &str) -> bool {
        self.handlers.contains_key(name)
    }
}

impl Default for ExCommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_range_spec() {
        assert_eq!(RangeSpec::from_str(".").unwrap(), RangeSpec::CurrentLine);
        assert_eq!(RangeSpec::from_str("$").unwrap(), RangeSpec::LastLine);
        assert_eq!(RangeSpec::from_str("5").unwrap(), RangeSpec::LineNumber(5));
        assert_eq!(RangeSpec::from_str("'a").unwrap(), RangeSpec::Mark('a'));
        assert_eq!(RangeSpec::from_str("/pattern/").unwrap(), RangeSpec::Search("pattern/".to_string()));
        assert_eq!(RangeSpec::from_str("+3").unwrap(), RangeSpec::Offset(3));
        assert_eq!(RangeSpec::from_str("-2").unwrap(), RangeSpec::Offset(-2));
        
        assert!(RangeSpec::from_str("").is_err());
        assert!(RangeSpec::from_str("invalid").is_err());
    }
    
    #[test]
    fn test_parse_range() {
        let parser = ExCommandParser::new();
        
        // Test empty range
        let (range, rest) = parser.parse_range("write").unwrap();
        assert!(range.is_empty());
        assert_eq!(rest, "write");
        
        // Test single line range
        let (range, rest) = parser.parse_range("5write").unwrap();
        assert!(range.is_single_line());
        assert_eq!(range.start.unwrap(), RangeSpec::LineNumber(5));
        assert_eq!(rest, "write");
        
        // Test range with comma
        let (range, rest) = parser.parse_range("1,5write").unwrap();
        assert_eq!(range.start.unwrap(), RangeSpec::LineNumber(1));
        assert_eq!(range.end.unwrap(), RangeSpec::LineNumber(5));
        assert_eq!(rest, "write");
        
        // Test range with semicolon
        let (range, rest) = parser.parse_range("1;5write").unwrap();
        assert_eq!(range.start.unwrap(), RangeSpec::LineNumber(1));
        assert_eq!(range.end.unwrap(), RangeSpec::LineNumber(5));
        assert_eq!(rest, "write");
        
        // Test range with special characters
        let (range, rest) = parser.parse_range(".,+5write").unwrap();
        assert_eq!(range.start.unwrap(), RangeSpec::CurrentLine);
        assert_eq!(range.end.unwrap(), RangeSpec::Offset(5));
        assert_eq!(rest, "write");
        
        // Test range with marks
        let (range, rest) = parser.parse_range("'a,'bwrite").unwrap();
        assert_eq!(range.start.unwrap(), RangeSpec::Mark('a'));
        assert_eq!(range.end.unwrap(), RangeSpec::Mark('b'));
        assert_eq!(rest, "write");
    }
    
    #[test]
    fn test_parse_command_name() {
        let parser = ExCommandParser::new();
        
        // Test simple command
        let (name, rest) = parser.parse_command_name("write").unwrap();
        assert_eq!(name, "write");
        assert_eq!(rest, "");
        
        // Test command with arguments
        let (name, rest) = parser.parse_command_name("write file.txt").unwrap();
        assert_eq!(name, "write");
        assert_eq!(rest, " file.txt");
        
        // Test command with flags
        let (name, rest) = parser.parse_command_name("write!").unwrap();
        assert_eq!(name, "write");
        assert_eq!(rest, "!");
        
        // Test invalid command
        assert!(parser.parse_command_name("").is_err());
        assert!(parser.parse_command_name("!").is_err());
    }
    
    #[test]
    fn test_parse_flags() {
        let parser = ExCommandParser::new();
        
        // Test no flags
        let (flags, rest) = parser.parse_flags("file.txt").unwrap();
        assert!(!flags.force);
        assert!(!flags.print);
        assert!(!flags.list);
        assert!(!flags.number);
        assert_eq!(rest, "file.txt");
        
        // Test force flag
        let (flags, rest) = parser.parse_flags("! file.txt").unwrap();
        assert!(flags.force);
        assert!(!flags.print);
        assert!(!flags.list);
        assert!(!flags.number);
        assert_eq!(rest, " file.txt");
        
        // Test multiple flags
        let (flags, rest) = parser.parse_flags("!pl file.txt").unwrap();
        assert!(flags.force);
        assert!(flags.print);
        assert!(flags.list);
        assert!(!flags.number);
        assert_eq!(rest, " file.txt");
        
        // Test all flags
        let (flags, rest) = parser.parse_flags("!pl# file.txt").unwrap();
        assert!(flags.force);
        assert!(flags.print);
        assert!(flags.list);
        assert!(flags.number);
        assert_eq!(rest, " file.txt");
    }
    
    #[test]
    fn test_parse_args() {
        let parser = ExCommandParser::new();
        
        // Test no args
        let args = parser.parse_args("").unwrap();
        assert!(args.is_empty());
        
        // Test single arg
        let args = parser.parse_args("file.txt").unwrap();
        assert_eq!(args, vec!["file.txt"]);
        
        // Test multiple args
        let args = parser.parse_args("file1.txt file2.txt").unwrap();
        assert_eq!(args, vec!["file1.txt", "file2.txt"]);
        
        // Test quoted args
        let args = parser.parse_args("\"file with spaces.txt\"").unwrap();
        assert_eq!(args, vec!["file with spaces.txt"]);
        
        // Test mixed args
        let args = parser.parse_args("file1.txt \"file with spaces.txt\" file3.txt").unwrap();
        assert_eq!(args, vec!["file1.txt", "file with spaces.txt", "file3.txt"]);
        
        // Test escaped quotes
        let args = parser.parse_args("file1.txt \"file \\\"with\\\" quotes.txt\"").unwrap();
        assert_eq!(args, vec!["file1.txt", "file \"with\" quotes.txt"]);
    }
    
    #[test]
    fn test_parse_command() {
        let parser = ExCommandParser::new();
        
        // Test simple command
        let cmd = parser.parse("write").unwrap();
        assert_eq!(cmd.name, "write");
        assert!(cmd.range.is_empty());
        assert!(!cmd.flags.force);
        assert!(cmd.args.is_empty());
        
        // Test command with range
        let cmd = parser.parse("1,5write").unwrap();
        assert_eq!(cmd.name, "write");
        assert_eq!(cmd.range.start.unwrap(), RangeSpec::LineNumber(1));
        assert_eq!(cmd.range.end.unwrap(), RangeSpec::LineNumber(5));
        assert!(!cmd.flags.force);
        assert!(cmd.args.is_empty());
        
        // Test command with flags
        let cmd = parser.parse("write!").unwrap();
        assert_eq!(cmd.name, "write");
        assert!(cmd.range.is_empty());
        assert!(cmd.flags.force);
        assert!(cmd.args.is_empty());
        
        // Test command with args
        let cmd = parser.parse("write file.txt").unwrap();
        assert_eq!(cmd.name, "write");
        assert!(cmd.range.is_empty());
        assert!(!cmd.flags.force);
        assert_eq!(cmd.args, vec!["file.txt"]);
        
        // Test command with range, flags, and args
        let cmd = parser.parse("1,5write! file.txt").unwrap();
        assert_eq!(cmd.name, "write");
        assert_eq!(cmd.range.start.unwrap(), RangeSpec::LineNumber(1));
        assert_eq!(cmd.range.end.unwrap(), RangeSpec::LineNumber(5));
        assert!(cmd.flags.force);
        assert_eq!(cmd.args, vec!["file.txt"]);
        
        // Test command with alias
        let cmd = parser.parse("w").unwrap();
        assert_eq!(cmd.name, "write");
        assert!(cmd.range.is_empty());
        assert!(!cmd.flags.force);
        assert!(cmd.args.is_empty());
    }
    
    #[test]
    fn test_command_registry() {
        let mut registry = ExCommandRegistry::new();
        
        // Register a command
        registry.register("test", |cmd| {
            if cmd.args.is_empty() {
                Ok(())
            } else {
                Err(ExCommandError::InvalidArgument("No arguments expected".to_string()))
            }
        });
        
        // Test command execution
        let cmd = ExCommand::new("test", Range::new(None, None), CommandFlags::default(), Vec::new(), "test");
        assert!(registry.execute(&cmd).is_ok());
        
        // Test command with invalid args
        let cmd = ExCommand::new("test", Range::new(None, None), CommandFlags::default(), vec!["arg".to_string()], "test arg");
        assert!(registry.execute(&cmd).is_err());
        
        // Test unknown command
        let cmd = ExCommand::new("unknown", Range::new(None, None), CommandFlags::default(), Vec::new(), "unknown");
        assert!(registry.execute(&cmd).is_err());
    }
}