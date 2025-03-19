//! Vim script execution tests

use xvim::{
    VimValue, VimScope, VimVariable, VimFunction, VimContext, VimScriptInterpreter,
    execute_file, execute, execute_line, register_vim_script_commands
};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[test]
fn test_vim_value() {
    // Test string value
    let string_value = VimValue::String("hello".to_string());
    assert_eq!(string_value.to_string(), "hello");
    assert_eq!(string_value.to_integer(), 0);
    assert_eq!(string_value.to_float(), 0.0);
    assert_eq!(string_value.to_boolean(), true);
    
    // Test integer value
    let int_value = VimValue::Integer(42);
    assert_eq!(int_value.to_string(), "42");
    assert_eq!(int_value.to_integer(), 42);
    assert_eq!(int_value.to_float(), 42.0);
    assert_eq!(int_value.to_boolean(), true);
    
    // Test float value
    let float_value = VimValue::Float(3.14);
    assert_eq!(float_value.to_string(), "3.14");
    assert_eq!(float_value.to_integer(), 3);
    assert_eq!(float_value.to_float(), 3.14);
    assert_eq!(float_value.to_boolean(), true);
    
    // Test boolean value
    let bool_value = VimValue::Boolean(true);
    assert_eq!(bool_value.to_string(), "1");
    assert_eq!(bool_value.to_integer(), 1);
    assert_eq!(bool_value.to_float(), 1.0);
    assert_eq!(bool_value.to_boolean(), true);
    
    // Test null value
    let null_value = VimValue::Null;
    assert_eq!(null_value.to_string(), "");
    assert_eq!(null_value.to_integer(), 0);
    assert_eq!(null_value.to_float(), 0.0);
    assert_eq!(null_value.to_boolean(), false);
    
    // Test list value
    let list_value = VimValue::List(vec![
        VimValue::Integer(1),
        VimValue::Integer(2),
        VimValue::Integer(3),
    ]);
    assert_eq!(list_value.to_string(), "[1, 2, 3]");
    assert_eq!(list_value.to_integer(), 0);
    assert_eq!(list_value.to_float(), 0.0);
    assert_eq!(list_value.to_boolean(), true);
    
    // Test dictionary value
    let mut dict = HashMap::new();
    dict.insert("foo".to_string(), VimValue::String("bar".to_string()));
    dict.insert("baz".to_string(), VimValue::Integer(42));
    let dict_value = VimValue::Dict(dict);
    assert!(dict_value.to_string().contains("'foo': bar"));
    assert!(dict_value.to_string().contains("'baz': 42"));
    assert_eq!(dict_value.to_integer(), 0);
    assert_eq!(dict_value.to_float(), 0.0);
    assert_eq!(dict_value.to_boolean(), true);
}

#[test]
fn test_vim_scope() {
    // Test scope parsing
    assert_eq!(VimScope::parse("g:"), Some(VimScope::Global));
    assert_eq!(VimScope::parse("b:"), Some(VimScope::Buffer));
    assert_eq!(VimScope::parse("w:"), Some(VimScope::Window));
    assert_eq!(VimScope::parse("t:"), Some(VimScope::Tab));
    assert_eq!(VimScope::parse("s:"), Some(VimScope::Script));
    assert_eq!(VimScope::parse("l:"), Some(VimScope::Function));
    assert_eq!(VimScope::parse("a:"), Some(VimScope::Argument));
    assert_eq!(VimScope::parse("v:"), Some(VimScope::Vim));
    assert_eq!(VimScope::parse("env:"), Some(VimScope::Environment));
    assert_eq!(VimScope::parse("&"), Some(VimScope::Option));
    assert_eq!(VimScope::parse("@"), Some(VimScope::Register));
    assert_eq!(VimScope::parse("x:"), None);
    
    // Test scope prefix
    assert_eq!(VimScope::Global.prefix(), "g:");
    assert_eq!(VimScope::Buffer.prefix(), "b:");
    assert_eq!(VimScope::Window.prefix(), "w:");
    assert_eq!(VimScope::Tab.prefix(), "t:");
    assert_eq!(VimScope::Script.prefix(), "s:");
    assert_eq!(VimScope::Function.prefix(), "l:");
    assert_eq!(VimScope::Argument.prefix(), "a:");
    assert_eq!(VimScope::Vim.prefix(), "v:");
    assert_eq!(VimScope::Environment.prefix(), "env:");
    assert_eq!(VimScope::Option.prefix(), "&");
    assert_eq!(VimScope::Register.prefix(), "@");
    assert_eq!(VimScope::None.prefix(), "");
}

