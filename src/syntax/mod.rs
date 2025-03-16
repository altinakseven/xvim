//! Syntax highlighting module for xvim
//!
//! This module provides syntax highlighting functionality for various programming languages
//! and file types. It uses a rule-based approach to identify and highlight different
//! elements of the code.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use crossterm::style::{Color, Attribute};
use regex::Regex;

/// A token type represents a specific element in the code
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    /// Keywords in the language
    Keyword,
    /// Identifiers (variable names, function names, etc.)
    Identifier,
    /// String literals
    String,
    /// Character literals
    Character,
    /// Number literals
    Number,
    /// Comments
    Comment,
    /// Preprocessor directives
    Preprocessor,
    /// Operators
    Operator,
    /// Delimiters (parentheses, brackets, braces, etc.)
    Delimiter,
    /// Type names
    Type,
    /// Function names
    Function,
    /// Macros
    Macro,
    /// Constants
    Constant,
    /// Special variables
    Special,
    /// Error highlighting
    Error,
    /// Warning highlighting
    Warning,
    /// Default text
    Default,
}

/// A style defines how a token should be displayed
#[derive(Debug, Clone)]
pub struct Style {
    /// Foreground color
    pub foreground: Option<Color>,
    /// Background color
    pub background: Option<Color>,
    /// Text attributes (bold, italic, etc.)
    pub attributes: Vec<Attribute>,
}

impl Style {
    /// Create a new style with the given foreground color
    pub fn new(foreground: Color) -> Self {
        Self {
            foreground: Some(foreground),
            background: None,
            attributes: Vec::new(),
        }
    }

    /// Add a background color to the style
    pub fn with_background(mut self, background: Color) -> Self {
        self.background = Some(background);
        self
    }

    /// Add an attribute to the style
    pub fn with_attribute(mut self, attribute: Attribute) -> Self {
        self.attributes.push(attribute);
        self
    }
}

/// A theme defines the styles for different token types
#[derive(Debug, Clone)]
pub struct Theme {
    /// Name of the theme
    pub name: String,
    /// Styles for different token types
    pub styles: HashMap<TokenType, Style>,
}

impl Theme {
    /// Create a new theme with the given name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            styles: HashMap::new(),
        }
    }

    /// Set the style for a token type
    pub fn set_style(&mut self, token_type: TokenType, style: Style) {
        self.styles.insert(token_type, style);
    }

    /// Get the style for a token type
    pub fn get_style(&self, token_type: &TokenType) -> Option<&Style> {
        self.styles.get(token_type)
    }
}

/// A token represents a piece of text with a specific type
#[derive(Debug, Clone)]
pub struct Token {
    /// The type of the token
    pub token_type: TokenType,
    /// The start position of the token (character index)
    pub start: usize,
    /// The end position of the token (character index)
    pub end: usize,
    /// The text of the token
    pub text: String,
}

impl Token {
    /// Create a new token
    pub fn new(token_type: TokenType, start: usize, end: usize, text: String) -> Self {
        Self {
            token_type,
            start,
            end,
            text,
        }
    }
}

/// A syntax rule defines how to identify a specific token type
#[derive(Debug, Clone)]
pub struct SyntaxRule {
    /// The type of token this rule identifies
    pub token_type: TokenType,
    /// The regular expression pattern to match
    pub pattern: Regex,
}

impl SyntaxRule {
    /// Create a new syntax rule
    pub fn new(token_type: TokenType, pattern: &str) -> Result<Self, regex::Error> {
        Ok(Self {
            token_type,
            pattern: Regex::new(pattern)?,
        })
    }
}

/// A syntax definition defines the rules for a specific language or file type
#[derive(Debug, Clone)]
pub struct SyntaxDefinition {
    /// Name of the language or file type
    pub name: String,
    /// File extensions associated with this syntax
    pub extensions: Vec<String>,
    /// Rules for identifying tokens
    pub rules: Vec<SyntaxRule>,
}

