//! Configuration for the NoxVim plugin

/// NoxVim configuration
#[derive(Debug, Clone)]
pub struct NoxVimConfig {
    /// API key for the AI service
    pub api_key: String,
    /// API endpoint for the AI service
    pub api_endpoint: String,
    /// AI model to use
    pub model: String,
    /// Maximum context size to send
    pub max_context: usize,
    /// Auto-approve mode
    pub auto_approve: bool,
}

impl Default for NoxVimConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            api_endpoint: "https://api.openai.com/v1".to_string(),
            model: "gpt-4".to_string(),
            max_context: 4000,
            auto_approve: true,
        }
    }
}

impl NoxVimConfig {
    /// Load configuration from xvim settings
    pub fn load() -> Self {
        // This would load configuration from xvim settings
        // For now, just return the default configuration
        Self::default()
    }
    
    /// Save configuration to xvim settings
    pub fn save(&self) -> Result<(), String> {
        // This would save configuration to xvim settings
        // For now, just return success
        Ok(())
    }
    
    /// Set the API key
    pub fn set_api_key(&mut self, api_key: &str) {
        self.api_key = api_key.to_string();
    }
    
    /// Set the API endpoint
    pub fn set_api_endpoint(&mut self, api_endpoint: &str) {
        self.api_endpoint = api_endpoint.to_string();
    }
    
    /// Set the AI model
    pub fn set_model(&mut self, model: &str) {
        self.model = model.to_string();
    }
    
    /// Set the maximum context size
    pub fn set_max_context(&mut self, max_context: usize) {
        self.max_context = max_context;
    }
    
    /// Set auto-approve mode
    pub fn set_auto_approve(&mut self, auto_approve: bool) {
        self.auto_approve = auto_approve;
    }
}