#[test]
fn test_vim_variable() {
    // Test variable creation
    let var = VimVariable::new("foo", VimScope::Global, VimValue::String("bar".to_string()));
    assert_eq!(var.name, "foo");
    assert_eq!(var.scope, VimScope::Global);
    assert!(matches!(var.value, VimValue::String(ref s) if s == "bar"));
    assert_eq!(var.full_name(), "g:foo");
    
    // Test variable with different scope
    let var = VimVariable::new("count", VimScope::Vim, VimValue::Integer(42));
    assert_eq!(var.name, "count");
    assert_eq!(var.scope, VimScope::Vim);
    assert!(matches!(var.value, VimValue::Integer(n) if n == 42));
    assert_eq!(var.full_name(), "v:count");
}

#[test]
fn test_vim_function() {
    // Test function creation
    let func = VimFunction::new(
        "MyFunc",
        vec!["arg1".to_string(), "arg2".to_string()],
        vec!["echo arg1".to_string(), "return arg2".to_string()],
        false,
        None,
        1,
    );
    assert_eq!(func.name, "MyFunc");
    assert_eq!(func.args, vec!["arg1", "arg2"]);
    assert_eq!(func.body, vec!["echo arg1", "return arg2"]);
    assert_eq!(func.is_script_local, false);
    assert_eq!(func.script_id, None);
    assert_eq!(func.line, 1);
    assert_eq!(func.full_name(), "MyFunc");
    
    // Test script-local function
    let func = VimFunction::new(
        "MyFunc",
        vec!["arg1".to_string(), "arg2".to_string()],
        vec!["echo arg1".to_string(), "return arg2".to_string()],
        true,
        Some(42),
        1,
    );
    assert_eq!(func.is_script_local, true);
    assert_eq!(func.script_id, Some(42));
    assert_eq!(func.full_name(), "<SNR>42_MyFunc");
}

#[test]
fn test_vim_context() {
    // Create a command registry
    let registry = Arc::new(Mutex::new(xvim::command::ExCommandRegistry::new()));
    
    // Create a context
    let mut context = VimContext::new(registry);
    
    // Test variable operations
    context.set_variable("foo", VimScope::Global, VimValue::String("bar".to_string()));
    let var = context.get_variable("foo", VimScope::Global);
    assert!(var.is_some());
    assert!(matches!(var.unwrap(), VimValue::String(ref s) if s == "bar"));
    
    // Test Vim variables
    let var = context.get_variable("version", VimScope::Vim);
    assert!(var.is_some());
    assert!(matches!(var.unwrap(), VimValue::Integer(n) if n == 900));
    
    // Test variable name parsing
    let (name, scope) = context.parse_variable_name("g:foo");
    assert_eq!(name, "foo");
    assert_eq!(scope, VimScope::Global);
    
    let (name, scope) = context.parse_variable_name("v:count");
    assert_eq!(name, "count");
    assert_eq!(scope, VimScope::Vim);
    
    let (name, scope) = context.parse_variable_name("&tabstop");
    assert_eq!(name, "tabstop");
    assert_eq!(scope, VimScope::Option);
    
    let (name, scope) = context.parse_variable_name("@a");
    assert_eq!(name, "a");
    assert_eq!(scope, VimScope::Register);
    
    let (name, scope) = context.parse_variable_name("foo");
    assert_eq!(name, "foo");
    assert_eq!(scope, VimScope::None);
    
    // Test function operations
    let func = VimFunction::new(
        "MyFunc",
        vec!["arg1".to_string(), "arg2".to_string()],
        vec!["echo arg1".to_string(), "return arg2".to_string()],
        false,
        None,
        1,
    );
    context.set_function(func);
    let func = context.get_function("MyFunc");
    assert!(func.is_some());
    assert_eq!(func.unwrap().name, "MyFunc");
    
    // Test expression evaluation
    let result = context.evaluate_expression("42");
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), VimValue::Integer(n) if n == 42));
    
    let result = context.evaluate_expression("\"hello\"");
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), VimValue::String(ref s) if s == "hello"));
    
    let result = context.evaluate_expression("v:version");
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), VimValue::Integer(n) if n == 900));
    
    let result = context.evaluate_expression("[1, 2, 3]");
    assert!(result.is_ok());
    if let Ok(VimValue::List(list)) = result {
        assert_eq!(list.len(), 3);
        assert!(matches!(list[0], VimValue::Integer(n) if n == 1));
        assert!(matches!(list[1], VimValue::Integer(n) if n == 2));
        assert!(matches!(list[2], VimValue::Integer(n) if n == 3));
    } else {
        panic!("Expected list value");
    }
    
    let result = context.evaluate_expression("{'foo': 'bar', 'baz': 42}");
    assert!(result.is_ok());
    if let Ok(VimValue::Dict(dict)) = result {
        assert_eq!(dict.len(), 2);
        assert!(matches!(dict.get("foo"), Some(VimValue::String(ref s)) if s == "bar"));
        assert!(matches!(dict.get("baz"), Some(VimValue::Integer(n)) if n == 42));
    } else {
        panic!("Expected dictionary value");
    }
}

