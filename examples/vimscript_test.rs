//! Test program for the Vim script execution system

use std::path::Path;
use std::sync::{Arc, Mutex};

use xvim::{
    VimValue, VimScope, VimVariable, VimFunction, VimContext, VimScriptInterpreter,
    execute_file, execute, execute_line, register_vim_script_commands,
    command::ExCommandRegistry,
};

fn main() {
    // Create a command registry
    let registry = Arc::new(Mutex::new(ExCommandRegistry::new()));
    
    // Register Vim script commands
    let mut registry_guard = registry.lock().unwrap();
    register_vim_script_commands(&mut registry_guard);
    drop(registry_guard);
    
    // Initialize the Vim script interpreter
    xvim::vimscript::init_vim_script_interpreter(registry.clone());
    
    // Set the runtime directory
    xvim::vimscript::set_runtime_dir(Path::new("runtime"));
    
    // Execute a test script
    match execute_file(Path::new("runtime/test.vim")) {
        Ok(_) => println!("Test script executed successfully!"),
        Err(err) => eprintln!("Error executing test script: {}", err),
    }
    
    // Execute some Vim script commands
    println!("\nExecuting Vim script commands:");
    
    // Set a variable
    match execute("let g:example = 'This is an example'") {
        Ok(_) => println!("Variable set successfully!"),
        Err(err) => eprintln!("Error setting variable: {}", err),
    }
    
    // Echo a message
    match execute("echo g:example") {
        Ok(_) => {},
        Err(err) => eprintln!("Error echoing message: {}", err),
    }
    
    // Define a function
    match execute("function! Example()\n  echo 'This is a function'\nendfunction") {
        Ok(_) => println!("Function defined successfully!"),
        Err(err) => eprintln!("Error defining function: {}", err),
    }
    
    // Call the function
    match execute("call Example()") {
        Ok(_) => {},
        Err(err) => eprintln!("Error calling function: {}", err),
    }
    
    // Test if statement
    match execute("if 1\n  echo 'True'\nelse\n  echo 'False'\nendif") {
        Ok(_) => {},
        Err(err) => eprintln!("Error executing if statement: {}", err),
    }
    
    // Test while loop
    match execute("let i = 0\nwhile i < 3\n  echo 'i = ' . i\n  let i += 1\nendwhile") {
        Ok(_) => {},
        Err(err) => eprintln!("Error executing while loop: {}", err),
    }
    
    // Test for loop
    match execute("for i in [1, 2, 3]\n  echo 'i = ' . i\nendfor") {
        Ok(_) => {},
        Err(err) => eprintln!("Error executing for loop: {}", err),
    }
    
    // Test vimrc.vim
    println!("\nExecuting vimrc.vim:");
    match execute_file(Path::new("runtime/vimrc.vim")) {
        Ok(_) => println!("vimrc.vim executed successfully!"),
        Err(err) => eprintln!("Error executing vimrc.vim: {}", err),
    }
}