//! Color scheme module
//!
//! This module provides color scheme functionality for the editor.

use crate::syntax::{Style, Theme, TokenType};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::path::{Path, PathBuf};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;

/// Color scheme
#[derive(Debug, Clone)]
pub struct ColorScheme {
    /// Name
    pub name: String,
    /// Theme
    pub theme: Theme,
    /// Path
    pub path: Option<PathBuf>,
}

impl ColorScheme {
    /// Create a new color scheme
    pub fn new(name: &str, theme: Theme) -> Self {
        Self {
            name: name.to_string(),
            theme,
            path: None,
        }
    }

    /// Set the path
    pub fn set_path(&mut self, path: &Path) {
        self.path = Some(path.to_path_buf());
    }
}

/// Color scheme registry
#[derive(Debug)]
pub struct ColorSchemeRegistry {
    /// Color schemes
    pub schemes: HashMap<String, ColorScheme>,
    /// Current color scheme
    pub current: Option<String>,
}

impl ColorSchemeRegistry {
    /// Create a new color scheme registry
    pub fn new() -> Self {
        Self {
            schemes: HashMap::new(),
            current: None,
        }
    }

    /// Add a color scheme
    pub fn add(&mut self, scheme: ColorScheme) {
        let name = scheme.name.clone();
        self.schemes.insert(name.clone(), scheme);
        
        // Set as current if no current scheme
        if self.current.is_none() {
            self.current = Some(name);
        }
    }

    /// Get a color scheme by name
    pub fn get(&self, name: &str) -> Option<&ColorScheme> {
        self.schemes.get(name)
    }

    /// Get the current color scheme
    pub fn current(&self) -> Option<&ColorScheme> {
        self.current.as_ref().and_then(|name| self.schemes.get(name))
    }

    /// Set the current color scheme
    pub fn set_current(&mut self, name: &str) -> bool {
        if self.schemes.contains_key(name) {
            self.current = Some(name.to_string());
            true
        } else {
            false
        }
    }

    /// Load color schemes from a directory
    pub fn load_from_directory(&mut self, path: &Path) -> io::Result<usize> {
        if !path.exists() {
            return Ok(0);
        }

        let mut count = 0;

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "vim") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Some(scheme) = parse_vim_colorscheme(&content, &path) {
                        self.add(scheme);
                        count += 1;
                    }
                }
            }
        }

        Ok(count)
    }
}

/// Parse a Vim color scheme file
fn parse_vim_colorscheme(content: &str, path: &Path) -> Option<ColorScheme> {
    let mut name = path.file_stem()?.to_str()?.to_string();
    let mut theme = Theme::new(&name);
    let mut background = "dark";

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("\"") {
            continue;
        }

        if line.starts_with("let g:colors_name") {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() >= 2 {
                let name_part = parts[1].trim().trim_matches('"').trim_matches('\'');
                if !name_part.is_empty() {
                    name = name_part.to_string();
                    theme.name = name.clone();
                }
            }
        } else if line.starts_with("set background=") {
            background = line["set background=".len()..].trim();
        } else if line.starts_with("highlight ") || line.starts_with("hi ") {
            parse_highlight_line(line, &mut theme);
        }
    }

    let mut scheme = ColorScheme::new(&name, theme);
    scheme.set_path(path);

    Some(scheme)
}

/// Parse a highlight line
fn parse_highlight_line(line: &str, theme: &mut Theme) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }

    let group_name = parts[1];
    let token_type = match group_name {
        "Normal" => None, // Normal is used for default style
        "Comment" => Some(TokenType::Comment),
        "Constant" => Some(TokenType::Constant),
        "String" => Some(TokenType::String),
        "Character" => Some(TokenType::Character),
        "Number" => Some(TokenType::Number),
        "Boolean" => Some(TokenType::Constant),
        "Float" => Some(TokenType::Number),
        "Identifier" => Some(TokenType::Identifier),
        "Function" => Some(TokenType::Function),
        "Statement" => Some(TokenType::Statement),
        "Conditional" => Some(TokenType::Conditional),
        "Repeat" => Some(TokenType::Repeat),
        "Label" => Some(TokenType::Label),
        "Operator" => Some(TokenType::Operator),
        "Keyword" => Some(TokenType::Keyword),
        "Exception" => Some(TokenType::Keyword),
        "PreProc" => Some(TokenType::Preprocessor),
        "Include" => Some(TokenType::Include),
        "Define" => Some(TokenType::Define),
        "Macro" => Some(TokenType::Macro),
        "PreCondit" => Some(TokenType::PreCondit),
        "Type" => Some(TokenType::Type),
        "StorageClass" => Some(TokenType::StorageClass),
        "Structure" => Some(TokenType::Structure),
        "Typedef" => Some(TokenType::Typedef),
        "Special" => Some(TokenType::Special),
        "SpecialChar" => Some(TokenType::SpecialChar),
        "Tag" => Some(TokenType::Tag),
        "Delimiter" => Some(TokenType::Delimiter),
        "SpecialComment" => Some(TokenType::SpecialComment),
        "Debug" => Some(TokenType::Debug),
        "Underlined" => Some(TokenType::Underlined),
        "Error" => Some(TokenType::Error),
        "Todo" => Some(TokenType::Todo),
        _ => None,
    };

    let mut style = Style::new();

    for i in 2..parts.len() {
        let part = parts[i];
        if part.starts_with("guifg=") {
            let color = part["guifg=".len()..].trim_matches('"').trim_matches('\'');
            if !color.is_empty() && color != "NONE" {
                style = style.foreground(color);
            }
        } else if part.starts_with("guibg=") {
            let color = part["guibg=".len()..].trim_matches('"').trim_matches('\'');
            if !color.is_empty() && color != "NONE" {
                style = style.background(color);
            }
        } else if part.starts_with("gui=") {
            let attrs = part["gui=".len()..].trim_matches('"').trim_matches('\'');
            for attr in attrs.split(',') {
                match attr {
                    "bold" => style = style.bold(true),
                    "italic" => style = style.italic(true),
                    "underline" => style = style.underline(true),
                    "reverse" => style = style.reverse(true),
                    "standout" => style = style.standout(true),
                    "undercurl" => style = style.undercurl(true),
                    "strikethrough" => style = style.strikethrough(true),
                    _ => {}
                }
            }
        }
    }

    if let Some(token_type) = token_type {
        theme.set_style(token_type, style);
    } else if group_name == "Normal" {
        theme.set_default_style(style);
    }
}

