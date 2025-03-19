//! Syntax highlighting renderer for the terminal UI
//!
//! This module provides functionality to render syntax-highlighted text
//! in the terminal UI.

use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor, SetAttribute, Attribute};
use std::io::Write;
use crossterm::Result;
use std::io::Write;
use std::io::{Stdout, Write};
use std::io::Write;

use crate::buffer::{Buffer, BufferSyntaxExt};
use std::io::Write;
use crate::syntax::{Token, TokenType, Style};
use std::io::Write;

/// Renders syntax-highlighted text to the terminal
#[derive(Clone)]
pub struct SyntaxRenderer;

impl SyntaxRenderer {
    /// Create a new syntax renderer
    pub fn new() -> Self {
        Self
    }
    
    /// Render a line of text with syntax highlighting
    pub fn render_line<W: Write>(
        &self,
        writer: &mut W,
        buffer: &Buffer,
        line_idx: usize,
        start_col: usize,
        width: usize,
    ) -> Result<()> {
        // Get the line content
        let line = match buffer.line(line_idx) {
            Ok(line) => line,
            Err(_) => return Ok(()), // Line doesn't exist
        };
        
        // If the line is empty or start_col is beyond the line length, just return
        if line.is_empty() || start_col >= line.len() {
            return Ok(());
        }
        
        // Calculate the visible portion of the line
        let visible_end = std::cmp::min(start_col + width, line.len());
        let visible_text = &line[start_col..visible_end];
        
        // Get syntax highlighting tokens for the line
        let tokens = match buffer.highlight_line(line_idx) {
            Ok(tokens) => tokens,
            Err(_) => {
                // If highlighting fails, just render the text without highlighting
                write!(writer, "{}", visible_text)?;
                return Ok(());
            }
        };
        
        // If there are no tokens or syntax highlighting is not enabled,
        // just render the text without highlighting
        if tokens.is_empty() {
            write!(writer, "{}", visible_text)?;
            return Ok(());
        }
        
        // Filter tokens to only include those in the visible range
        let visible_tokens: Vec<&Token> = tokens.iter()
            .filter(|token| {
                // Token is visible if any part of it is in the visible range
                token.end > start_col && token.start < visible_end
            })
            .collect();
        
        // If there are no visible tokens, just render the text without highlighting
        if visible_tokens.is_empty() {
            write!(writer, "{}", visible_text)?;
            return Ok(());
        }
        
        // Render each visible token with its style
        let mut current_pos = start_col;
        for token in visible_tokens {
            // If there's a gap between the current position and the token start,
            // render that text without highlighting
            if current_pos < token.start {
                let gap_text = &line[current_pos..token.start];
                write!(writer, "{}", gap_text)?;
                current_pos = token.start;
            }
            
            // Calculate the visible portion of the token
            let token_start = std::cmp::max(token.start, start_col);
            let token_end = std::cmp::min(token.end, visible_end);
            
            if token_start < token_end {
                // Get the visible text for this token
                let token_text = &line[token_start..token_end];
                
                // Apply the style for this token type
                if let Some(highlighter) = buffer.syntax_highlighter() {
                    if let Some(style) = highlighter.theme.get_style(&token.token_type) {
                        self.apply_style(writer, style, token_text)?;
                    } else {
                        // No style for this token type, render without highlighting
                        write!(writer, "{}", token_text)?;
                    }
                } else {
                    // No highlighter, render without highlighting
                    write!(writer, "{}", token_text)?;
                }
                
                current_pos = token_end;
            }
        }
        
        // If there's text after the last token, render it without highlighting
        if current_pos < visible_end {
            let remaining_text = &line[current_pos..visible_end];
            write!(writer, "{}", remaining_text)?;
        }
        
        Ok(())
    }
    
    /// Apply a style to text
    fn apply_style<W: Write>(&self, writer: &mut W, style: &Style, text: &str) -> Result<()> {
        // Apply foreground color
        if let Some(fg) = style.foreground {
            write!(writer, "{}", SetForegroundColor(fg))?;
        }
        
        // Apply background color
        if let Some(bg) = style.background {
            write!(writer, "{}", SetBackgroundColor(bg))?;
        }
        
        // Apply attributes
        for attr in &style.attributes {
            write!(writer, "{}", SetAttribute(*attr))?;
        }
        
        // Write the text
        write!(writer, "{}", text)?;
        
        // Reset attributes
        write!(writer, "{}", SetAttribute(Attribute::Reset))?;
        
        Ok(())
    }
}

impl Default for SyntaxRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{create_default_registry, create_default_theme};
    
    #[test]
    fn test_render_line() {
        // This test is limited since we can't easily test terminal output
        // But we can at least verify that the code runs without errors
        
        // Create a buffer with Rust code
        let mut buffer = Buffer::new(1);
        buffer.insert(0, "fn main() {\n    println!(\"Hello, world!\");\n}").unwrap();
        
        // Set up syntax highlighting
        let registry = create_default_registry().unwrap();
        let rust_def = registry.get_definition_by_name("Rust").unwrap();
        buffer.set_syntax_definition(rust_def);
        
        // Create a syntax renderer
        let renderer = SyntaxRenderer::new();
        
        // Create a mock writer
        let mut output = Vec::new();
        
        // Render a line
        renderer.render_line(&mut output, &buffer, 0, 0, 80).unwrap();
        
        // Verify that something was written
        assert!(!output.is_empty());
    }
}