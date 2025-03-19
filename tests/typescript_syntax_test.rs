//! Test for TypeScript syntax highlighting
//!
//! This file contains tests for the TypeScript syntax highlighting functionality.

use xvim::syntax::{get_syntax_definition, TokenType, Token};

/// Test TypeScript syntax highlighting
fn test_typescript_syntax() {
    println!("Testing TypeScript syntax highlighting...");
    
    // Get the TypeScript syntax definition
    let definition = match get_syntax_definition("typescript") {
        Some(def) => def,
        None => {
            println!("  Error: TypeScript syntax definition not found");
            return;
        }
    };
    
    // Test TypeScript keywords
    let keywords = vec![
        "class", "interface", "extends", "implements", "type", "enum",
        "const", "let", "var", "function", "return", "if", "else",
        "for", "while", "do", "switch", "case", "default", "break",
        "continue", "import", "export", "from", "as", "async", "await",
        "public", "private", "protected", "static", "readonly", "abstract",
    ];
    
    for keyword in keywords {
        let tokens = definition.highlight_line(keyword);
        
        if tokens.is_empty() {
            println!("  Error: No tokens found for keyword '{}'", keyword);
            continue;
        }
        
        let token = &tokens[0];
        if token.token_type != TokenType::Keyword {
            println!("  Error: Expected keyword token for '{}', got {:?}", keyword, token.token_type);
        } else {
            println!("  Keyword '{}' correctly highlighted", keyword);
        }
    }
    
    // Test TypeScript types
    let types = vec![
        "string", "number", "boolean", "any", "void", "never", "unknown",
        "object", "null", "undefined", "bigint", "symbol",
    ];
    
    for type_name in types {
        let tokens = definition.highlight_line(type_name);
        
        if tokens.is_empty() {
            println!("  Error: No tokens found for type '{}'", type_name);
            continue;
        }
        
        let token = &tokens[0];
        if token.token_type != TokenType::Type {
            println!("  Error: Expected type token for '{}', got {:?}", type_name, token.token_type);
        } else {
            println!("  Type '{}' correctly highlighted", type_name);
        }
    }
    
    // Test TypeScript comments
    let comment_lines = vec![
        "// This is a single-line comment",
        "/* This is a multi-line comment */",
    ];
    
    for comment in comment_lines {
        let tokens = definition.highlight_line(comment);
        
        if tokens.is_empty() {
            println!("  Error: No tokens found for comment '{}'", comment);
            continue;
        }
        
        let token = &tokens[0];
        if token.token_type != TokenType::Comment {
            println!("  Error: Expected comment token for '{}', got {:?}", comment, token.token_type);
        } else {
            println!("  Comment '{}' correctly highlighted", comment);
        }
    }
    
    // Test TypeScript strings
    let string_lines = vec![
        "\"This is a double-quoted string\"",
        "'This is a single-quoted string'",
        "`This is a template string`",
    ];
    
    for string in string_lines {
        let tokens = definition.highlight_line(string);
        
        if tokens.is_empty() {
            println!("  Error: No tokens found for string '{}'", string);
            continue;
        }
        
        let token = &tokens[0];
        if token.token_type != TokenType::String {
            println!("  Error: Expected string token for '{}', got {:?}", string, token.token_type);
        } else {
            println!("  String '{}' correctly highlighted", string);
        }
    }
    
    // Test TypeScript numbers
    let number_lines = vec![
        "123", "123.456", "0x1A", "0b1010", "0o777",
    ];
    
    for number in number_lines {
        let tokens = definition.highlight_line(number);
        
        if tokens.is_empty() {
            println!("  Error: No tokens found for number '{}'", number);
            continue;
        }
        
        let token = &tokens[0];
        if token.token_type != TokenType::Number {
            println!("  Error: Expected number token for '{}', got {:?}", number, token.token_type);
        } else {
            println!("  Number '{}' correctly highlighted", number);
        }
    }
    
    // Test TypeScript decorators
    let decorator_lines = vec![
        "@Component", "@Injectable", "@Input", "@Output",
    ];
    
    for decorator in decorator_lines {
        let tokens = definition.highlight_line(decorator);
        
        if tokens.is_empty() {
            println!("  Error: No tokens found for decorator '{}'", decorator);
            continue;
        }
        
        let token = &tokens[0];
        if token.token_type != TokenType::Preprocessor {
            println!("  Error: Expected preprocessor token for '{}', got {:?}", decorator, token.token_type);
        } else {
            println!("  Decorator '{}' correctly highlighted", decorator);
        }
    }
    
    // Test TypeScript type annotations
    let type_annotation_lines = vec![
        ": string", ": number", ": boolean", ": any[]",
    ];
    
    for type_annotation in type_annotation_lines {
        let tokens = definition.highlight_line(type_annotation);
        
        if tokens.is_empty() {
            println!("  Error: No tokens found for type annotation '{}'", type_annotation);
            continue;
        }
        
        let token = &tokens[0];
        if token.token_type != TokenType::Type {
            println!("  Error: Expected type token for '{}', got {:?}", type_annotation, token.token_type);
        } else {
            println!("  Type annotation '{}' correctly highlighted", type_annotation);
        }
    }
    
    // Test TypeScript JSX/TSX tags
    let tag_lines = vec![
        "<div>", "</div>", "<Component>", "</Component>", "/>",
    ];
    
    for tag in tag_lines {
        let tokens = definition.highlight_line(tag);
        
        if tokens.is_empty() {
            println!("  Error: No tokens found for tag '{}'", tag);
            continue;
        }
        
        let token = &tokens[0];
        if token.token_type != TokenType::Tag {
            println!("  Error: Expected tag token for '{}', got {:?}", tag, token.token_type);
        } else {
            println!("  Tag '{}' correctly highlighted", tag);
        }
    }
    
    println!("TypeScript syntax highlighting tests completed");
}

/// Run all tests
fn main() {
    println!("Running TypeScript syntax highlighting tests...");
    
    // Run the tests
    test_typescript_syntax();
    
    println!("All TypeScript syntax highlighting tests completed!");
}