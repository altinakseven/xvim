//! UI module - Terminal user interface for xvim
//!
//! This module handles the terminal rendering and user interface components.

pub mod syntax_renderer;
pub mod status_line;
pub mod tab;
pub mod window;

use std::io::{self, Write};
use std::error::Error;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{self, Color, Stylize},
    terminal::{self, ClearType},
};

use crate::buffer::{Buffer, BufferSyntaxExt};
use crate::selection::Selection;
use crate::syntax::{SyntaxHighlighter, Theme, TokenType};
use crate::cursor::CursorPosition;
use crate::mode::Mode;
use self::syntax_renderer::SyntaxRenderer;
use self::status_line::StatusLine;
use self::tab::{TabManager, Tab, TabId};
use self::window::{WindowManager, WindowRect, SplitDirection, Window};

/// UI-related errors
#[derive(Debug)]
pub enum UiError {
    /// I/O error
    Io(io::Error),
    /// Terminal error
    Terminal(String),
    /// Other errors
    Other(String),
}

impl std::fmt::Display for UiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UiError::Io(err) => write!(f, "I/O error: {}", err),
            UiError::Terminal(msg) => write!(f, "Terminal error: {}", msg),
            UiError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for UiError {}

impl From<io::Error> for UiError {
    fn from(err: io::Error) -> Self {
        UiError::Io(err)
    }
}

/// Result type for UI operations
pub type UiResult<T> = Result<T, UiError>;

/// Terminal UI manager
#[derive(Clone)]
pub struct TerminalUi {
    /// Terminal size
    size: (u16, u16),
    /// Whether the terminal is in raw mode
    raw_mode: bool,
    /// Whether the terminal is in alternate screen
    alternate_screen: bool,
    /// Syntax renderer
    syntax_renderer: SyntaxRenderer,
    /// Status line renderer
    status_line: StatusLine,
    /// Tab manager
    tab_manager: TabManager,
}

impl TerminalUi {
    /// Create a new terminal UI manager
    pub fn new() -> UiResult<Self> {
        let size = terminal::size()?;
        
        Ok(Self {
            size,
            raw_mode: false,
            alternate_screen: false,
            syntax_renderer: SyntaxRenderer::new(),
            status_line: StatusLine::new(),
            tab_manager: TabManager::new(),
        })
    }
    
    /// Initialize the terminal
    pub fn init(&mut self) -> UiResult<()> {
        // Enter raw mode
        terminal::enable_raw_mode()?;
        self.raw_mode = true;
        
        // Enter alternate screen
        execute!(
            io::stdout(),
            terminal::EnterAlternateScreen,
            cursor::Hide
        )?;
        self.alternate_screen = true;
        
        // Clear the screen
        execute!(
            io::stdout(),
            terminal::Clear(ClearType::All)
        )?;
        
        Ok(())
    }
    
    /// Clean up the terminal
    pub fn cleanup(&mut self) -> UiResult<()> {
        // Restore terminal state
        if self.alternate_screen {
            execute!(
                io::stdout(),
                terminal::LeaveAlternateScreen,
                cursor::Show
            )?;
            self.alternate_screen = false;
        }
        
        if self.raw_mode {
            terminal::disable_raw_mode()?;
            self.raw_mode = false;
        }
        
        Ok(())
    }
    
    /// Get the terminal size
    pub fn size(&self) -> (u16, u16) {
        self.size
    }
    
    /// Update the terminal size
    pub fn update_size(&mut self) -> UiResult<()> {
        self.size = terminal::size()?;
        Ok(())
    }
    
    /// Wait for a key event
    pub fn read_key(&self) -> UiResult<KeyEvent> {
        loop {
            if let Event::Key(key) = event::read()? {
                return Ok(key);
            }
        }
    }
    
    /// Check if a key event is available
    pub fn poll_key(&self, timeout_ms: u64) -> UiResult<Option<KeyEvent>> {
        if event::poll(std::time::Duration::from_millis(timeout_ms))? {
            if let Event::Key(key) = event::read()? {
                return Ok(Some(key));
            }
        }
        
        Ok(None)
    }
    
