//! Tab management for the terminal UI
//!
//! This module provides functionality for managing multiple tabs in the terminal UI.
//! Each tab can contain multiple windows, and the user can switch between tabs.

use std::collections::HashMap;
use crate::ui::window::{WindowManager, Window};

/// Tab ID type
pub type TabId = usize;

/// Tab structure
#[derive(Debug)]
pub struct Tab {
    /// Tab ID
    pub id: TabId,
    /// Tab name
    pub name: String,
    /// Window manager for this tab
    pub window_manager: WindowManager,
    /// Whether this tab is modified
    pub modified: bool,
}

impl Tab {
    /// Create a new tab
    pub fn new(id: TabId, name: String) -> Self {
        Self {
            id,
            name,
            window_manager: WindowManager::new(),
            modified: false,
        }
    }

    /// Set the tab name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Mark the tab as modified
    pub fn mark_modified(&mut self, modified: bool) {
        self.modified = modified;
    }

    /// Get the current window ID
    pub fn current_window_id(&self) -> Option<usize> {
        if self.window_manager.windows().is_empty() {
            None
        } else {
            Some(self.window_manager.current_window_id())
        }
    }

    /// Get a reference to the current window
    pub fn current_window(&self) -> Option<&Window> {
        self.window_manager.current_window()
    }

    /// Get a mutable reference to the current window
    pub fn current_window_mut(&mut self) -> Option<&mut Window> {
        self.window_manager.current_window_mut()
    }
}

/// Tab manager
#[derive(Debug)]
pub struct TabManager {
    /// Tabs
    tabs: HashMap<TabId, Tab>,
    /// Current tab ID
    current_tab_id: Option<TabId>,
    /// Next tab ID
    next_tab_id: TabId,
}

impl TabManager {
    /// Create a new tab manager
    pub fn new() -> Self {
        Self {
            tabs: HashMap::new(),
            current_tab_id: None,
            next_tab_id: 1,
        }
    }

    /// Create a new tab
    pub fn create_tab(&mut self, name: String) -> TabId {
        let id = self.next_tab_id;
        self.next_tab_id += 1;

        let tab = Tab::new(id, name);
        self.tabs.insert(id, tab);

        // If this is the first tab, make it the current tab
        if self.current_tab_id.is_none() {
            self.current_tab_id = Some(id);
        }

        id
    }

    /// Get a reference to a tab by ID
    pub fn get_tab(&self, id: TabId) -> Option<&Tab> {
        self.tabs.get(&id)
    }

    /// Get a mutable reference to a tab by ID
    pub fn get_tab_mut(&mut self, id: TabId) -> Option<&mut Tab> {
        self.tabs.get_mut(&id)
    }

    /// Get a reference to the current tab
    pub fn current_tab(&self) -> Option<&Tab> {
        self.current_tab_id.and_then(|id| self.get_tab(id))
    }

    /// Get a mutable reference to the current tab
    pub fn current_tab_mut(&mut self) -> Option<&mut Tab> {
        let id = self.current_tab_id?;
        self.get_tab_mut(id)
    }

    /// Set the current tab
    pub fn set_current_tab(&mut self, id: TabId) -> bool {
        if self.tabs.contains_key(&id) {
            self.current_tab_id = Some(id);
            true
        } else {
            false
        }
    }

    /// Get the current tab ID
    pub fn current_tab_id(&self) -> Option<TabId> {
        self.current_tab_id
    }

    /// Close a tab by ID
    pub fn close_tab(&mut self, id: TabId) -> bool {
        if !self.tabs.contains_key(&id) {
            return false;
        }

        // Remove the tab
        self.tabs.remove(&id);

        // If we closed the current tab, select another one
        if Some(id) == self.current_tab_id {
            self.current_tab_id = self.tabs.keys().next().copied();
        }

        true
    }

    /// Navigate to the next tab
    pub fn next_tab(&mut self) -> bool {
        if self.tabs.is_empty() {
            return false;
        }

        let current_id = match self.current_tab_id {
            Some(id) => id,
            None => return false,
        };

        // Get all tab IDs sorted
        let mut tab_ids: Vec<TabId> = self.tabs.keys().copied().collect();
        tab_ids.sort();

        // Find the current tab's index
        let current_idx = match tab_ids.iter().position(|&id| id == current_id) {
            Some(idx) => idx,
            None => return false,
        };

        // Get the next tab ID
        let next_idx = (current_idx + 1) % tab_ids.len();
        let next_id = tab_ids[next_idx];

        // Set the current tab
        self.current_tab_id = Some(next_id);
        true
    }

