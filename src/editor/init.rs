//! Editor initialization module
//!
//! This module provides functionality for initializing the editor.

use std::fs;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::path::{Path, PathBuf};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;

use crate::editor::Editor;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::vimscript;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::command::config_handlers;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;

/// Initialize the editor
pub fn initialize_editor(editor: &mut Editor) -> io::Result<()> {
    // Set up the runtime directory
    let runtime_dir = setup_runtime_directory()?;
    
    // Initialize the configuration system
    initialize_configuration_system(editor)?;
    
    // Initialize the Vim script interpreter
    initialize_vim_script_interpreter(editor, &runtime_dir)?;
    
    // Load the vimrc file
    load_vimrc(editor, &runtime_dir)?;
    
    Ok(())
}

/// Initialize the configuration system
fn initialize_configuration_system(editor: &mut Editor) -> io::Result<()> {
    // Initialize the configuration manager
    config_handlers::init_config_manager();
    
    // Register configuration commands
    config_handlers::register_config_handlers(&mut editor.ex_command_registry);
    
    Ok(())
}

/// Initialize the environment
pub fn initialize_environment(editor: &mut Editor) -> io::Result<()> {
    // Create necessary directories
    create_directories()?;
    
    // Set up the runtime directory
    let runtime_dir = setup_runtime_directory()?;
    
    // Load plugins
    load_plugins(editor, &runtime_dir)?;
    
    // Load filetype plugins
    load_filetype_plugins(editor, &runtime_dir)?;
    
    // Load syntax files
    load_syntax_files(editor, &runtime_dir)?;
    
    // Load indent files
    load_indent_files(editor, &runtime_dir)?;
    
    // Load color schemes
    load_color_schemes(editor, &runtime_dir)?;
    
    Ok(())
}

/// Set up the runtime directory
fn setup_runtime_directory() -> io::Result<PathBuf> {
    // Check if the runtime directory exists in the current directory
    let local_runtime = Path::new("runtime");
    if local_runtime.exists() && local_runtime.is_dir() {
        return Ok(local_runtime.to_path_buf());
    }
    
    // Check if the runtime directory exists in the user's home directory
    let home_dir = dirs::home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
    let home_runtime = home_dir.join(".xvim").join("runtime");
    if home_runtime.exists() && home_runtime.is_dir() {
        return Ok(home_runtime);
    }
    
    // Check if the runtime directory exists in the system directory
    let system_runtime = Path::new("/usr/share/xvim/runtime");
    if system_runtime.exists() && system_runtime.is_dir() {
        return Ok(system_runtime.to_path_buf());
    }
    
    // If no runtime directory exists, use the local one and create it if it doesn't exist
    if !local_runtime.exists() {
        fs::create_dir_all(local_runtime)?;
    }
    
    Ok(local_runtime.to_path_buf())
}

/// Initialize the Vim script interpreter
fn initialize_vim_script_interpreter(editor: &mut Editor, runtime_dir: &Path) -> io::Result<()> {
    // Set the editor reference
    vimscript::set_editor(editor as *mut Editor);
    
    // Set the runtime directory
    vimscript::set_runtime_dir(runtime_dir);
    
    // Initialize the Vim script interpreter
    vimscript::init_vim_script_interpreter(editor.command_registry.clone());
    
    Ok(())
}

/// Load the vimrc file
fn load_vimrc(editor: &mut Editor, runtime_dir: &Path) -> io::Result<()> {
    // Check if the vimrc file exists in the user's home directory
    let home_dir = dirs::home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
    let home_vimrc = home_dir.join(".xvimrc");
    if home_vimrc.exists() && home_vimrc.is_file() {
        vimscript::execute_file(&home_vimrc)?;
        return Ok(());
    }
    
    // Check if the vimrc file exists in the runtime directory
    let runtime_vimrc = runtime_dir.join("vimrc.vim");
    if runtime_vimrc.exists() && runtime_vimrc.is_file() {
        vimscript::execute_file(&runtime_vimrc)?;
        return Ok(());
    }
    
    // If no vimrc file exists, create a default one
    let default_vimrc = runtime_dir.join("vimrc.vim");
    if !default_vimrc.exists() {
        let default_vimrc_content = r#"" Default vimrc.vim for xvim
" This file is loaded at startup

" Set default options
set nocompatible
set backspace=indent,eol,start
set history=50
set ruler
set showcmd
set incsearch
set hlsearch
set ignorecase
set smartcase
set tabstop=4
set shiftwidth=4
set expandtab
set number
set autoindent
set smartindent
set wrap
set linebreak
set showmatch
set wildmenu
set wildmode=list:longest
set laststatus=2
set statusline=%f\ %h%w%m%r\ %=%(%l,%c%V\ %=\ %P%)
"#;
        fs::write(&default_vimrc, default_vimrc_content)?;
        vimscript::execute_file(&default_vimrc)?;
    }
    
    Ok(())
}