    /// Render a buffer to the terminal
    pub fn render_buffer(
        &self,
        buffer: &Buffer,
        start_line: usize,
        mode: Mode,
        cursor_pos: CursorPosition,
        selection: Option<&Selection>,
    ) -> UiResult<()> {
        // Get the syntax highlighter from the buffer if available
        let highlighter = buffer.syntax_highlighter();
        
        // Get the theme from the highlighter if available
        let theme = highlighter.map(|h| &h.theme);
        
        self.render_buffer_with_highlighting(buffer, start_line, mode, highlighter, theme.map(|v| &**v), cursor_pos, selection)
    }
    
    /// Render a buffer to the terminal with syntax highlighting
    pub fn render_buffer_with_highlighting(
        &self,
        buffer: &Buffer,
        start_line: usize,
        mode: Mode,
        highlighter: Option<&SyntaxHighlighter>,
        theme: Option<&Theme>,
        cursor_pos: CursorPosition,
        selection: Option<&Selection>,
    ) -> UiResult<()> {
        let (width, height) = self.size;
        let visible_lines = height as usize - 2; // Reserve space for status line and command line
        
        // Clear the screen
        execute!(
            io::stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        
        // Render buffer content
        let mut stdout = io::stdout();
        
        for i in 0..visible_lines {
            let line_idx = start_line + i;
            
            if line_idx < buffer.line_count() {
                // Render the line with line number
                execute!(stdout, cursor::MoveTo(0, i as u16))?;
                write!(stdout, "{:4} ", line_idx + 1)?;
                
                // Check if this is the cursor line
                let is_cursor_line = line_idx == cursor_pos.line;
                
                // Get the line content
                if let Ok(line) = buffer.line(line_idx) {
                    // Use the syntax renderer to render the line with highlighting
                    self.syntax_renderer.render_line(&mut stdout, buffer, line_idx, 0, width as usize - 5)?;
                    
                    // If this is the cursor line, position the cursor
                    if is_cursor_line {
                        let cursor_x = cursor_pos.column + 5; // +5 for line number and space
                        execute!(stdout, cursor::MoveTo(cursor_x as u16, i as u16))?;
                        
                        // Highlight the cursor position
                        if cursor_pos.column < line.len() {
                            let ch = line.chars().nth(cursor_pos.column).unwrap_or(' ');
                            execute!(stdout, style::SetBackgroundColor(Color::White))?;
                            execute!(stdout, style::SetForegroundColor(Color::Black))?;
                            write!(stdout, "{}", ch)?;
                            execute!(stdout, style::ResetColor)?;
                        } else {
                            // Cursor at end of line
                            execute!(stdout, style::SetBackgroundColor(Color::White))?;
                            execute!(stdout, style::SetForegroundColor(Color::Black))?;
                            write!(stdout, " ")?;
                            execute!(stdout, style::ResetColor)?;
                        }
                    }
                    
                    // Handle selection highlighting if needed
                    if let Some(_sel) = selection {
                        // Implement selection highlighting here
                        // This would need to iterate through the line and highlight selected characters
                    }
                    
                    writeln!(stdout)?;
                } else {
                    writeln!(stdout, "~")?;
                }
            } else {
                writeln!(stdout, "~")?;
            }
        }
        
        // Create a window rect for the status line
        let status_rect = WindowRect {
            x: 0,
            y: height - 2,
            width,
            height: 1,
        };
        
        // Render the enhanced status line
        self.status_line.render_window_status_line(
            &mut stdout,
            &status_rect,
            buffer,
            mode,
            cursor_pos.line,
            cursor_pos.column,
        )?;
        
        // Render command line
        execute!(
            stdout,
            cursor::MoveTo(0, height - 1)
        )?;
        
        if mode == Mode::Command {
            write!(stdout, ":")?;
        }
        
        stdout.flush()?;
        
        Ok(())
    }
}

impl Drop for TerminalUi {
    fn drop(&mut self) {
        // Ensure terminal is restored on drop
        let _ = self.cleanup();
    }
}

// Include window management methods
include!("window_methods.rs");

// Include tab management methods
include!("tab_methods.rs");