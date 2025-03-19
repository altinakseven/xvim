use std::path::Path;
use std::sync::Arc;

#[test]
fn test_load_colorscheme() {
    // Create a color scheme registry
    let mut registry = xvim::syntax::ColorSchemeRegistry::new();
    
    // Load color schemes from the runtime directory
    let runtime_dir = Path::new("runtime");
    let colors_dir = runtime_dir.join("colors");
    
    // Load color schemes
    let count = registry.load_from_directory(&colors_dir).unwrap();
    
    // Verify that at least one color scheme was loaded
    assert!(count > 0, "No color schemes were loaded");
    
    // Verify that the default color scheme exists
    assert!(registry.get("default").is_some(), "Default color scheme not found");
    
    // Verify that the monokai color scheme exists
    assert!(registry.get("monokai").is_some(), "Monokai color scheme not found");
    
    // Verify that the solarized color scheme exists
    assert!(registry.get("solarized").is_some(), "Solarized color scheme not found");
}

#[test]
fn test_set_colorscheme() {
    // Create a color scheme registry
    let mut registry = xvim::syntax::ColorSchemeRegistry::new();
    
    // Load color schemes from the runtime directory
    let runtime_dir = Path::new("runtime");
    let colors_dir = runtime_dir.join("colors");
    
    // Load color schemes
    registry.load_from_directory(&colors_dir).unwrap();
    
    // Set the current color scheme to monokai
    assert!(registry.set_current("monokai"), "Failed to set color scheme to monokai");
    
    // Verify that the current color scheme is monokai
    let current = registry.current().unwrap();
    assert_eq!(current.name, "monokai", "Current color scheme is not monokai");
    
    // Set the current color scheme to solarized
    assert!(registry.set_current("solarized"), "Failed to set color scheme to solarized");
    
    // Verify that the current color scheme is solarized
    let current = registry.current().unwrap();
    assert_eq!(current.name, "solarized", "Current color scheme is not solarized");
    
    // Set the current color scheme to a non-existent color scheme
    assert!(!registry.set_current("nonexistent"), "Set color scheme to nonexistent succeeded");
}

#[test]
fn test_colorscheme_theme() {
    // Create a color scheme registry
    let mut registry = xvim::syntax::ColorSchemeRegistry::new();
    
    // Load color schemes from the runtime directory
    let runtime_dir = Path::new("runtime");
    let colors_dir = runtime_dir.join("colors");
    
    // Load color schemes
    registry.load_from_directory(&colors_dir).unwrap();
    
    // Get the monokai color scheme
    let monokai = registry.get("monokai").unwrap();
    
    // Verify that the theme has styles
    assert!(!monokai.theme.styles.is_empty(), "Monokai theme has no styles");
    
    // Get the solarized color scheme
    let solarized = registry.get("solarized").unwrap();
    
    // Verify that the theme has styles
    assert!(!solarized.theme.styles.is_empty(), "Solarized theme has no styles");
}