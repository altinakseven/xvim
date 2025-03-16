//! Command module - Command parsing and execution
//!
//! This module handles the parsing and execution of Vim commands.

mod ex;
pub mod handlers;

pub use ex::{
    ExCommand, ExCommandError, ExCommandParser, ExCommandRegistry,
    ExCommandResult, Range, RangeSpec, CommandFlags
};
pub use handlers::{register_handlers, should_quit, reset_quit_flag, set_editor, handle_edit, handle_split};

/// Command types
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Ex command (commands that start with :)
    Ex(ExCommand),
    /// Normal mode command
    Normal(String),
    /// Visual mode command
    Visual(String),
    /// Insert mode command
    Insert(String),
}

/// Command parser
pub struct CommandParser {
    /// Ex command parser
    ex_parser: ExCommandParser,
}

impl CommandParser {
    /// Create a new command parser
    pub fn new() -> Self {
        Self {
            ex_parser: ExCommandParser::new(),
        }
    }

    /// Parse a command string
    pub fn parse(&self, input: &str) -> Option<Command> {
        let input = input.trim();
        
        if input.is_empty() {
            return None;
        }
        
        // Check if this is an ex command (starts with :)
        if input.starts_with(':') {
            let ex_input = &input[1..];
            match self.ex_parser.parse(ex_input) {
                Ok(ex_cmd) => Some(Command::Ex(ex_cmd)),
                Err(_) => None,
            }
        } else if input.starts_with('/') || input.starts_with('?') {
            // Search command (handled as a special ex command)
            let pattern = &input[1..];
            let mut cmd = ex::ExCommand::new(
                "search",
                ex::Range::new(None, None),
                ex::CommandFlags::default(),
                vec![pattern.to_string()],
                input
            );
            Some(Command::Ex(cmd))
        } else {
            // For now, treat other commands as normal mode commands
            Some(Command::Normal(input.to_string()))
        }
    }
    
    /// Parse an ex command string
    pub fn parse_ex(&self, input: &str) -> ExCommandResult<ExCommand> {
        self.ex_parser.parse(input)
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_ex_command() {
        let parser = CommandParser::new();
        
        // Test ex command parsing
        if let Some(Command::Ex(cmd)) = parser.parse(":write") {
            assert_eq!(cmd.name, "write");
            assert!(cmd.range.is_empty());
            assert!(!cmd.flags.force);
            assert!(cmd.args.is_empty());
        } else {
            panic!("Failed to parse ex command");
        }
        
        // Test ex command with range
        if let Some(Command::Ex(cmd)) = parser.parse(":1,5write") {
            assert_eq!(cmd.name, "write");
            assert_eq!(cmd.range.start.unwrap(), RangeSpec::LineNumber(1));
            assert_eq!(cmd.range.end.unwrap(), RangeSpec::LineNumber(5));
            assert!(!cmd.flags.force);
            assert!(cmd.args.is_empty());
        } else {
            panic!("Failed to parse ex command with range");
        }
        
        // Test ex command with flags
        if let Some(Command::Ex(cmd)) = parser.parse(":write!") {
            assert_eq!(cmd.name, "write");
            assert!(cmd.range.is_empty());
            assert!(cmd.flags.force);
            assert!(cmd.args.is_empty());
        } else {
            panic!("Failed to parse ex command with flags");
        }
        
        // Test ex command with args
        if let Some(Command::Ex(cmd)) = parser.parse(":write file.txt") {
            assert_eq!(cmd.name, "write");
            assert!(cmd.range.is_empty());
            assert!(!cmd.flags.force);
            assert_eq!(cmd.args, vec!["file.txt"]);
        } else {
            panic!("Failed to parse ex command with args");
        }
    }
    
    #[test]
    fn test_parse_normal_command() {
        let parser = CommandParser::new();
        
        // Test normal mode command
        if let Some(Command::Normal(cmd)) = parser.parse("dd") {
            assert_eq!(cmd, "dd");
        } else {
            panic!("Failed to parse normal mode command");
        }
        
        // Test normal mode command with count
        if let Some(Command::Normal(cmd)) = parser.parse("5dd") {
            assert_eq!(cmd, "5dd");
        } else {
            panic!("Failed to parse normal mode command with count");
        }
    }
    
    #[test]
    fn test_parse_search_command() {
        let parser = CommandParser::new();
        
        // Test search command
        if let Some(Command::Ex(cmd)) = parser.parse("/pattern") {
            assert_eq!(cmd.name, "search");
            assert!(cmd.range.is_empty());
            assert!(!cmd.flags.force);
            assert_eq!(cmd.args, vec!["pattern"]);
        } else {
            panic!("Failed to parse search command");
        }
    }
    
    #[test]
    fn test_parse_empty_command() {
        let parser = CommandParser::new();
        
        // Test empty command
        assert_eq!(parser.parse(""), None);
        assert_eq!(parser.parse("  "), None);
    }
}