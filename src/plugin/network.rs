//! Network access security model for plugins
//!
//! This module provides a secure way for plugins to access network resources.
//! It implements a permission-based security model that allows plugins to
//! make network requests only to approved domains and with specific methods.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// Network permission level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionLevel {
    /// No network access allowed
    None,
    /// Only localhost access allowed
    Localhost,
    /// Only specific domains allowed
    Restricted,
    /// All domains allowed
    All,
}

/// HTTP method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
    /// GET method
    Get,
    /// POST method
    Post,
    /// PUT method
    Put,
    /// DELETE method
    Delete,
    /// HEAD method
    Head,
    /// OPTIONS method
    Options,
    /// PATCH method
    Patch,
}

/// Network permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPermission {
    /// Permission level
    level: PermissionLevel,
    /// Allowed domains
    allowed_domains: HashSet<String>,
    /// Allowed methods
    allowed_methods: HashSet<HttpMethod>,
    /// Maximum request size in bytes
    max_request_size: usize,
    /// Maximum response size in bytes
    max_response_size: usize,
    /// Maximum requests per minute
    max_requests_per_minute: usize,
}

impl NetworkPermission {
    /// Create a new network permission with no access
    pub fn none() -> Self {
        Self {
            level: PermissionLevel::None,
            allowed_domains: HashSet::new(),
            allowed_methods: HashSet::new(),
            max_request_size: 0,
            max_response_size: 0,
            max_requests_per_minute: 0,
        }
    }
    
    /// Create a new network permission with localhost access only
    pub fn localhost() -> Self {
        let mut methods = HashSet::new();
        methods.insert(HttpMethod::Get);
        methods.insert(HttpMethod::Post);
        
        Self {
            level: PermissionLevel::Localhost,
            allowed_domains: HashSet::new(),
            allowed_methods: methods,
            max_request_size: 1024 * 1024, // 1 MB
            max_response_size: 1024 * 1024, // 1 MB
            max_requests_per_minute: 60,
        }
    }
    
    /// Create a new network permission with restricted access
    pub fn restricted(domains: &[&str], methods: &[HttpMethod]) -> Self {
        let mut allowed_domains = HashSet::new();
        for domain in domains {
            allowed_domains.insert(domain.to_string());
        }
        
        let mut allowed_methods = HashSet::new();
        for method in methods {
            allowed_methods.insert(*method);
        }
        
        Self {
            level: PermissionLevel::Restricted,
            allowed_domains,
            allowed_methods,
            max_request_size: 1024 * 1024, // 1 MB
            max_response_size: 1024 * 1024, // 1 MB
            max_requests_per_minute: 60,
        }
    }
    
    /// Create a new network permission with full access
    pub fn all() -> Self {
        let mut methods = HashSet::new();
        methods.insert(HttpMethod::Get);
        methods.insert(HttpMethod::Post);
        methods.insert(HttpMethod::Put);
        methods.insert(HttpMethod::Delete);
        methods.insert(HttpMethod::Head);
        methods.insert(HttpMethod::Options);
        methods.insert(HttpMethod::Patch);
        
        Self {
            level: PermissionLevel::All,
            allowed_domains: HashSet::new(),
            allowed_methods: methods,
            max_request_size: 10 * 1024 * 1024, // 10 MB
            max_response_size: 10 * 1024 * 1024, // 10 MB
            max_requests_per_minute: 600,
        }
    }
    
    /// Check if a domain is allowed
    pub fn is_domain_allowed(&self, domain: &str) -> bool {
        match self.level {
            PermissionLevel::None => false,
            PermissionLevel::Localhost => domain == "localhost" || domain == "127.0.0.1",
            PermissionLevel::Restricted => self.allowed_domains.contains(domain),
            PermissionLevel::All => true,
        }
    }
    
    /// Check if a method is allowed
    pub fn is_method_allowed(&self, method: HttpMethod) -> bool {
        self.allowed_methods.contains(&method)
    }
    
    /// Check if a request size is allowed
    pub fn is_request_size_allowed(&self, size: usize) -> bool {
        size <= self.max_request_size
    }
    
