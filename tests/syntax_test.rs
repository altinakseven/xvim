//! Syntax highlighting tests

use xvim::{
    Buffer, BufferManager, Editor, SyntaxDefinition, SyntaxManager, TokenType, Token, Rule,
    highlight_buffer, get_syntax_definition_for_file
};
use std::path::PathBuf;

#[test]
fn test_syntax_definition() {
    // Create a Rust syntax definition
    let mut definition = SyntaxDefinition::new("rust");
    
    // Add file extensions
    definition.add_file_extension("rs");
    
    // Add keywords
    let keywords = vec![
        "fn", "let", "mut", "if", "else", "match", "for", "while", "loop", "return",
    ];
    
    for keyword in keywords {
        definition.add_keyword(TokenType::Keyword, keyword);
    }
    
    // Add rules
    
    // Comments
    definition.add_rule(Rule::new_multiline(r"//", r"$", TokenType::Comment));
    definition.add_rule(Rule::new_multiline(r"/\*", r"\*/", TokenType::Comment));
    
    // Strings
    definition.add_rule(Rule::new_multiline(r#"""#, r#"""#, TokenType::String));
    
    // Test file extension matching
    assert!(definition.matches_file(&PathBuf::from("test.rs")));
    assert!(!definition.matches_file(&PathBuf::from("test.js")));
    
    // Test keyword matching
    let line = "fn main() {";
    let tokens = definition.highlight_line(line);
    
    assert!(!tokens.is_empty());
    assert_eq!(tokens[0].token_type, TokenType::Keyword);
    assert_eq!(&line[tokens[0].start..tokens[0].end], "fn");
}

#[test]
fn test_syntax_manager() {
    // Create a syntax manager
    let mut manager = SyntaxManager::new();
    
    // Create a Rust syntax definition
    let mut rust_def = SyntaxDefinition::new("rust");
    rust_def.add_file_extension("rs");
    
    // Create a JavaScript syntax definition
    let mut js_def = SyntaxDefinition::new("javascript");
    js_def.add_file_extension("js");
    
    // Add definitions to the manager
    manager.add_definition(rust_def);
    manager.add_definition(js_def);
    
    // Test getting definitions
    let rust_def = manager.get_definition("rust");
    assert!(rust_def.is_some());
    assert_eq!(rust_def.unwrap().name, "rust");
    
    let js_def = manager.get_definition("javascript");
    assert!(js_def.is_some());
    assert_eq!(js_def.unwrap().name, "javascript");
    
    // Test getting definitions for files
    let rust_def = manager.get_definition_for_file(&PathBuf::from("test.rs"));
    assert!(rust_def.is_some());
    assert_eq!(rust_def.unwrap().name, "rust");
    
    let js_def = manager.get_definition_for_file(&PathBuf::from("test.js"));
    assert!(js_def.is_some());
    assert_eq!(js_def.unwrap().name, "javascript");
}

#[test]
fn test_highlight_buffer() {
    // Create a buffer manager
    let mut buffer_manager = BufferManager::new();
    
    // Create a buffer
    let buffer_id = buffer_manager.create_buffer().unwrap();
    let buffer = buffer_manager.get_buffer_mut(buffer_id).unwrap();
    
    // Set buffer content
    buffer.set_content("fn main() {\n    println!(\"Hello, world!\");\n}");
    
    // Set buffer path
    buffer.set_path(Some(PathBuf::from("test.rs")));
    
    // Create a syntax manager
    let mut manager = SyntaxManager::new();
    
    // Create a Rust syntax definition
    let mut rust_def = SyntaxDefinition::new("rust");
    rust_def.add_file_extension("rs");
    
    // Add keywords
    let keywords = vec![
        "fn", "let", "mut", "if", "else", "match", "for", "while", "loop", "return",
    ];
    
    for keyword in keywords {
        rust_def.add_keyword(TokenType::Keyword, keyword);
    }
    
    // Add rules
    
    // Comments
    rust_def.add_rule(Rule::new_multiline(r"//", r"$", TokenType::Comment));
    rust_def.add_rule(Rule::new_multiline(r"/\*", r"\*/", TokenType::Comment));
    
    // Strings
    rust_def.add_rule(Rule::new_multiline(r#"""#, r#"""#, TokenType::String));
    
    // Add definition to the manager
    manager.add_definition(rust_def);
    
    // Highlight the buffer
    let tokens = manager.highlight_buffer(buffer);
    
    assert!(tokens.is_some());
    let tokens = tokens.unwrap();
    
    // Check first line
    assert!(!tokens[0].is_empty());
    assert_eq!(tokens[0][0].token_type, TokenType::Keyword);
    
    // Check second line
    assert!(!tokens[1].is_empty());
    assert!(tokens[1].iter().any(|token| token.token_type == TokenType::String));
}

#[test]
fn test_built_in_syntax() {
    // Initialize built-in syntax definitions
    xvim::syntax::init_built_in_syntax();
    
    // Get Rust syntax definition
    let rust_def = xvim::syntax::get_syntax_definition("rust");
    assert!(rust_def.is_some());
    
    // Get C syntax definition
    let c_def = xvim::syntax::get_syntax_definition("c");
    assert!(c_def.is_some());
    
    // Get Python syntax definition
    let python_def = xvim::syntax::get_syntax_definition("python");
    assert!(python_def.is_some());
    
    // Get JavaScript syntax definition
    let js_def = xvim::syntax::get_syntax_definition("javascript");
    assert!(js_def.is_some());
    
    // Get HTML syntax definition
    let html_def = xvim::syntax::get_syntax_definition("html");
    assert!(html_def.is_some());
    
    // Get CSS syntax definition
    let css_def = xvim::syntax::get_syntax_definition("css");
    assert!(css_def.is_some());
    
    // Get Markdown syntax definition
    let md_def = xvim::syntax::get_syntax_definition("markdown");
    assert!(md_def.is_some());
}

#[test]
fn test_vim_syntax_parsing() {
    // Create a syntax manager
    let manager = SyntaxManager::new();
    
    // Create a simple Vim syntax file
    let vim_syntax = r#"
" Vim syntax file
" Language: Test
" Maintainer: Test
" Latest Revision: 2023-01-01

syntax clear

syntax keyword testKeyword if else for while
syntax keyword testType int float string
syntax match testNumber /\d\+/
syntax match testString /"[^"]*"/
syntax region testComment start="/\*" end="\*/"
syntax region testLineComment start="//" end="$"

let b:current_syntax = "test"

au BufRead,BufNewFile *.test setfiletype test
"#;
    
    // Parse the Vim syntax file
    let definition = manager.parse_vim_syntax(vim_syntax);
    
    assert!(definition.is_some());
    let definition = definition.unwrap();
    
    // Check the definition
    assert_eq!(definition.name, "test");
    assert!(definition.file_extensions.contains(&"test".to_string()));
    
    // Check the keywords
    assert!(definition.keywords.contains_key(&TokenType::Keyword));
    assert!(definition.keywords.get(&TokenType::Keyword).unwrap().contains("if"));
    assert!(definition.keywords.get(&TokenType::Keyword).unwrap().contains("else"));
    assert!(definition.keywords.get(&TokenType::Keyword).unwrap().contains("for"));
    assert!(definition.keywords.get(&TokenType::Keyword).unwrap().contains("while"));
    
    assert!(definition.keywords.contains_key(&TokenType::Type));
    assert!(definition.keywords.get(&TokenType::Type).unwrap().contains("int"));
    assert!(definition.keywords.get(&TokenType::Type).unwrap().contains("float"));
    assert!(definition.keywords.get(&TokenType::Type).unwrap().contains("string"));
    
    // Check the rules
    assert!(!definition.rules.is_empty());
}