//! xvim - A ground-up rewrite of the Vim text editor in pure Rust with a WASM plugin system

pub mod buffer;
pub mod command;
pub mod config;
pub mod cursor;
pub mod editor;
pub mod keymap;
pub mod r#macro;
pub mod mark;
pub mod mode;
pub mod operator;
pub mod plugin;
pub mod register;
pub mod search;
pub mod selection;
pub mod syntax;
pub mod text_object;
pub mod ui;
pub mod visual;

// Re-export commonly used types
pub use buffer::{Buffer, BufferManager, BufferResult, BufferError};
pub use command::{Command, CommandParser};
pub use editor::{Editor, EditorResult, EditorError};
pub use r#macro::{MacroRecorder, MacroPlayer, MacroRecorderState};
pub use mode::{Mode, ModeManager};
pub use operator::{Operator, OperatorTarget, OperatorState, OperatorManager};
pub use register::{RegisterManager, RegisterType, RegisterContent};
pub use search::{SearchState, SearchDirection, SearchFunctions};
pub use visual::{VisualMode, VisualState, VisualFunctions, VisualArea};