#[test]
fn test_vim_script_interpreter() {
    // Create a command registry
    let registry = Arc::new(Mutex::new(xvim::command::ExCommandRegistry::new()));
    
    // Create an interpreter
    let mut interpreter = VimScriptInterpreter::new(registry);
    
    // Test script execution
    let result = interpreter.execute("let g:foo = 'bar'");
    assert!(result.is_ok());
    
    let result = interpreter.execute("let g:num = 42");
    assert!(result.is_ok());
    
    let result = interpreter.execute("let g:list = [1, 2, 3]");
    assert!(result.is_ok());
    
    let result = interpreter.execute("let g:dict = {'foo': 'bar', 'baz': 42}");
    assert!(result.is_ok());
    
    // Test variable access
    let var = interpreter.context.get_variable("foo", VimScope::Global);
    assert!(var.is_some());
    assert!(matches!(var.unwrap(), VimValue::String(ref s) if s == "bar"));
    
    let var = interpreter.context.get_variable("num", VimScope::Global);
    assert!(var.is_some());
    assert!(matches!(var.unwrap(), VimValue::Integer(n) if n == 42));
    
    let var = interpreter.context.get_variable("list", VimScope::Global);
    assert!(var.is_some());
    if let VimValue::List(list) = var.unwrap() {
        assert_eq!(list.len(), 3);
        assert!(matches!(list[0], VimValue::Integer(n) if n == 1));
        assert!(matches!(list[1], VimValue::Integer(n) if n == 2));
        assert!(matches!(list[2], VimValue::Integer(n) if n == 3));
    } else {
        panic!("Expected list value");
    }
    
    let var = interpreter.context.get_variable("dict", VimScope::Global);
    assert!(var.is_some());
    if let VimValue::Dict(dict) = var.unwrap() {
        assert_eq!(dict.len(), 2);
        assert!(matches!(dict.get("foo"), Some(VimValue::String(ref s)) if s == "bar"));
        assert!(matches!(dict.get("baz"), Some(VimValue::Integer(n)) if n == 42));
    } else {
        panic!("Expected dictionary value");
    }
    
    // Test function definition and execution
    let result = interpreter.execute("function! MyFunc(arg1, arg2)\necho a:arg1\nreturn a:arg2\nendfunction");
    assert!(result.is_ok());
    
    let func = interpreter.context.get_function("MyFunc");
    assert!(func.is_some());
    assert_eq!(func.unwrap().name, "MyFunc");
    
    // Test if statement
    let result = interpreter.execute("if 1\nlet g:result = 'true'\nelse\nlet g:result = 'false'\nendif");
    assert!(result.is_ok());
    
    let var = interpreter.context.get_variable("result", VimScope::Global);
    assert!(var.is_some());
    assert!(matches!(var.unwrap(), VimValue::String(ref s) if s == "true"));
    
    // Test while loop
    let result = interpreter.execute("let g:i = 0\nwhile g:i < 5\nlet g:i += 1\nendwhile");
    assert!(result.is_ok());
    
    let var = interpreter.context.get_variable("i", VimScope::Global);
    assert!(var.is_some());
    assert!(matches!(var.unwrap(), VimValue::Integer(n) if n == 5));
    
    // Test for loop
    let result = interpreter.execute("let g:sum = 0\nfor i in [1, 2, 3, 4, 5]\nlet g:sum += i\nendfor");
    assert!(result.is_ok());
    
    let var = interpreter.context.get_variable("sum", VimScope::Global);
    assert!(var.is_some());
    assert!(matches!(var.unwrap(), VimValue::Integer(n) if n == 15));
}