impl SyntaxDefinition {
    /// Create a new syntax definition
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            extensions: Vec::new(),
            rules: Vec::new(),
        }
    }

    /// Add a file extension to the syntax definition
    pub fn add_extension(&mut self, extension: &str) {
        self.extensions.push(extension.to_string());
    }

    /// Add a rule to the syntax definition
    pub fn add_rule(&mut self, rule: SyntaxRule) {
        self.rules.push(rule);
    }

    /// Check if the syntax definition applies to a file
    pub fn applies_to_file<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();
        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                return self.extensions.iter().any(|e| e == ext_str);
            }
        }
        false
    }
}

/// A syntax registry manages all available syntax definitions
#[derive(Debug, Clone)]
pub struct SyntaxRegistry {
    /// Available syntax definitions
    pub definitions: Vec<Arc<SyntaxDefinition>>,
}

impl SyntaxRegistry {
    /// Create a new syntax registry
    pub fn new() -> Self {
        Self {
            definitions: Vec::new(),
        }
    }

    /// Add a syntax definition to the registry
    pub fn add_definition(&mut self, definition: SyntaxDefinition) {
        self.definitions.push(Arc::new(definition));
    }

    /// Get a syntax definition for a file
    pub fn get_definition_for_file<P: AsRef<Path>>(&self, path: P) -> Option<Arc<SyntaxDefinition>> {
        let path = path.as_ref();
        for definition in &self.definitions {
            if definition.applies_to_file(path) {
                return Some(definition.clone());
            }
        }
        None
    }

    /// Get a syntax definition by name
    pub fn get_definition_by_name(&self, name: &str) -> Option<Arc<SyntaxDefinition>> {
        for definition in &self.definitions {
            if definition.name == name {
                return Some(definition.clone());
            }
        }
        None
    }
}

/// A syntax highlighter applies syntax rules to text
#[derive(Debug, Clone)]
pub struct SyntaxHighlighter {
    /// The syntax definition to use
    pub definition: Arc<SyntaxDefinition>,
    /// The theme to use for highlighting
    pub theme: Arc<Theme>,
}

impl SyntaxHighlighter {
    /// Create a new syntax highlighter
    pub fn new(definition: Arc<SyntaxDefinition>, theme: Arc<Theme>) -> Self {
        Self {
            definition,
            theme,
        }
    }

    /// Highlight a line of text
    pub fn highlight_line(&self, line: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut processed = vec![false; line.len()];

        // Apply each rule in order
        for rule in &self.definition.rules {
            for cap in rule.pattern.captures_iter(line) {
                // If there are capture groups, use the first one
                let (start, end, text) = if cap.len() > 1 {
                    // Get the first capture group
                    let m = cap.get(1).unwrap();
                    (m.start(), m.end(), m.as_str().to_string())
                } else {
                    // Use the entire match
                    let m = cap.get(0).unwrap();
                    (m.start(), m.end(), m.as_str().to_string())
                };

                // Check if this range has already been processed
                let already_processed = (start..end).any(|i| i < processed.len() && processed[i]);
                if already_processed {
                    continue;
                }

                // Mark this range as processed
                for i in start..end {
                    if i < processed.len() {
                        processed[i] = true;
                    }
                }

                // Add the token
                tokens.push(Token::new(rule.token_type.clone(), start, end, text));
            }
        }

        // Add any unprocessed text as default tokens, skipping whitespace
        let mut start = 0;
        while start < line.len() {
            if !processed[start] {
                // Skip whitespace
                if line[start..].chars().next().unwrap().is_whitespace() {
                    processed[start] = true;
                    start += 1;
                    continue;
                }

                let mut end = start + 1;
                while end < line.len() && !processed[end] && !line[end..].chars().next().unwrap().is_whitespace() {
                    end += 1;
                }

                let text = line[start..end].to_string();
                tokens.push(Token::new(TokenType::Default, start, end, text));

                // Mark this range as processed
                for i in start..end {
                    processed[i] = true;
                }

                start = end;
            } else {
                start += 1;
            }
        }

        // Sort tokens by start position
        tokens.sort_by_key(|t| t.start);

        tokens
    }
}