    /// Check if a response size is allowed
    pub fn is_response_size_allowed(&self, size: usize) -> bool {
        size <= self.max_response_size
    }
    
    /// Add an allowed domain
    pub fn add_allowed_domain(&mut self, domain: &str) {
        self.allowed_domains.insert(domain.to_string());
    }
    
    /// Remove an allowed domain
    pub fn remove_allowed_domain(&mut self, domain: &str) {
        self.allowed_domains.remove(domain);
    }
    
    /// Add an allowed method
    pub fn add_allowed_method(&mut self, method: HttpMethod) {
        self.allowed_methods.insert(method);
    }
    
    /// Remove an allowed method
    pub fn remove_allowed_method(&mut self, method: HttpMethod) {
        self.allowed_methods.remove(&method);
    }
    
    /// Set the maximum request size
    pub fn set_max_request_size(&mut self, size: usize) {
        self.max_request_size = size;
    }
    
    /// Set the maximum response size
    pub fn set_max_response_size(&mut self, size: usize) {
        self.max_response_size = size;
    }
    
    /// Set the maximum requests per minute
    pub fn set_max_requests_per_minute(&mut self, count: usize) {
        self.max_requests_per_minute = count;
    }
}

/// Network request
#[derive(Debug, Clone)]
pub struct NetworkRequest {
    /// Request URL
    url: String,
    /// Request method
    method: HttpMethod,
    /// Request headers
    headers: HashMap<String, String>,
    /// Request body
    body: Option<Vec<u8>>,
}

impl NetworkRequest {
    /// Create a new network request
    pub fn new(url: &str, method: HttpMethod) -> Self {
        Self {
            url: url.to_string(),
            method,
            headers: HashMap::new(),
            body: None,
        }
    }
    
    /// Get the request URL
    pub fn url(&self) -> &str {
        &self.url
    }
    
    /// Get the request method
    pub fn method(&self) -> HttpMethod {
        self.method
    }
    
    /// Get the request headers
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
    
    /// Get a request header
    pub fn header(&self, name: &str) -> Option<&String> {
        self.headers.get(name)
    }
    
    /// Set a request header
    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_string(), value.to_string());
    }
    
    /// Remove a request header
    pub fn remove_header(&mut self, name: &str) {
        self.headers.remove(name);
    }
    
    /// Get the request body
    pub fn body(&self) -> Option<&Vec<u8>> {
        self.body.as_ref()
    }
    
    /// Set the request body
    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = Some(body);
    }
    
    /// Clear the request body
    pub fn clear_body(&mut self) {
        self.body = None;
    }
    
    /// Get the domain from the URL
    pub fn domain(&self) -> Result<String> {
        // Parse the URL to extract the domain
        let url = url::Url::parse(&self.url)?;
        let host = url.host_str().ok_or_else(|| anyhow!("Invalid URL: no host"))?;
        Ok(host.to_string())
    }
    
    /// Get the request size in bytes
    pub fn size(&self) -> usize {
        let mut size = self.url.len();
        
        for (name, value) in &self.headers {
            size += name.len() + value.len() + 2; // +2 for ": "
        }
        
        if let Some(body) = &self.body {
            size += body.len();
        }
        
        size
    }
}

/// Network response
#[derive(Debug, Clone)]
pub struct NetworkResponse {
    /// Response status code
    status: u16,
    /// Response headers
    headers: HashMap<String, String>,
    /// Response body
    body: Vec<u8>,
}

impl NetworkResponse {
    /// Create a new network response
    pub fn new(status: u16, body: Vec<u8>) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body,
        }
    }
    
    /// Get the response status code
    pub fn status(&self) -> u16 {
        self.status
    }
    
    /// Get the response headers
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
    
    /// Get a response header
    pub fn header(&self, name: &str) -> Option<&String> {
        self.headers.get(name)
    }
    
    /// Set a response header
    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_string(), value.to_string());
    }
    
    /// Get the response body
    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }
    
    /// Get the response size in bytes
    pub fn size(&self) -> usize {
        let mut size = 0;
        
        for (name, value) in &self.headers {
            size += name.len() + value.len() + 2; // +2 for ": "
        }
        
        size += self.body.len();
        
        size
    }
}

