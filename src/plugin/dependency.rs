//! Plugin dependency management
//!
//! This module provides support for managing plugin dependencies.
//! It allows plugins to depend on other plugins and ensures they
//! are loaded in the correct order.

use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::io::{Read, Write};
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use anyhow::{anyhow, Result};
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;
use std::io::Write;
use std::io::Read;
use std::io::Write;

/// Plugin dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependencyInfo {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin dependencies
    pub dependencies: Vec<PluginDependency>,
}

/// Plugin dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    /// Dependency name
    pub name: String,
    /// Dependency version requirement
    pub version_req: String,
    /// Whether the dependency is optional
    pub optional: bool,
}

/// Plugin dependency manager
pub struct DependencyManager {
    /// Plugin dependencies
    dependencies: HashMap<String, PluginDependencyInfo>,
    /// Plugin load order
    load_order: Vec<String>,
}

impl DependencyManager {
    /// Create a new dependency manager
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            load_order: Vec::new(),
        }
    }
    
    /// Register a plugin with its dependencies
    pub fn register_plugin(&mut self, info: PluginDependencyInfo) -> Result<()> {
        // Check if the plugin is already registered
        if self.dependencies.contains_key(&info.name) {
            return Err(anyhow!("Plugin '{}' is already registered", info.name));
        }
        
        // Add the plugin to the dependencies map
        self.dependencies.insert(info.name.clone(), info);
        
        // Recalculate the load order
        self.calculate_load_order()?;
        
        Ok(())
    }
    
    /// Unregister a plugin
    pub fn unregister_plugin(&mut self, name: &str) -> Result<()> {
        // Check if the plugin is registered
        if !self.dependencies.contains_key(name) {
            return Err(anyhow!("Plugin '{}' is not registered", name));
        }
        
        // Check if any other plugins depend on this one
        for (plugin_name, info) in &self.dependencies {
            if plugin_name != name {
                for dep in &info.dependencies {
                    if dep.name == name && !dep.optional {
                        return Err(anyhow!("Cannot unregister plugin '{}' because '{}' depends on it", name, plugin_name));
                    }
                }
            }
        }
        
        // Remove the plugin from the dependencies map
        self.dependencies.remove(name);
        
        // Recalculate the load order
        self.calculate_load_order()?;
        
        Ok(())
    }
    
    /// Get the plugin load order
    pub fn load_order(&self) -> &[String] {
        &self.load_order
    }
    
    /// Get plugin dependency information
    pub fn get_plugin_info(&self, name: &str) -> Option<&PluginDependencyInfo> {
        self.dependencies.get(name)
    }
    
    /// List all registered plugins
    pub fn list_plugins(&self) -> Vec<&PluginDependencyInfo> {
        self.dependencies.values().collect()
    }
    
    /// Check if a plugin is registered
    pub fn is_plugin_registered(&self, name: &str) -> bool {
        self.dependencies.contains_key(name)
    }
    
    /// Get the dependencies of a plugin
    pub fn get_plugin_dependencies(&self, name: &str) -> Option<&[PluginDependency]> {
        self.dependencies.get(name).map(|info| info.dependencies.as_slice())
    }
    
    /// Check if a plugin version satisfies a version requirement
    pub fn check_version_requirement(&self, version: &str, version_req: &str) -> bool {
        // TODO: Implement proper semantic version checking
        // For now, just check if the versions match exactly
        version == version_req
    }
    
    /// Calculate the plugin load order
    fn calculate_load_order(&mut self) -> Result<()> {
        // Clear the current load order
        self.load_order.clear();
        
        // Create a set of visited plugins
        let mut visited = HashSet::new();
        
        // Create a set of plugins in the current path (for cycle detection)
        let mut path = HashSet::new();
        
        // Get a list of plugin names first to avoid borrowing issues
        let plugin_names: Vec<String> = self.dependencies.keys().cloned().collect();
        
        // Process each plugin
        for name in plugin_names {
            if !visited.contains(&name) {
                self.visit_plugin(&name, &mut visited, &mut path)?;
            }
        }
        
        Ok(())
    }
    
    /// Visit a plugin and its dependencies
    fn visit_plugin(&mut self, name: &str, visited: &mut HashSet<String>, path: &mut HashSet<String>) -> Result<()> {
        // Check if the plugin is in the current path (cycle detection)
        if path.contains(name) {
            return Err(anyhow!("Dependency cycle detected: {}", name));
        }
        
        // Add the plugin to the current path
        path.insert(name.to_string());
        
        // Get the plugin info and clone the dependencies to avoid borrowing issues
        let deps = {
            let info = self.dependencies.get(name)
                .ok_or_else(|| anyhow!("Plugin '{}' not found", name))?;
            info.dependencies.clone()
        };
        
        // Visit each dependency
        for dep in &deps {
            if !dep.optional && self.dependencies.contains_key(&dep.name) && !visited.contains(&dep.name) {
                self.visit_plugin(&dep.name, visited, path)?;
            }
        }
        
        // Remove the plugin from the current path
        path.remove(name);
        
        // Add the plugin to the visited set
        visited.insert(name.to_string());
        
        // Add the plugin to the load order
        self.load_order.push(name.to_string());
        
        Ok(())
    }
    
    /// Load plugin dependency information from a file
    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        // Read the file
        let content = std::fs::read_to_string(path)?;
        
        // Parse the JSON
        let info: PluginDependencyInfo = serde_json::from_str(&content)?;
        
        // Register the plugin
        self.register_plugin(info)?;
        
        Ok(())
    }
    
    /// Save plugin dependency information to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, name: &str, path: P) -> Result<()> {
        // Get the plugin info
        let info = self.dependencies.get(name)
            .ok_or_else(|| anyhow!("Plugin '{}' not found", name))?;
        
        // Serialize to JSON
        let content = serde_json::to_string_pretty(info)?;
        
        // Write to the file
        std::fs::write(path, content)?;
        
        Ok(())
    }
    
    /// Check if all dependencies are satisfied
    pub fn check_dependencies(&self) -> Result<()> {
        // Check each plugin
        for (name, info) in &self.dependencies {
            // Check each dependency
            for dep in &info.dependencies {
                // Skip optional dependencies
                if dep.optional {
                    continue;
                }
                
                // Check if the dependency exists
                if let Some(dep_info) = self.dependencies.get(&dep.name) {
                    // Check if the version requirement is satisfied
                    if !self.check_version_requirement(&dep_info.version, &dep.version_req) {
                        return Err(anyhow!("Plugin '{}' requires '{}' version '{}', but version '{}' is installed",
                            name, dep.name, dep.version_req, dep_info.version));
                    }
                } else {
                    return Err(anyhow!("Plugin '{}' depends on '{}', which is not installed", name, dep.name));
                }
            }
        }
        
        Ok(())
    }
    
    /// Resolve plugin dependencies
    pub fn resolve_dependencies(&self, name: &str) -> Result<Vec<String>> {
        // Check if the plugin exists
        if !self.dependencies.contains_key(name) {
            return Err(anyhow!("Plugin '{}' not found", name));
        }
        
        // Create a set of visited plugins
        let mut visited = HashSet::new();
        
        // Create a queue of plugins to visit
        let mut queue = VecDeque::new();
        queue.push_back(name.to_string());
        
        // Create a list of dependencies
        let mut dependencies = Vec::new();
        
        // Process the queue
        while let Some(plugin_name) = queue.pop_front() {
            // Skip if already visited
            if visited.contains(&plugin_name) {
                continue;
            }
            
            // Mark as visited
            visited.insert(plugin_name.clone());
            
            // Get the plugin info
            if let Some(info) = self.dependencies.get(&plugin_name) {
                // Add each dependency to the queue
                for dep in &info.dependencies {
                    if !dep.optional && self.dependencies.contains_key(&dep.name) {
                        queue.push_back(dep.name.clone());
                    }
                }
                
                // Add the plugin to the dependencies list
                dependencies.push(plugin_name);
            }
        }
        
        // Remove the original plugin from the dependencies
        dependencies.retain(|dep| dep != name);
        
        Ok(dependencies)
    }
}

impl Default for DependencyManager {
    fn default() -> Self {
        Self::new()
    }
}