/// Create a default theme with sensible colors
pub fn create_default_theme() -> Theme {
    let mut theme = Theme::new("Default");

    // Set up default styles
    theme.set_style(TokenType::Keyword, Style::new(Color::Blue));
    theme.set_style(TokenType::Identifier, Style::new(Color::White));
    theme.set_style(TokenType::String, Style::new(Color::Green));
    theme.set_style(TokenType::Character, Style::new(Color::Green));
    theme.set_style(TokenType::Number, Style::new(Color::Magenta));
    theme.set_style(TokenType::Comment, Style::new(Color::Grey));
    theme.set_style(TokenType::Preprocessor, Style::new(Color::Cyan));
    theme.set_style(TokenType::Operator, Style::new(Color::Yellow));
    theme.set_style(TokenType::Delimiter, Style::new(Color::White));
    theme.set_style(TokenType::Type, Style::new(Color::Cyan));
    theme.set_style(TokenType::Function, Style::new(Color::Yellow));
    theme.set_style(TokenType::Macro, Style::new(Color::Red));
    theme.set_style(TokenType::Constant, Style::new(Color::Magenta));
    theme.set_style(TokenType::Special, Style::new(Color::Cyan));
    theme.set_style(TokenType::Error, Style::new(Color::Red).with_attribute(Attribute::Bold));
    theme.set_style(TokenType::Warning, Style::new(Color::Yellow).with_attribute(Attribute::Bold));
    theme.set_style(TokenType::Default, Style::new(Color::White));

    theme
}

/// Create a syntax definition for Rust
pub fn create_rust_syntax() -> Result<SyntaxDefinition, regex::Error> {
    let mut rust = SyntaxDefinition::new("Rust");
    rust.add_extension("rs");

    // Keywords
    rust.add_rule(SyntaxRule::new(
        TokenType::Keyword,
        r"\b(as|break|const|continue|crate|else|enum|extern|false|fn|for|if|impl|in|let|loop|match|mod|move|mut|pub|ref|return|self|Self|static|struct|super|trait|true|type|unsafe|use|where|while|async|await|dyn)\b"
    )?);

    // Types
    rust.add_rule(SyntaxRule::new(
        TokenType::Type,
        r"\b(i8|i16|i32|i64|i128|isize|u8|u16|u32|u64|u128|usize|f32|f64|bool|char|str|String|Vec|Option|Result)\b"
    )?);

    // Macros
    rust.add_rule(SyntaxRule::new(
        TokenType::Macro,
        r"\b[a-zA-Z_][a-zA-Z0-9_]*!"
    )?);

    // Comments
    rust.add_rule(SyntaxRule::new(
        TokenType::Comment,
        r"//.*$|/\*[\s\S]*?\*/"
    )?);

    // Strings
    rust.add_rule(SyntaxRule::new(
        TokenType::String,
        r#""([^"\\]|\\.)*""#
    )?);

    // Characters
    rust.add_rule(SyntaxRule::new(
        TokenType::Character,
        r"'([^'\\]|\\.)'"
    )?);

    // Numbers
    rust.add_rule(SyntaxRule::new(
        TokenType::Number,
        r"\b\d+(\.\d+)?([eE][+-]?\d+)?(f32|f64|i8|i16|i32|i64|i128|isize|u8|u16|u32|u64|u128|usize)?\b"
    )?);

    // Function definitions
    rust.add_rule(SyntaxRule::new(
        TokenType::Function,
        r"\bfn\s+([a-zA-Z_][a-zA-Z0-9_]*)"
    )?);

    // Operators
    rust.add_rule(SyntaxRule::new(
        TokenType::Operator,
        r"[+\-*/%&|^!~<>]|->|=>|::|&&|\|\|"
    )?);

    // Delimiters
    rust.add_rule(SyntaxRule::new(
        TokenType::Delimiter,
        r"[(){}\[\];,]"
    )?);

    Ok(rust)
}

