//! xvim - A ground-up rewrite of the Vim text editor in pure Rust with a WASM plugin system

use clap::Parser;
use log::{info, LevelFilter};
use xvim::editor;

/// Command line arguments for xvim
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Files to open
    #[clap(name = "FILE")]
    files: Vec<String>,

    /// Enable debug logging
    #[clap(short, long)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging
    let log_level = if args.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    
    // TODO: Set up proper logging
    println!("xvim starting up...");
    
    // Initialize the editor
    let mut editor = editor::Editor::new()?;
    
    // Open files specified on the command line
    for file in args.files {
        match editor.open_file(&file) {
            Ok(_) => println!("Opened file: {}", file),
            Err(e) => eprintln!("Error opening file {}: {}", file, e),
        }
    }
    
    // Run the editor
    editor.run()?;
    
    Ok(())
}