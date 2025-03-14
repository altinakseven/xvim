//! Configuration module for xvim
//!
//! This module handles loading, saving, and accessing configuration settings
//! for the editor, including user preferences and key mappings.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use toml;

use crate::keymap::KeyMapping;
use crate::mode::Mode;

/// Errors that can occur during configuration operations
#[derive(Debug)]
pub enum ConfigError {
    /// I/O error
    Io(io::Error),
    /// TOML parsing error
    Parse(toml::de::Error),
    /// TOML serialization error
    Serialize(toml::ser::Error),
    /// Other errors
    Other(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(err) => write!(f, "I/O error: {}", err),
            ConfigError::Parse(err) => write!(f, "Parse error: {}", err),
            ConfigError::Serialize(err) => write!(f, "Serialization error: {}", err),
            ConfigError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for ConfigError {}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::Io(err)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::Parse(err)
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(err: toml::ser::Error) -> Self {
        ConfigError::Serialize(err)
    }
}

/// Result type for configuration operations
pub type ConfigResult<T> = Result<T, ConfigError>;

/// Editor configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// General editor settings
    #[serde(default)]
    pub general: GeneralConfig,
    
    /// UI settings
    #[serde(default)]
    pub ui: UiConfig,
    
    /// Key mappings
    #[serde(default)]
    pub keymaps: KeymapConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            ui: UiConfig::default(),
            keymaps: KeymapConfig::default(),
        }
    }
}

/// General editor settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Whether to use the system clipboard
    #[serde(default = "default_true")]
    pub use_system_clipboard: bool,
    
    /// Whether to show line numbers
    #[serde(default = "default_true")]
    pub show_line_numbers: bool,
    
    /// Whether to enable syntax highlighting
    #[serde(default = "default_true")]
    pub syntax_highlighting: bool,
    
    /// Whether to enable auto-indentation
    #[serde(default = "default_true")]
    pub auto_indent: bool,
    
    /// Tab width in spaces
    #[serde(default = "default_tab_width")]
    pub tab_width: u8,
    
    /// Whether to expand tabs to spaces
    #[serde(default = "default_true")]
    pub expand_tabs: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            use_system_clipboard: true,
            show_line_numbers: true,
            syntax_highlighting: true,
            auto_indent: true,
            tab_width: 4,
            expand_tabs: true,
        }
    }
}

/// UI settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Color scheme
    #[serde(default = "default_color_scheme")]
    pub color_scheme: String,
    
    /// Font family
    #[serde(default = "default_font_family")]
    pub font_family: String,
    
    /// Font size
    #[serde(default = "default_font_size")]
    pub font_size: u8,
    
    /// Whether to show the status line
    #[serde(default = "default_true")]
    pub show_status_line: bool,
    
    /// Whether to show the command line
    #[serde(default = "default_true")]
    pub show_command_line: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            color_scheme: default_color_scheme(),
            font_family: default_font_family(),
            font_size: default_font_size(),
            show_status_line: true,
            show_command_line: true,
        }
    }
}

/// Key mapping configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeymapConfig {
    /// Normal mode key mappings
    #[serde(default)]
    pub normal: HashMap<String, String>,
    
    /// Insert mode key mappings
    #[serde(default)]
    pub insert: HashMap<String, String>,
    
    /// Visual mode key mappings
    #[serde(default)]
    pub visual: HashMap<String, String>,
    
    /// Command mode key mappings
    #[serde(default)]
    pub command: HashMap<String, String>,
}

/// Default value functions for serde
fn default_true() -> bool {
    true
}

fn default_tab_width() -> u8 {
    4
}

fn default_color_scheme() -> String {
    "default".to_string()
}

fn default_font_family() -> String {
    "monospace".to_string()
}

fn default_font_size() -> u8 {
    12
}

/// Configuration manager
pub struct ConfigManager {
    /// The current configuration
    config: Config,
    /// The path to the configuration file
    config_path: PathBuf,
}