// Global color scheme registry
static mut COLOR_SCHEME_REGISTRY: Option<Arc<Mutex<ColorSchemeRegistry>>> = None;

/// Initialize the color scheme registry
pub fn init_color_scheme_registry() {
    unsafe {
        if COLOR_SCHEME_REGISTRY.is_none() {
            let mut registry = ColorSchemeRegistry::new();
            
            // Add default color scheme
            let default_theme = crate::syntax::create_default_theme();
            let default_scheme = ColorScheme::new("default", default_theme);
            registry.add(default_scheme);
            
            COLOR_SCHEME_REGISTRY = Some(Arc::new(Mutex::new(registry)));
        }
    }
}

/// Get the color scheme registry
pub fn get_color_scheme_registry() -> Option<Arc<Mutex<ColorSchemeRegistry>>> {
    unsafe {
        COLOR_SCHEME_REGISTRY.clone()
    }
}

/// Load color schemes from a directory
pub fn load_color_schemes_from_directory(path: &Path) -> io::Result<usize> {
    let registry = match get_color_scheme_registry() {
        Some(registry) => registry,
        None => {
            init_color_scheme_registry();
            get_color_scheme_registry().unwrap()
        }
    };

    let mut registry = registry.lock().unwrap();
    registry.load_from_directory(path)
}

/// Load a color scheme from a file
pub fn load_color_scheme_from_file(path: &Path) -> io::Result<bool> {
    if !path.exists() {
        return Ok(false);
    }

    let content = fs::read_to_string(path)?;
    
    if let Some(scheme) = parse_vim_colorscheme(&content, path) {
        let registry = match get_color_scheme_registry() {
            Some(registry) => registry,
            None => {
                init_color_scheme_registry();
                get_color_scheme_registry().unwrap()
            }
        };

        let mut registry = registry.lock().unwrap();
        registry.add(scheme);
        
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Get a color scheme by name
pub fn get_color_scheme(name: &str) -> Option<ColorScheme> {
    let registry = match get_color_scheme_registry() {
        Some(registry) => registry,
        None => {
            init_color_scheme_registry();
            get_color_scheme_registry().unwrap()
        }
    };

    let registry = registry.lock().unwrap();
    registry.get(name).cloned()
}

/// Get the current color scheme
pub fn get_current_color_scheme() -> Option<ColorScheme> {
    let registry = match get_color_scheme_registry() {
        Some(registry) => registry,
        None => {
            init_color_scheme_registry();
            get_color_scheme_registry().unwrap()
        }
    };

    let registry = registry.lock().unwrap();
    registry.current().cloned()
}

/// Set the current color scheme
pub fn set_current_color_scheme(name: &str) -> bool {
    let registry = match get_color_scheme_registry() {
        Some(registry) => registry,
        None => {
            init_color_scheme_registry();
            get_color_scheme_registry().unwrap()
        }
    };

    let mut registry = registry.lock().unwrap();
    registry.set_current(name)
}

/// Apply a color scheme to the editor
pub fn apply_color_scheme(editor: &mut crate::editor::Editor, name: &str) -> Result<bool, io::Error> {
    // Check if the color scheme exists
    if !set_current_color_scheme(name) {
        return Ok(false);
    }

    // Get the color scheme
    let scheme = match get_color_scheme(name) {
        Some(scheme) => scheme,
        None => return Ok(false),
    };

    // Apply the color scheme to the editor
    editor.set_theme(Arc::new(scheme.theme));

    // Redraw the editor
    editor.redraw();

    Ok(true)
}

/// Create a default color scheme registry
pub fn create_default_registry() -> ColorSchemeRegistry {
    let mut registry = ColorSchemeRegistry::new();
    
    // Add default color scheme
    let default_theme = crate::syntax::create_default_theme();
    let default_scheme = ColorScheme::new("default", default_theme);
    registry.add(default_scheme);
    
    registry
}