/// Plugin request tracker
struct PluginRequestTracker {
    /// Plugin name
    plugin_name: String,
    /// Request count in the current minute
    request_count: usize,
    /// Last request time
    last_request_time: std::time::Instant,
}

impl PluginRequestTracker {
    /// Create a new plugin request tracker
    fn new(plugin_name: &str) -> Self {
        Self {
            plugin_name: plugin_name.to_string(),
            request_count: 0,
            last_request_time: std::time::Instant::now(),
        }
    }
    
    /// Check if a request is allowed
    fn is_request_allowed(&mut self, max_requests_per_minute: usize) -> bool {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_request_time);
        
        // Reset the counter if a minute has passed
        if elapsed.as_secs() >= 60 {
            self.request_count = 0;
            self.last_request_time = now;
        }
        
        // Check if the request count is below the limit
        if self.request_count < max_requests_per_minute {
            self.request_count += 1;
            true
        } else {
            false
        }
    }
}

/// Network manager
pub struct NetworkManager {
    /// Plugin permissions
    permissions: HashMap<String, NetworkPermission>,
    /// Plugin request trackers
    request_trackers: HashMap<String, PluginRequestTracker>,
}

impl NetworkManager {
    /// Create a new network manager
    pub fn new() -> Self {
        Self {
            permissions: HashMap::new(),
            request_trackers: HashMap::new(),
        }
    }
    
    /// Set the permission for a plugin
    pub fn set_permission(&mut self, plugin_name: &str, permission: NetworkPermission) {
        self.permissions.insert(plugin_name.to_string(), permission);
    }
    
    /// Get the permission for a plugin
    pub fn get_permission(&self, plugin_name: &str) -> Option<&NetworkPermission> {
        self.permissions.get(plugin_name)
    }
    
    /// Remove the permission for a plugin
    pub fn remove_permission(&mut self, plugin_name: &str) -> Option<NetworkPermission> {
        self.permissions.remove(plugin_name)
    }
    
    /// Check if a request is allowed
    pub fn is_request_allowed(&mut self, plugin_name: &str, request: &NetworkRequest) -> Result<bool> {
        // Get the permission for the plugin
        let permission = self.permissions.get(plugin_name)
            .ok_or_else(|| anyhow!("No permission found for plugin '{}'", plugin_name))?;
        
        // Check if the domain is allowed
        let domain = request.domain()?;
        if !permission.is_domain_allowed(&domain) {
            return Ok(false);
        }
        
        // Check if the method is allowed
        if !permission.is_method_allowed(request.method()) {
            return Ok(false);
        }
        
        // Check if the request size is allowed
        if !permission.is_request_size_allowed(request.size()) {
            return Ok(false);
        }
        
        // Check if the request rate is allowed
        let tracker = self.request_trackers.entry(plugin_name.to_string())
            .or_insert_with(|| PluginRequestTracker::new(plugin_name));
        
        if !tracker.is_request_allowed(permission.max_requests_per_minute) {
            return Ok(false);
        }
        
        Ok(true)
    }
    
    /// Send a network request
    pub fn send_request(&mut self, plugin_name: &str, request: &NetworkRequest) -> Result<NetworkResponse> {
        // Check if the request is allowed
        if !self.is_request_allowed(plugin_name, request)? {
            return Err(anyhow!("Request not allowed"));
        }
        
        // TODO: Implement actual HTTP request sending
        // For now, just return a dummy response
        
        let response = NetworkResponse::new(200, b"OK".to_vec());
        
        // Get the permission for the plugin
        let permission = self.permissions.get(plugin_name)
            .ok_or_else(|| anyhow!("No permission found for plugin '{}'", plugin_name))?;
        
        // Check if the response size is allowed
        if !permission.is_response_size_allowed(response.size()) {
            return Err(anyhow!("Response size exceeds the limit"));
        }
        
        Ok(response)
    }
}

impl Default for NetworkManager {
    fn default() -> Self {
        Self::new()
    }
}