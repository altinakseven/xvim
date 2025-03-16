//! Syntax highlighting integration for buffers
//!
//! This module integrates the syntax highlighting system with the buffer system,
//! allowing buffers to be displayed with syntax highlighting.

use std::sync::Arc;
use std::path::Path;

use crate::buffer::Buffer;
use crate::buffer::BufferResult;
use crate::syntax::{SyntaxDefinition, SyntaxHighlighter, SyntaxRegistry, Theme, Token};
use std::io::{BufReader, Read};
use std::fs::File;
use ropey::Rope;

/// Extension trait to add syntax highlighting methods to Buffer
pub trait BufferSyntaxExt {
    /// Set the syntax definition for this buffer
    fn set_syntax_definition(&mut self, definition: Arc<SyntaxDefinition>);
    
    /// Set the theme for this buffer
    fn set_theme(&mut self, theme: Arc<Theme>);
    
    /// Get the syntax highlighter for this buffer, if any
    fn syntax_highlighter(&self) -> Option<&SyntaxHighlighter>;
    
    /// Highlight a line of text
    fn highlight_line(&self, line_idx: usize) -> BufferResult<Vec<Token>>;
    
    /// Auto-detect and set the syntax definition based on file path or content
    fn auto_detect_syntax(&mut self, registry: &SyntaxRegistry);
}

/// Syntax highlighting data for a buffer
#[derive(Debug, Clone)]
pub struct BufferSyntax {
    /// The syntax highlighter, if any
    highlighter: Option<SyntaxHighlighter>,
}

impl BufferSyntax {
    /// Create a new buffer syntax
    pub fn new() -> Self {
        Self {
            highlighter: None,
        }
    }
    
    /// Set the syntax definition and theme
    pub fn set_highlighter(&mut self, definition: Arc<SyntaxDefinition>, theme: Arc<Theme>) {
        self.highlighter = Some(SyntaxHighlighter::new(definition, theme));
    }
    
    /// Get the syntax highlighter, if any
    pub fn highlighter(&self) -> Option<&SyntaxHighlighter> {
        self.highlighter.as_ref()
    }
}

impl Default for BufferSyntax {
    fn default() -> Self {
        Self::new()
    }
}

// Implement the BufferSyntaxExt trait for Buffer
impl BufferSyntaxExt for Buffer {
    fn set_syntax_definition(&mut self, definition: Arc<SyntaxDefinition>) {
        // If we already have a highlighter, update it with the new definition
        // Otherwise, create a new one with a default theme
        if let Some(highlighter) = &self.syntax.highlighter {
            self.syntax.set_highlighter(definition, highlighter.theme.clone());
        } else {
            // Create a default theme
            let theme = Arc::new(crate::syntax::create_default_theme());
            self.syntax.set_highlighter(definition, theme);
        }
    }
    
    fn set_theme(&mut self, theme: Arc<Theme>) {
        // If we already have a highlighter, update it with the new theme
        // Otherwise, do nothing (we need a syntax definition first)
        if let Some(highlighter) = &self.syntax.highlighter {
            self.syntax.set_highlighter(highlighter.definition.clone(), theme);
        }
    }
    
    fn syntax_highlighter(&self) -> Option<&SyntaxHighlighter> {
        self.syntax.highlighter()
    }
    
    fn highlight_line(&self, line_idx: usize) -> BufferResult<Vec<Token>> {
        // Get the line content
        let line = self.line(line_idx)?;
        
        // If we have a syntax highlighter, use it
        if let Some(highlighter) = self.syntax.highlighter() {
            Ok(highlighter.highlight_line(&line))
        } else {
            // No syntax highlighting, return an empty token list
            Ok(Vec::new())
        }
    }
    
    fn auto_detect_syntax(&mut self, registry: &SyntaxRegistry) {
        // Try to detect syntax based on file path
        if let Some(path) = self.file_path() {
            if let Some(definition) = registry.get_definition_for_file(path) {
                self.set_syntax_definition(definition);
                return;
            }
        }
        
        // If no syntax was detected based on file path, try to detect based on content
        // This is a simple implementation that just checks the first line for common patterns
        if let Ok(first_line) = self.line(0) {
            // Check for shebang
            if first_line.starts_with("#!") {
                if first_line.contains("python") {
                    if let Some(definition) = registry.get_definition_by_name("Python") {
                        self.set_syntax_definition(definition);
                        return;
                    }
                } else if first_line.contains("node") || first_line.contains("javascript") {
                    if let Some(definition) = registry.get_definition_by_name("JavaScript") {
                        self.set_syntax_definition(definition);
                        return;
                    }
                } else if first_line.contains("bash") || first_line.contains("sh") {
                    if let Some(definition) = registry.get_definition_by_name("Bash") {
                        self.set_syntax_definition(definition);
                        return;
                    }
                }
            }
            
            // Check for common file headers
            if first_line.contains("<?php") {
                if let Some(definition) = registry.get_definition_by_name("PHP") {
                    self.set_syntax_definition(definition);
                    return;
                }
            } else if first_line.contains("<?xml") || first_line.contains("<!DOCTYPE html") {
                if let Some(definition) = registry.get_definition_by_name("XML") {
                    self.set_syntax_definition(definition);
                    return;
                }
            }
            
            // Check for Rust files based on content
            if first_line.contains("fn ") {
                if let Some(definition) = registry.get_definition_by_name("Rust") {
                    self.set_syntax_definition(definition);
                    return;
                }
            }
        }
        
        // If we still haven't detected a syntax, check the file extension from the path
        if let Some(path) = self.file_path() {
            if let Some(extension) = path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    if ext_str == "rs" {
                        if let Some(definition) = registry.get_definition_by_name("Rust") {
                            self.set_syntax_definition(definition);
                            return;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{create_default_registry, create_default_theme, TokenType};
    use std::sync::Arc;
    
    #[test]
    fn test_buffer_syntax_highlighting() {
        // Create a buffer with Rust code
        let mut buffer = Buffer::new(1);
        buffer.insert(0, "fn main() {\n    println!(\"Hello, world!\");\n}").unwrap();
        
        // Create a syntax registry and get the Rust definition
        let registry = create_default_registry().unwrap();
        let rust_def = registry.get_definition_by_name("Rust").unwrap();
        
        // Set the syntax definition for the buffer
        buffer.set_syntax_definition(rust_def);
        
        // Highlight the first line
        let tokens = buffer.highlight_line(0).unwrap();
        
        // Verify that we got some tokens
        assert!(!tokens.is_empty());
        
        // Check that "fn" is highlighted as a keyword
        let fn_token = tokens.iter().find(|t| t.text == "fn").unwrap();
        assert_eq!(fn_token.token_type, TokenType::Keyword);
        
        // Check that "main" is highlighted as a function
        let main_token = tokens.iter().find(|t| t.text == "main").unwrap();
        assert_eq!(main_token.token_type, TokenType::Function);
    }
    
    #[test]
    fn test_auto_detect_syntax() {
        use tempfile::NamedTempFile;
        use std::io::Write;
        
        // Create a temporary Rust file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "fn main() {{\n    println!(\"Hello, world!\");\n}}").unwrap();
        
        // Create a buffer from the file
        let mut buffer = Buffer::from_file(1, temp_file.path()).unwrap();
        
        // Create a syntax registry
        let registry = create_default_registry().unwrap();
        
        // Auto-detect syntax
        buffer.auto_detect_syntax(&registry);
        
        // Verify that Rust syntax was detected
        assert!(buffer.syntax_highlighter().is_some());
        assert_eq!(buffer.syntax_highlighter().unwrap().definition.name, "Rust");
    }
}