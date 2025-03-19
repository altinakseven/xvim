//! TypeScript syntax highlighting definition
//!
//! This module provides syntax highlighting for TypeScript files.

use crate::syntax::{SyntaxDefinition, Rule, TokenType};
// use std::io::Read;

/// Create a TypeScript syntax definition
pub fn create_typescript_syntax() -> SyntaxDefinition {
    let mut definition = SyntaxDefinition::new("typescript");
    
    // Add file extensions
    definition.add_file_extension("ts");
    definition.add_file_extension("tsx");
    
    // Add keywords
    let keywords = vec![
        // JavaScript keywords
        "break", "case", "catch", "class", "const", "continue", "debugger", "default",
        "delete", "do", "else", "export", "extends", "finally", "for", "function",
        "if", "import", "in", "instanceof", "new", "return", "super", "switch",
        "this", "throw", "try", "typeof", "var", "void", "while", "with", "yield",
        "let", "static", "enum", "await", "implements", "package", "protected",
        "interface", "private", "public", "async", "of",
        
        // TypeScript-specific keywords
        "abstract", "as", "any", "boolean", "constructor", "declare", "get", "infer",
        "is", "keyof", "module", "namespace", "never", "readonly", "require", "number",
        "object", "set", "string", "symbol", "type", "undefined", "unique", "unknown",
        "from", "global", "bigint", "override",
    ];
    
    for keyword in keywords {
        definition.add_keyword(TokenType::Keyword, keyword);
    }
    
    // Add built-ins
    let builtins = vec![
        "true", "false", "null", "undefined", "NaN", "Infinity", "console", "window",
        "document", "global", "process", "require", "module", "exports", "Promise",
        "Array", "Object", "String", "Number", "Boolean", "Error", "Date", "RegExp",
        "Map", "Set", "WeakMap", "WeakSet", "Symbol", "Proxy", "Reflect", "JSON",
        "Math", "Intl", "ArrayBuffer", "Uint8Array", "Int8Array", "Uint16Array",
        "Int16Array", "Uint32Array", "Int32Array", "Float32Array", "Float64Array",
        "Uint8ClampedArray", "BigInt", "BigInt64Array", "BigUint64Array",
    ];
    
    for builtin in builtins {
        definition.add_keyword(TokenType::Constant, builtin);
    }
    
    // Add types
    let types = vec![
        "any", "void", "number", "boolean", "string", "object", "never", "undefined",
        "null", "symbol", "unknown", "bigint", "Function", "Record", "Partial", "Required",
        "Readonly", "Pick", "Omit", "Exclude", "Extract", "NonNullable", "Parameters",
        "ConstructorParameters", "ReturnType", "InstanceType", "ThisParameterType",
        "OmitThisParameter", "ThisType", "Uppercase", "Lowercase", "Capitalize",
        "Uncapitalize", "Promise", "Awaited",
    ];
    
    for ty in types {
        definition.add_keyword(TokenType::Type, ty);
    }
    
    // Add rules
    
    // Comments
    definition.add_rule(Rule::new_multiline(r"//", r"$", TokenType::Comment));
    definition.add_rule(Rule::new_multiline(r"/\*", r"\*/", TokenType::Comment));
    
    // Strings
    definition.add_rule(Rule::new_multiline(r#"""#, r#"""#, TokenType::String));
    definition.add_rule(Rule::new_multiline(r"'", r"'", TokenType::String));
    definition.add_rule(Rule::new_multiline(r"`", r"`", TokenType::String));
    
    // Numbers
    definition.add_rule(Rule::new_regex(r"\b\d+\b", TokenType::Number));
    definition.add_rule(Rule::new_regex(r"\b0x[0-9a-fA-F]+\b", TokenType::Number));
    definition.add_rule(Rule::new_regex(r"\b0o[0-7]+\b", TokenType::Number));
    definition.add_rule(Rule::new_regex(r"\b0b[01]+\b", TokenType::Number));
    
    // Regular expressions
    definition.add_rule(Rule::new_multiline(r"/", r"/[gimuy]*", TokenType::String));
    
    // Type annotations
    definition.add_rule(Rule::new_regex(r":\s*[A-Za-z_][A-Za-z0-9_]*(\[\])?", TokenType::Type));
    definition.add_rule(Rule::new_regex(r"<[^>]+>", TokenType::Type));
    
    // Decorators
    definition.add_rule(Rule::new_regex(r"@[A-Za-z_][A-Za-z0-9_]*", TokenType::Preprocessor));
    
    // JSX/TSX tags
    definition.add_rule(Rule::new_regex(r"<[A-Za-z_][A-Za-z0-9_]*", TokenType::Tag));
    definition.add_rule(Rule::new_regex(r"</[A-Za-z_][A-Za-z0-9_]*>", TokenType::Tag));
    definition.add_rule(Rule::new_regex(r"/>", TokenType::Tag));
    
    // JSX/TSX attributes
    definition.add_rule(Rule::new_regex(r"\b[A-Za-z_][A-Za-z0-9_]*=", TokenType::Attribute));
    
    // Add comments
    definition.add_comment("//", None);
    definition.add_comment("/*", Some("*/"));
    
    // Set indentation
    definition.set_indentation("typescript");
    
    definition
}