    /// Navigate to the previous tab
    pub fn prev_tab(&mut self) -> bool {
        if self.tabs.is_empty() {
            return false;
        }

        let current_id = match self.current_tab_id {
            Some(id) => id,
            None => return false,
        };

        // Get all tab IDs sorted
        let mut tab_ids: Vec<TabId> = self.tabs.keys().copied().collect();
        tab_ids.sort();

        // Find the current tab's index
        let current_idx = match tab_ids.iter().position(|&id| id == current_id) {
            Some(idx) => idx,
            None => return false,
        };

        // Get the previous tab ID
        let prev_idx = if current_idx == 0 {
            tab_ids.len() - 1
        } else {
            current_idx - 1
        };
        let prev_id = tab_ids[prev_idx];

        // Set the current tab
        self.current_tab_id = Some(prev_id);
        true
    }

    /// Get all tabs
    pub fn tabs(&self) -> &HashMap<TabId, Tab> {
        &self.tabs
    }

    /// Get the number of tabs
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    /// Check if there are any tabs
    pub fn has_tabs(&self) -> bool {
        !self.tabs.is_empty()
    }

    /// Get the tab IDs in order
    pub fn tab_ids(&self) -> Vec<TabId> {
        let mut ids: Vec<TabId> = self.tabs.keys().copied().collect();
        ids.sort();
        ids
    }

    /// Rename a tab
    pub fn rename_tab(&mut self, id: TabId, name: String) -> bool {
        if let Some(tab) = self.get_tab_mut(id) {
            tab.set_name(name);
            true
        } else {
            false
        }
    }

    /// Mark a tab as modified
    pub fn mark_tab_modified(&mut self, id: TabId, modified: bool) -> bool {
        if let Some(tab) = self.get_tab_mut(id) {
            tab.mark_modified(modified);
            true
        } else {
            false
        }
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tab() {
        let mut manager = TabManager::new();
        let id = manager.create_tab("Tab 1".to_string());

        assert_eq!(id, 1);
        assert_eq!(manager.tab_count(), 1);
        assert_eq!(manager.current_tab_id(), Some(1));

        let tab = manager.get_tab(id).unwrap();
        assert_eq!(tab.name, "Tab 1");
        assert_eq!(tab.id, 1);
        assert!(!tab.modified);
    }

    #[test]
    fn test_multiple_tabs() {
        let mut manager = TabManager::new();
        let id1 = manager.create_tab("Tab 1".to_string());
        let id2 = manager.create_tab("Tab 2".to_string());
        let id3 = manager.create_tab("Tab 3".to_string());

        assert_eq!(manager.tab_count(), 3);
        assert_eq!(manager.current_tab_id(), Some(1)); // First tab is current by default

        // Test tab navigation
        assert!(manager.next_tab());
        assert_eq!(manager.current_tab_id(), Some(2));

        assert!(manager.next_tab());
        assert_eq!(manager.current_tab_id(), Some(3));

        assert!(manager.next_tab());
        assert_eq!(manager.current_tab_id(), Some(1)); // Wrap around

        assert!(manager.prev_tab());
        assert_eq!(manager.current_tab_id(), Some(3)); // Wrap around backwards

        // Test tab closing
        assert!(manager.close_tab(id2));
        assert_eq!(manager.tab_count(), 2);
        assert_eq!(manager.current_tab_id(), Some(3)); // Current tab unchanged

        assert!(manager.close_tab(id3));
        assert_eq!(manager.tab_count(), 1);
        assert_eq!(manager.current_tab_id(), Some(1)); // Current tab changed to remaining tab

        assert!(manager.close_tab(id1));
        assert_eq!(manager.tab_count(), 0);
        assert_eq!(manager.current_tab_id(), None); // No tabs left
    }

    #[test]
    fn test_tab_modification() {
        let mut manager = TabManager::new();
        let id = manager.create_tab("Tab 1".to_string());

        // Test renaming
        assert!(manager.rename_tab(id, "New Name".to_string()));
        assert_eq!(manager.get_tab(id).unwrap().name, "New Name");

        // Test marking as modified
        assert!(!manager.get_tab(id).unwrap().modified);
        assert!(manager.mark_tab_modified(id, true));
        assert!(manager.get_tab(id).unwrap().modified);
    }
}