/// Create a syntax definition for C/C++
pub fn create_cpp_syntax() -> Result<SyntaxDefinition, regex::Error> {
    let mut cpp = SyntaxDefinition::new("C/C++");
    cpp.add_extension("c");
    cpp.add_extension("cpp");
    cpp.add_extension("h");
    cpp.add_extension("hpp");

    // Keywords
    cpp.add_rule(SyntaxRule::new(
        TokenType::Keyword,
        r"\b(auto|break|case|const|continue|default|do|else|enum|extern|for|goto|if|inline|register|restrict|return|sizeof|static|struct|switch|typedef|union|volatile|while|class|namespace|template|try|catch|throw|new|delete|using|public|protected|private|virtual|friend|explicit|final|override|nullptr)\b"
    )?);

    // Types
    cpp.add_rule(SyntaxRule::new(
        TokenType::Type,
        r"\b(void|char|short|int|long|float|double|signed|unsigned|bool|size_t|int8_t|int16_t|int32_t|int64_t|uint8_t|uint16_t|uint32_t|uint64_t|std::string|std::vector|std::map|std::set|std::list|std::deque|std::queue|std::stack|std::pair|std::tuple)\b"
    )?);

    // Preprocessor directives
    cpp.add_rule(SyntaxRule::new(
        TokenType::Preprocessor,
        r"#\s*(include|define|undef|if|ifdef|ifndef|else|elif|endif|line|error|pragma)"
    )?);

    // Comments
    cpp.add_rule(SyntaxRule::new(
        TokenType::Comment,
        r"//.*$|/\*[\s\S]*?\*/"
    )?);

    // Strings
    cpp.add_rule(SyntaxRule::new(
        TokenType::String,
        r#""([^"\\]|\\.)*""#
    )?);

    // Characters
    cpp.add_rule(SyntaxRule::new(
        TokenType::Character,
        r"'([^'\\]|\\.)'"
    )?);

    // Numbers
    cpp.add_rule(SyntaxRule::new(
        TokenType::Number,
        r"\b\d+(\.\d+)?([eE][+-]?\d+)?(f|F|l|L|u|U)?\b|0x[0-9a-fA-F]+|0b[01]+"
    )?);

    // Function definitions
    cpp.add_rule(SyntaxRule::new(
        TokenType::Function,
        r"\b([a-zA-Z_][a-zA-Z0-9_]*)\s*\("
    )?);

    // Operators
    cpp.add_rule(SyntaxRule::new(
        TokenType::Operator,
        r"[+\-*/%&|^!~<>]=?|->|::|&&|\|\||<<|>>"
    )?);

    // Delimiters
    cpp.add_rule(SyntaxRule::new(
        TokenType::Delimiter,
        r"[(){}\[\];,]"
    )?);

    Ok(cpp)
}

