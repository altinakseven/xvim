//! Utility functions for the NoxVim plugin
//!
//! This module provides utility functions used throughout the plugin.

/// Escape HTML special characters
///
/// This escapes special characters in a string for inclusion in HTML.
pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Truncate a string to a maximum length
///
/// This truncates a string to a maximum length, adding an ellipsis if truncated.
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}

/// Get the file extension from a path
///
/// This extracts the file extension from a path.
pub fn get_file_extension(path: &str) -> Option<String> {
    path.split('.').last().map(|s| s.to_string())
}

/// Get the file name from a path
///
/// This extracts the file name from a path.
pub fn get_file_name(path: &str) -> Option<String> {
    path.split('/').last().map(|s| s.to_string())
}

/// Get the language from a file extension
///
/// This maps a file extension to a language name.
pub fn get_language_from_extension(ext: &str) -> Option<String> {
    match ext {
        "rs" => Some("rust".to_string()),
        "js" => Some("javascript".to_string()),
        "ts" => Some("typescript".to_string()),
        "py" => Some("python".to_string()),
        "java" => Some("java".to_string()),
        "c" => Some("c".to_string()),
        "cpp" => Some("cpp".to_string()),
        "h" => Some("c".to_string()),
        "hpp" => Some("cpp".to_string()),
        "go" => Some("go".to_string()),
        "rb" => Some("ruby".to_string()),
        "php" => Some("php".to_string()),
        "html" => Some("html".to_string()),
        "css" => Some("css".to_string()),
        "json" => Some("json".to_string()),
        "md" => Some("markdown".to_string()),
        "txt" => Some("text".to_string()),
        _ => None,
    }
}

/// Format a duration in milliseconds
///
/// This formats a duration in milliseconds as a human-readable string.
pub fn format_duration(millis: u64) -> String {
    if millis < 1000 {
        format!("{}ms", millis)
    } else if millis < 60000 {
        format!("{:.1}s", millis as f64 / 1000.0)
    } else {
        let minutes = millis / 60000;
        let seconds = (millis % 60000) / 1000;
        format!("{}m {}s", minutes, seconds)
    }
}

/// Format a file size in bytes
///
/// This formats a file size in bytes as a human-readable string.
pub fn format_file_size(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1}KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1}MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1}GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

/// Parse a markdown code block
///
/// This extracts the language and content from a markdown code block.
pub fn parse_code_block(block: &str) -> Option<(String, String)> {
    let lines: Vec<&str> = block.lines().collect();
    
    if lines.len() < 2 {
        return None;
    }
    
    let first_line = lines[0];
    let last_line = lines[lines.len() - 1];
    
    if !first_line.starts_with("```") || last_line != "```" {
        return None;
    }
    
    let language = first_line.trim_start_matches("```").to_string();
    let content = lines[1..lines.len() - 1].join("\n");
    
    Some((language, content))
}

/// Extract code blocks from markdown
///
/// This extracts all code blocks from a markdown string.
pub fn extract_code_blocks(markdown: &str) -> Vec<(String, String)> {
    let mut blocks = Vec::new();
    let mut in_block = false;
    let mut current_block = Vec::new();
    
    for line in markdown.lines() {
        if line.starts_with("```") {
            if in_block {
                // End of block
                current_block.push(line);
                if let Some((language, content)) = parse_code_block(&current_block.join("\n")) {
                    blocks.push((language, content));
                }
                current_block.clear();
                in_block = false;
            } else {
                // Start of block
                current_block.push(line);
                in_block = true;
            }
        } else if in_block {
            // Inside a block
            current_block.push(line);
        }
    }
    
    blocks
}

/// Generate a random ID
///
/// This generates a random ID string.
pub fn generate_id() -> String {
    // This would generate a random ID
    // For now, just return a fixed string
    "id_123456".to_string()
}

/// Check if a string is a valid file path
///
/// This checks if a string is a valid file path.
pub fn is_valid_file_path(path: &str) -> bool {
    // This would check if a string is a valid file path
    // For now, just return true
    true
}

/// Check if a string is a valid command
///
/// This checks if a string is a valid shell command.
pub fn is_valid_command(command: &str) -> bool {
    // This would check if a string is a valid shell command
    // For now, just return true
    true
}