/// Create necessary directories
fn create_directories() -> io::Result<()> {
    // Create the xvim directory in the user's home directory
    let home_dir = dirs::home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
    let xvim_dir = home_dir.join(".xvim");
    if !xvim_dir.exists() {
        fs::create_dir_all(&xvim_dir)?;
    }
    
    // Create the backup directory
    let backup_dir = xvim_dir.join("backup");
    if !backup_dir.exists() {
        fs::create_dir_all(&backup_dir)?;
    }
    
    // Create the swap directory
    let swap_dir = xvim_dir.join("swap");
    if !swap_dir.exists() {
        fs::create_dir_all(&swap_dir)?;
    }
    
    // Create the undo directory
    let undo_dir = xvim_dir.join("undo");
    if !undo_dir.exists() {
        fs::create_dir_all(&undo_dir)?;
    }
    
    // Create the view directory
    let view_dir = xvim_dir.join("view");
    if !view_dir.exists() {
        fs::create_dir_all(&view_dir)?;
    }
    
    // Create the spell directory
    let spell_dir = xvim_dir.join("spell");
    if !spell_dir.exists() {
        fs::create_dir_all(&spell_dir)?;
    }
    
    // Create the plugin directory
    let plugin_dir = xvim_dir.join("plugin");
    if !plugin_dir.exists() {
        fs::create_dir_all(&plugin_dir)?;
    }
    
    // Create the ftplugin directory
    let ftplugin_dir = xvim_dir.join("ftplugin");
    if !ftplugin_dir.exists() {
        fs::create_dir_all(&ftplugin_dir)?;
    }
    
    // Create the syntax directory
    let syntax_dir = xvim_dir.join("syntax");
    if !syntax_dir.exists() {
        fs::create_dir_all(&syntax_dir)?;
    }
    
    // Create the indent directory
    let indent_dir = xvim_dir.join("indent");
    if !indent_dir.exists() {
        fs::create_dir_all(&indent_dir)?;
    }
    
    // Create the colors directory
    let colors_dir = xvim_dir.join("colors");
    if !colors_dir.exists() {
        fs::create_dir_all(&colors_dir)?;
    }
    
    // Create the autoload directory
    let autoload_dir = xvim_dir.join("autoload");
    if !autoload_dir.exists() {
        fs::create_dir_all(&autoload_dir)?;
    }
    
    // Create the doc directory
    let doc_dir = xvim_dir.join("doc");
    if !doc_dir.exists() {
        fs::create_dir_all(&doc_dir)?;
    }
    
    Ok(())
}

/// Load plugins
fn load_plugins(editor: &mut Editor, runtime_dir: &Path) -> io::Result<()> {
    // Check if the plugin directory exists in the user's home directory
    let home_dir = dirs::home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
    let home_plugin_dir = home_dir.join(".xvim").join("plugin");
    if home_plugin_dir.exists() && home_plugin_dir.is_dir() {
        // Load all .vim files in the plugin directory
        for entry in fs::read_dir(home_plugin_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "vim") {
                vimscript::execute_file(&path)?;
            }
        }
    }
    
    // Check if the plugin directory exists in the runtime directory
    let runtime_plugin_dir = runtime_dir.join("plugin");
    if runtime_plugin_dir.exists() && runtime_plugin_dir.is_dir() {
        // Load all .vim files in the plugin directory
        for entry in fs::read_dir(runtime_plugin_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "vim") {
                vimscript::execute_file(&path)?;
            }
        }
    }
    
    Ok(())
}

/// Load filetype plugins
fn load_filetype_plugins(editor: &mut Editor, runtime_dir: &Path) -> io::Result<()> {
    // Load the filetype.vim file
    let filetype_vim = runtime_dir.join("filetype.vim");
    if filetype_vim.exists() && filetype_vim.is_file() {
        vimscript::execute_file(&filetype_vim)?;
    }
    
    // Load the ftplugin.vim file
    let ftplugin_vim = runtime_dir.join("ftplugin.vim");
    if ftplugin_vim.exists() && ftplugin_vim.is_file() {
        vimscript::execute_file(&ftplugin_vim)?;
    }
    
    Ok(())
}

/// Load syntax files
fn load_syntax_files(editor: &mut Editor, runtime_dir: &Path) -> io::Result<()> {
    // Load the syntax.vim file
    let syntax_vim = runtime_dir.join("syntax").join("syntax.vim");
    if syntax_vim.exists() && syntax_vim.is_file() {
        vimscript::execute_file(&syntax_vim)?;
    }
    
    Ok(())
}

/// Load indent files
fn load_indent_files(editor: &mut Editor, runtime_dir: &Path) -> io::Result<()> {
    // Load the indent.vim file
    let indent_vim = runtime_dir.join("indent.vim");
    if indent_vim.exists() && indent_vim.is_file() {
        vimscript::execute_file(&indent_vim)?;
    }
    
    Ok(())
}

/// Load color schemes
fn load_color_schemes(editor: &mut Editor, runtime_dir: &Path) -> io::Result<()> {
    // Load the default color scheme
    let default_colors = runtime_dir.join("colors").join("default.vim");
    if default_colors.exists() && default_colors.is_file() {
        vimscript::execute_file(&default_colors)?;
    }
    
    Ok(())
}