/// Create a syntax definition for Python
pub fn create_python_syntax() -> Result<SyntaxDefinition, regex::Error> {
    let mut python = SyntaxDefinition::new("Python");
    python.add_extension("py");

    // Keywords
    python.add_rule(SyntaxRule::new(
        TokenType::Keyword,
        r"\b(and|as|assert|break|class|continue|def|del|elif|else|except|finally|for|from|global|if|import|in|is|lambda|nonlocal|not|or|pass|raise|return|try|while|with|yield)\b"
    )?);

    // Built-in types
    python.add_rule(SyntaxRule::new(
        TokenType::Type,
        r"\b(bool|int|float|str|list|tuple|set|dict|None|True|False)\b"
    )?);

    // Comments
    python.add_rule(SyntaxRule::new(
        TokenType::Comment,
        r"#.*$"
    )?);

    // Strings
    python.add_rule(SyntaxRule::new(
        TokenType::String,
        r#"("""[\s\S]*?"""|'''[\s\S]*?'''|"([^"\\]|\\.)*"|'([^'\\]|\\.)*')"#
    )?);

    // Numbers
    python.add_rule(SyntaxRule::new(
        TokenType::Number,
        r"\b\d+(\.\d+)?([eE][+-]?\d+)?(j)?\b|0x[0-9a-fA-F]+|0b[01]+"
    )?);

    // Function definitions
    python.add_rule(SyntaxRule::new(
        TokenType::Function,
        r"\bdef\s+([a-zA-Z_][a-zA-Z0-9_]*)"
    )?);

    // Operators
    python.add_rule(SyntaxRule::new(
        TokenType::Operator,
        r"[+\-*/%&|^!~<>]=?|//|==|!=|<=|>=|@"
    )?);

    // Delimiters
    python.add_rule(SyntaxRule::new(
        TokenType::Delimiter,
        r"[(){}\[\]:;,]"
    )?);

    Ok(python)
}

/// Create a syntax registry with default syntax definitions
pub fn create_default_registry() -> Result<SyntaxRegistry, regex::Error> {
    let mut registry = SyntaxRegistry::new();
    
    registry.add_definition(create_rust_syntax()?);
    registry.add_definition(create_cpp_syntax()?);
    registry.add_definition(create_python_syntax()?);
    
    Ok(registry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme() {
        let mut theme = Theme::new("Test");
        let style = Style::new(Color::Blue);
        theme.set_style(TokenType::Keyword, style.clone());
        
        let retrieved = theme.get_style(&TokenType::Keyword).unwrap();
        assert_eq!(retrieved.foreground, Some(Color::Blue));
    }

    #[test]
    fn test_syntax_rule() {
        let rule = SyntaxRule::new(TokenType::Keyword, r"\blet\b").unwrap();
        assert!(rule.pattern.is_match("let x = 5;"));
        assert!(!rule.pattern.is_match("letter"));
    }

    #[test]
    fn test_syntax_definition() {
        let mut def = SyntaxDefinition::new("Test");
        def.add_extension("test");
        
        assert!(def.applies_to_file("file.test"));
        assert!(!def.applies_to_file("file.txt"));
    }

    #[test]
    fn test_highlight_line() {
        let mut def = SyntaxDefinition::new("Test");
        def.add_rule(SyntaxRule::new(TokenType::Keyword, r"\blet\b").unwrap());
        def.add_rule(SyntaxRule::new(TokenType::Identifier, r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap());
        def.add_rule(SyntaxRule::new(TokenType::Operator, r"=").unwrap());
        def.add_rule(SyntaxRule::new(TokenType::Number, r"\b\d+\b").unwrap());
        def.add_rule(SyntaxRule::new(TokenType::Delimiter, r";").unwrap());
        
        let mut theme = Theme::new("Test");
        theme.set_style(TokenType::Keyword, Style::new(Color::Blue));
        theme.set_style(TokenType::Identifier, Style::new(Color::White));
        theme.set_style(TokenType::Operator, Style::new(Color::Yellow));
        theme.set_style(TokenType::Number, Style::new(Color::Magenta));
        theme.set_style(TokenType::Delimiter, Style::new(Color::White));
        
        let highlighter = SyntaxHighlighter::new(Arc::new(def), Arc::new(theme));
        let tokens = highlighter.highlight_line("let x = 5;");
        
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type, TokenType::Keyword);
        assert_eq!(tokens[0].text, "let");
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].text, "x");
        assert_eq!(tokens[2].token_type, TokenType::Operator);
        assert_eq!(tokens[2].text, "=");
        assert_eq!(tokens[3].token_type, TokenType::Number);
        assert_eq!(tokens[3].text, "5");
        assert_eq!(tokens[4].token_type, TokenType::Delimiter);
        assert_eq!(tokens[4].text, ";");
    }
}