impl ConfigManager {
    /// Create a new configuration manager with the default configuration
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            config_path: Self::default_config_path(),
        }
    }
    
    /// Create a new configuration manager with a custom configuration path
    pub fn with_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            config: Config::default(),
            config_path: path.as_ref().to_path_buf(),
        }
    }
    
    /// Get the default configuration path
    pub fn default_config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("xvim");
        path.push("config.toml");
        path
    }
    
    /// Load configuration from the default path
    pub fn load(&mut self) -> ConfigResult<()> {
        let path = self.config_path.clone();
        self.load_from(&path)
    }
    
    /// Load configuration from a specific path
    pub fn load_from<P: AsRef<Path>>(&mut self, path: P) -> ConfigResult<()> {
        let path = path.as_ref();
        
        // If the file doesn't exist, use the default configuration
        if !path.exists() {
            self.config = Config::default();
            return Ok(());
        }
        
        // Read the file
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        // Parse the TOML
        let config: Config = toml::from_str(&contents)?;
        self.config = config;
        
        Ok(())
    }
    
    /// Save the current configuration to the default path
    pub fn save(&self) -> ConfigResult<()> {
        self.save_to(&self.config_path)
    }
    
    /// Save the current configuration to a specific path
    pub fn save_to<P: AsRef<Path>>(&self, path: P) -> ConfigResult<()> {
        let path = path.as_ref();
        
        // Create the parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Serialize the configuration to TOML
        let toml = toml::to_string_pretty(&self.config)?;
        
        // Write to the file
        let mut file = File::create(path)?;
        file.write_all(toml.as_bytes())?;
        
        Ok(())
    }
    
    /// Get a reference to the current configuration
    pub fn config(&self) -> &Config {
        &self.config
    }
    
    /// Get a mutable reference to the current configuration
    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }
    
    /// Get the path to the configuration file
    pub fn config_path(&self) -> &Path {
        &self.config_path
    }
    
    /// Set the path to the configuration file
    pub fn set_config_path<P: AsRef<Path>>(&mut self, path: P) {
        self.config_path = path.as_ref().to_path_buf();
    }
    
    /// Convert key mappings from the configuration to KeyMapping objects
    pub fn get_key_mappings(&self) -> Vec<KeyMapping> {
        let mut mappings = Vec::new();
        
        // Normal mode mappings
        for (key_str, cmd_str) in &self.config.keymaps.normal {
            if let Some(mapping) = self.parse_key_mapping(key_str, cmd_str, Mode::Normal) {
                mappings.push(mapping);
            }
        }
        
        // Insert mode mappings
        for (key_str, cmd_str) in &self.config.keymaps.insert {
            if let Some(mapping) = self.parse_key_mapping(key_str, cmd_str, Mode::Insert) {
                mappings.push(mapping);
            }
        }
        
        // Visual mode mappings
        for (key_str, cmd_str) in &self.config.keymaps.visual {
            if let Some(mapping) = self.parse_key_mapping(key_str, cmd_str, Mode::Visual) {
                mappings.push(mapping);
            }
        }
        
        // Command mode mappings
        for (key_str, cmd_str) in &self.config.keymaps.command {
            if let Some(mapping) = self.parse_key_mapping(key_str, cmd_str, Mode::Command) {
                mappings.push(mapping);
            }
        }
        
        mappings
    }
    
    /// Parse a key mapping from strings
    fn parse_key_mapping(&self, _key_str: &str, _cmd_str: &str, _mode: Mode) -> Option<KeyMapping> {
        // TODO: Implement parsing of key sequences and commands
        // For now, just return None
        None
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.general.tab_width, 4);
        assert_eq!(config.general.expand_tabs, true);
        assert_eq!(config.ui.color_scheme, "default");
    }
    
    #[test]
    fn test_save_load_config() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        
        // Create a config with custom settings
        let mut config = Config::default();
        config.general.tab_width = 2;
        config.ui.color_scheme = "monokai".to_string();
        
        // Add some key mappings
        config.keymaps.normal.insert("jj".to_string(), "escape".to_string());
        
        // Save the config
        let mut manager = ConfigManager::with_path(path);
        *manager.config_mut() = config.clone();
        manager.save().unwrap();
        
        // Load the config
        let mut new_manager = ConfigManager::with_path(path);
        new_manager.load().unwrap();
        
        // Check that the loaded config matches the original
        assert_eq!(new_manager.config().general.tab_width, 2);
        assert_eq!(new_manager.config().ui.color_scheme, "monokai");
        assert_eq!(
            new_manager.config().keymaps.normal.get("jj").unwrap(),
            "escape"
        );
    }
}