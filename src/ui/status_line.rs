//! Status line implementation for the terminal UI
//!
//! This module provides a more feature-rich status line for the editor,
//! displaying information about the current buffer, cursor position, and editor state.

use std::io::{self, Write};
use std::path::Path;
use crossterm::{
    cursor,
    style::{self, Color, Stylize},
    execute,
};
use crate::buffer::{Buffer, BufferType};
use crate::mode::Mode;

/// Status line renderer
pub struct StatusLine;

impl StatusLine {
    /// Create a new status line renderer
    pub fn new() -> Self {
        Self
    }
    
    /// Render the status line for a window
    pub fn render_window_status_line(
        &self,
        stdout: &mut io::Stdout,
        window_rect: &crate::ui::window::WindowRect,
        buffer: &Buffer,
        mode: Mode,
        cursor_line: usize,
        cursor_column: usize,
    ) -> crate::ui::UiResult<()> {
        // Position the cursor at the status line
        let status_y = window_rect.y + window_rect.height - 1;
        execute!(stdout, cursor::MoveTo(window_rect.x, status_y))?;
        
        // Set the status line color based on mode
        let (bg_color, fg_color) = self.get_mode_colors(mode);
        execute!(
            stdout,
            style::SetBackgroundColor(bg_color),
            style::SetForegroundColor(fg_color)
        )?;
        
        // Render the mode indicator with a different background
        let mode_name = mode.name().to_uppercase();
        write!(stdout, " {} ", mode_name)?;
        
        // Reset to standard status line colors
        execute!(
            stdout,
            style::SetBackgroundColor(Color::Blue),
            style::SetForegroundColor(Color::White)
        )?;
        
        // Get file information
        let file_name = buffer.name();
        let file_type = self.get_file_type(buffer);
        let encoding = "UTF-8"; // Assuming UTF-8 for now
        let line_ending = "LF";  // Assuming LF for now
        
        // Get buffer state indicators
        let modified_indicator = if buffer.is_modified() { "[+]" } else { "" };
        let readonly_indicator = if buffer.is_read_only() { "[RO]" } else { "" };
        
        // Calculate percentage through file
        let percentage = if buffer.line_count() > 0 {
            ((cursor_line as f64 + 1.0) / buffer.line_count() as f64 * 100.0).round() as usize
        } else {
            0
        };
        
        // Format position information
        let position_info = format!(
            "{}:{} | {}/{}L ({}%)",
            cursor_line + 1,
            cursor_column + 1,
            cursor_line + 1,
            buffer.line_count(),
            percentage
        );
        
        // Format file information
        let file_info = format!(
            "{}{}{} | {} | {}",
            file_name,
            modified_indicator,
            readonly_indicator,
            file_type,
            encoding
        );
        
        // Calculate available space
        let available_width = window_rect.width as usize - mode_name.len() - 4; // Account for mode indicator and padding
        
        // Determine how to split the available space
        let file_info_width = available_width / 2;
        let position_info_width = available_width - file_info_width;
        
        // Truncate file info if needed
        let file_info_display = if file_info.len() > file_info_width {
            format!("{}…", &file_info[0..file_info_width.saturating_sub(1)])
        } else {
            file_info
        };
        
        // Add padding to file info
        let file_info_padding = " ".repeat(file_info_width.saturating_sub(file_info_display.len()));
        
        // Write file info
        write!(stdout, "{}{}", file_info_display, file_info_padding)?;
        
        // Write position info with right alignment
        let position_padding = " ".repeat(position_info_width.saturating_sub(position_info.len()));
        write!(stdout, "{}{}", position_padding, position_info)?;
        
        // Reset colors
        execute!(stdout, style::ResetColor)?;
        
        Ok(())
    }
    
    /// Get the file type based on buffer type or file extension
    fn get_file_type(&self, buffer: &Buffer) -> &'static str {
        match buffer.buffer_type() {
            BufferType::Normal => {
                if let Some(path) = buffer.file_path() {
                    self.get_file_type_from_extension(path)
                } else {
                    "TXT"
                }
            },
            BufferType::NoFile => "TXT",
            BufferType::QuickFix => "QFIX",
            BufferType::Help => "HELP",
            BufferType::Terminal => "TERM",
            BufferType::Prompt => "PROMPT",
            BufferType::Popup => "POPUP",
        }
    }
    
    /// Get the file type from the file extension
    fn get_file_type_from_extension(&self, path: &Path) -> &'static str {
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            match extension.to_lowercase().as_str() {
                "rs" => "RUST",
                "py" => "PYTHON",
                "js" => "JS",
                "ts" => "TS",
                "html" => "HTML",
                "css" => "CSS",
                "json" => "JSON",
                "md" => "MARKDOWN",
                "txt" => "TXT",
                "c" => "C",
                "cpp" | "cc" | "cxx" => "C++",
                "h" | "hpp" => "H",
                "java" => "JAVA",
                "go" => "GO",
                "rb" => "RUBY",
                "php" => "PHP",
                "sh" => "SHELL",
                "toml" => "TOML",
                "yaml" | "yml" => "YAML",
                "xml" => "XML",
                _ => "TXT",
            }
        } else {
            "TXT"
        }
    }
    
    /// Get the colors for the mode indicator
    fn get_mode_colors(&self, mode: Mode) -> (Color, Color) {
        match mode {
            Mode::Normal => (Color::DarkGreen, Color::White),
            Mode::Insert => (Color::DarkBlue, Color::White),
            Mode::Visual => (Color::DarkMagenta, Color::White),
            Mode::VisualLine => (Color::Magenta, Color::White),
            Mode::VisualBlock => (Color::DarkMagenta, Color::White),
            Mode::Command => (Color::DarkYellow, Color::White),
            Mode::Replace => (Color::DarkRed, Color::White),
            Mode::Terminal => (Color::DarkCyan, Color::White),
            Mode::OperatorPending => (Color::DarkRed, Color::White),
        }
    }
}