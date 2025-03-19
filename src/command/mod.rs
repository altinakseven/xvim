//! Command module - Command parsing and execution
//!
//! This module handles the parsing and execution of Vim commands.

mod ex;
pub mod handlers;
pub mod additional_handlers;
pub mod quickfix_handlers;
pub mod fold_handlers;
pub mod tag_handlers;
pub mod window_handlers;
pub mod search_handlers;
pub mod macro_handlers;
pub mod undo_handlers;
pub mod mark_handlers;
pub mod completion_handlers;
pub mod spell_handlers;
pub mod diff_handlers;
pub mod session_handlers;
pub mod autocmd_handlers;
pub mod terminal_handlers;
pub mod vim_commands;
pub mod config_handlers;
pub mod set_handlers;
pub mod nohlsearch_handlers;

pub use ex::{
    ExCommand, ExCommandError, ExCommandParser, ExCommandRegistry,
    ExCommandResult, Range, RangeSpec, CommandFlags
};
pub use handlers::{register_handlers, should_quit, reset_quit_flag, set_editor, handle_edit, handle_split};
pub use additional_handlers::{register_additional_handlers, add_to_history};
pub use quickfix_handlers::{register_quickfix_handlers, init_quickfix_manager};
pub use fold_handlers::{register_fold_handlers, init_fold_manager};
pub use tag_handlers::{register_tag_handlers, init_tag_database};
pub use window_handlers::{register_window_handlers, init_window_manager};
pub use search_handlers::{register_search_handlers, init_search_manager};
pub use macro_handlers::{register_macro_handlers, init_macro_manager, record_keystroke, is_recording, is_executing};
pub use undo_handlers::{register_undo_handlers, init_undo_manager};
pub use mark_handlers::{register_mark_handlers, init_mark_manager, set_mark, get_mark, add_jump, jump_to, jump_back, jump_forward, auto_mark, get_jump_list, get_jump_position};
pub use completion_handlers::{register_completion_handlers, init_completion_manager, start_completion, next_completion, prev_completion, current_completion, cancel_completion, accept_completion, find_completions, register_user_defined_completion, register_omni_completion, add_keyword_dictionary, remove_keyword_dictionary, set_completion_options, get_completion_options, CompletionType, CompletionItem, CompletionContext, CompletionOptions};
pub use spell_handlers::{register_spell_handlers, init_spell_manager, enable_spell_checking, disable_spell_checking, toggle_spell_checking, check_spelling, get_spell_errors, add_word_to_dictionary, add_bad_word, get_suggestions, check_word, set_spell_options, get_spell_options, SpellError, SpellSuggestion, SpellOptions, SpellErrorType};
pub use diff_handlers::{register_diff_handlers, init_diff_manager, enable_diff_mode, disable_diff_mode, toggle_diff_mode, update_diffs, next_change, prev_change, DiffChange, DiffHunk, DiffOptions, DiffAlgorithm, DiffChangeType};
pub use session_handlers::{register_session_handlers, init_session_manager, save_session, load_session, check_auto_save, set_session_options, get_session_options, set_auto_save, get_auto_save, set_auto_save_interval, get_auto_save_interval, get_current_session, SessionOptions};
pub use autocmd_handlers::{register_autocmd_handlers, init_autocmd_manager, trigger_event, add_autocmd, remove_autocmds, create_group, delete_group, set_current_group, get_autocmds, get_all_autocmds, get_groups, get_current_group, AutocmdEvent, Autocmd, AutocmdPattern, AutocmdCommand};
pub use terminal_handlers::{register_terminal_handlers, init_terminal_manager, create_terminal, kill_terminal, write_input, resize_terminal, set_current_terminal, get_terminal, get_current_terminal, get_terminals, get_current_terminal_id, TerminalBuffer, TerminalAction};
pub use vim_commands::register_vim_commands;
pub use config_handlers::{register_config_handlers, init_config_manager, get_config_manager};
pub use set_handlers::{register_set_handlers, init_set_manager, get_bool_option, get_string_option, get_number_option, set_bool_option, set_string_option, set_number_option};
pub use nohlsearch_handlers::{register_nohlsearch_handlers};

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