// This file contains tab management methods for the TerminalUi class
// These will be added to the TerminalUi implementation in mod.rs

impl TerminalUi {
    /// Initialize the tab manager with the first tab
    pub fn init_tabs(&mut self, buffer_id: usize) -> UiResult<TabId> {
        // Create a new tab
        let tab_id = self.tab_manager.create_tab(format!("Buffer {}", buffer_id));
        
        // Get the tab
        let tab = self.tab_manager.get_tab_mut(tab_id).unwrap();
        
        // Create a window in the tab
        let (width, height) = self.size;
        let window_id = tab.window_manager.create_initial_window(buffer_id, width, height);
        
        Ok(tab_id)
    }
    
    /// Create a new tab
    pub fn create_tab(&mut self, buffer_id: usize, name: Option<String>) -> UiResult<TabId> {
        // Create a new tab
        let tab_name = name.unwrap_or_else(|| format!("Buffer {}", buffer_id));
        let tab_id = self.tab_manager.create_tab(tab_name);
        
        // Get the tab
        let tab = self.tab_manager.get_tab_mut(tab_id).unwrap();
        
        // Create a window in the tab
        let (width, height) = self.size;
        let window_id = tab.window_manager.create_initial_window(buffer_id, width, height);
        
        // Make this the current tab
        self.tab_manager.set_current_tab(tab_id);
        
        Ok(tab_id)
    }
    
    /// Close the current tab
    pub fn close_current_tab(&mut self) -> UiResult<bool> {
        if let Some(tab_id) = self.tab_manager.current_tab_id() {
            Ok(self.tab_manager.close_tab(tab_id))
        } else {
            Ok(false)
        }
    }
    
    /// Close a tab by ID
    pub fn close_tab(&mut self, tab_id: TabId) -> UiResult<bool> {
        Ok(self.tab_manager.close_tab(tab_id))
    }
    
    /// Navigate to the next tab
    pub fn next_tab(&mut self) -> UiResult<bool> {
        Ok(self.tab_manager.next_tab())
    }
    
    /// Navigate to the previous tab
    pub fn prev_tab(&mut self) -> UiResult<bool> {
        Ok(self.tab_manager.prev_tab())
    }
    
    /// Get the current tab ID
    pub fn current_tab_id(&self) -> Option<TabId> {
        self.tab_manager.current_tab_id()
    }
    
    /// Set the current tab
    pub fn set_current_tab(&mut self, tab_id: TabId) -> UiResult<bool> {
        Ok(self.tab_manager.set_current_tab(tab_id))
    }
    
    /// Get a reference to the current tab
    pub fn current_tab(&self) -> Option<&Tab> {
        self.tab_manager.current_tab()
    }
    
    /// Get a mutable reference to the current tab
    pub fn current_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tab_manager.current_tab_mut()
    }
    
    /// Get the current window ID
    pub fn current_window_id(&self) -> Option<usize> {
        self.tab_manager.current_tab().and_then(|tab| tab.current_window_id())
    }
    
    /// Get a reference to the current window
    pub fn current_window(&self) -> Option<&Window> {
        self.tab_manager.current_tab().and_then(|tab| tab.current_window())
    }
    
    /// Get a mutable reference to the current window
    pub fn current_window_mut(&mut self) -> Option<&mut Window> {
        self.tab_manager.current_tab_mut().and_then(|tab| tab.current_window_mut())
    }
    
    /// Split the current window
    pub fn split_window(&mut self, direction: SplitDirection, buffer_id: usize) -> UiResult<Option<usize>> {
        if let Some(tab) = self.tab_manager.current_tab_mut() {
            Ok(tab.window_manager.split_current_window(direction, buffer_id))
        } else {
            Ok(None)
        }
    }
    
    /// Navigate to the next window in the current tab
    pub fn next_window(&mut self) -> UiResult<bool> {
        if let Some(tab) = self.tab_manager.current_tab_mut() {
            Ok(tab.window_manager.next_window())
        } else {
            Ok(false)
        }
    }
    
    /// Navigate to the previous window in the current tab
    pub fn prev_window(&mut self) -> UiResult<bool> {
        if let Some(tab) = self.tab_manager.current_tab_mut() {
            Ok(tab.window_manager.prev_window())
        } else {
            Ok(false)
        }
    }
    
    /// Close the current window in the current tab
    pub fn close_current_window(&mut self) -> UiResult<bool> {
        if let Some(tab) = self.tab_manager.current_tab_mut() {
            let current_id = tab.window_manager.current_window_id();
            Ok(tab.window_manager.close_window(current_id))
        } else {
            Ok(false)
        }
    }
    
    /// Rename the current tab
    pub fn rename_current_tab(&mut self, name: String) -> UiResult<bool> {
        if let Some(tab_id) = self.tab_manager.current_tab_id() {
            Ok(self.tab_manager.rename_tab(tab_id, name))
        } else {
            Ok(false)
        }
    }
    
    /// Mark the current tab as modified
    pub fn mark_current_tab_modified(&mut self, modified: bool) -> UiResult<bool> {
        if let Some(tab_id) = self.tab_manager.current_tab_id() {
            Ok(self.tab_manager.mark_tab_modified(tab_id, modified))
        } else {
            Ok(false)
        }
    }
    
    /// Update the tab manager when the terminal size changes
    pub fn update_tabs(&mut self, buffers: &[&Buffer]) -> UiResult<()> {
        let (width, height) = self.size;
        
        // Get all tab IDs first
        let tab_ids: Vec<_> = self.tab_manager.tab_ids().clone();
        
        // Update each tab's windows
        for &tab_id in &tab_ids {
            // Get the tab mutably
            if let Some(tab) = self.tab_manager.get_tab_mut(tab_id) {
                // Get the buffers for this tab's windows
                let tab_buffers: Vec<&Buffer> = tab.window_manager.windows()
                    .iter()
                    .filter_map(|window| {
                        buffers.iter().find(|buffer| buffer.id() == window.buffer_id)
                    })
                    .copied()
                    .collect();
                
                // Resize the windows
                if let Err(e) = tab.window_manager.resize_all(width, height, &tab_buffers) {
                    return Err(UiError::Other(format!("Failed to resize windows: {}", e)));
                }
            }
        }
        
        Ok(())
    }
    
    /// Render the current tab
    pub fn render_current_tab(&self, buffers: &[&Buffer], mode: Mode, command_buffer: &str) -> UiResult<()> {
        // Clear the screen
        execute!(
            io::stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        
        // Render the tab bar
        self.render_tab_bar()?;
        
        // Render the current tab's windows
        if let Some(tab) = self.tab_manager.current_tab() {
            // Get the buffers for this tab's windows
            let tab_buffers: Vec<&Buffer> = tab.window_manager.windows()
                .iter()
                .filter_map(|window| {
                    buffers.iter().find(|buffer| buffer.id() == window.buffer_id)
                })
                .copied()
                .collect();
            
            // Render the windows
            self.render_windows(&tab.window_manager, &tab_buffers, mode)?;
        }
        
        // Render command line if in command mode
        if mode == Mode::Command {
            let (width, height) = self.size;
            let mut stdout = io::stdout();
            
            // Move to the command line position
            execute!(
                stdout,
                cursor::MoveTo(0, height - 1)
            )?;
            
            // Clear the line
            execute!(
                stdout,
                terminal::Clear(ClearType::CurrentLine)
            )?;
            
            // Write the command prompt and buffer
            write!(stdout, ":{}", command_buffer)?;
            
            // Position the cursor after the command text
            execute!(
                stdout,
                cursor::MoveTo(1 + command_buffer.len() as u16, height - 1)
            )?;
        }
        
        // Flush stdout
        io::stdout().flush()?;
        
        Ok(())
    }
    
    /// Render the tab bar
    fn render_tab_bar(&self) -> UiResult<()> {
        let mut stdout = io::stdout();
        let (width, _) = self.size;
        
        // Set the tab bar color
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            style::SetBackgroundColor(Color::DarkBlue),
            style::SetForegroundColor(Color::White)
        )?;
        
        // Get all tab IDs in order
        let tab_ids = self.tab_manager.tab_ids();
        let current_tab_id = self.tab_manager.current_tab_id();
        
        // Calculate the available width for tabs
        let available_width = width as usize;
        let tab_count = tab_ids.len();
        
        if tab_count == 0 {
            // No tabs, just render an empty bar
            write!(stdout, "{}", " ".repeat(available_width))?;
        } else {
            // Calculate the width for each tab
            let tab_width = available_width / tab_count;
            
            // Render each tab
            for (i, &tab_id) in tab_ids.iter().enumerate() {
                let tab = self.tab_manager.get_tab(tab_id).unwrap();
                let is_current = Some(tab_id) == current_tab_id;
                
                // Set the tab color
                if is_current {
                    execute!(
                        stdout,
                        style::SetBackgroundColor(Color::Blue),
                        style::SetForegroundColor(Color::White),
                        style::SetAttribute(style::Attribute::Bold)
                    )?;
                } else {
                    execute!(
                        stdout,
                        style::SetBackgroundColor(Color::DarkBlue),
                        style::SetForegroundColor(Color::White)
                    )?;
                }
                
                // Calculate the tab position
                let tab_x = i * tab_width;
                execute!(stdout, cursor::MoveTo(tab_x as u16, 0))?;
                
                // Render the tab name
                let mut tab_name = tab.name.clone();
                if tab.modified {
                    tab_name.push('+');
                }
                
                // Truncate the tab name if it's too long
                if tab_name.len() > tab_width - 2 {
                    tab_name = format!("{}…", &tab_name[0..tab_width - 3]);
                }
                
                // Pad the tab name
                let padding = " ".repeat(tab_width - tab_name.len());
                write!(stdout, " {}{}", tab_name, padding)?;
            }
        }
        
        // Reset the color
        execute!(stdout, style::ResetColor)?;
        
        Ok(())
    }
    
    /// Render the windows in a tab
    fn render_windows(&self, window_manager: &WindowManager, buffers: &[&Buffer], mode: Mode) -> UiResult<()> {
        let mut stdout = io::stdout();
        
        // Get all windows
        let windows = window_manager.windows();
        
        // Render each window
        for window in windows {
            // Find the buffer for this window
            let buffer_idx = buffers.iter().position(|b| b.id() == window.buffer_id);
            
            if let Some(idx) = buffer_idx {
                let buffer = buffers[idx];
                
                // Render the window
                self.render_window(window, buffer, mode)?;
            }
        }
        
        Ok(())
    }
    
    /// Render a single window
    fn render_window(&self, window: &Window, buffer: &Buffer, mode: Mode) -> UiResult<()> {
        let mut stdout = io::stdout();
        
        // Get the window content area
        let content_area = window.rect.content_area();
        
        // Render the window border
        self.render_window_border(window)?;
        
        // Render the buffer content
        for i in 0..content_area.height {
            let line_idx = window.top_line + i as usize;
            
            if line_idx < buffer.line_count() {
                // Position the cursor at the start of the line
                execute!(stdout, cursor::MoveTo(content_area.x, content_area.y + i))?;
                
                // Render the line number
                write!(stdout, "{:4} ", line_idx + 1)?;
                
                // Render the line content with syntax highlighting
                self.syntax_renderer.render_line(
                    &mut stdout,
                    buffer,
                    line_idx,
                    window.left_col,
                    content_area.width as usize - 5, // Subtract 5 for line number and space
                )?;
            } else {
                // Position the cursor at the start of the line
                execute!(stdout, cursor::MoveTo(content_area.x, content_area.y + i))?;
                
                // Render a tilde for empty lines
                write!(stdout, "~")?;
            }
        }
        
        // Render the status line
        self.render_window_status_line(window, buffer, mode)?;
        
        // Highlight the cursor position
        let cursor_x = content_area.x + 5 + (window.cursor.column - window.left_col) as u16;
        let cursor_y = content_area.y + (window.cursor.line - window.top_line) as u16;
        
        if cursor_x < content_area.x + content_area.width && cursor_y < content_area.y + content_area.height {
            // Move to the cursor position
            execute!(stdout, cursor::MoveTo(cursor_x, cursor_y))?;
            
            // Get the character at the cursor position
            let cursor_char = if window.cursor.line < buffer.line_count() {
                let line = buffer.line(window.cursor.line).unwrap_or_default();
                if window.cursor.column < line.len() {
                    line.chars().nth(window.cursor.column).unwrap_or(' ')
                } else {
                    ' ' // Cursor is at the end of the line or beyond
                }
            } else {
                ' ' // Cursor is beyond the end of the buffer
            };
            
            // Highlight the cursor by inverting the colors
            execute!(
                stdout,
                style::SetBackgroundColor(Color::White),
                style::SetForegroundColor(Color::Black)
            )?;
            
            // Write the character at the cursor position
            write!(stdout, "{}", cursor_char)?;
            
            // Reset the colors
            execute!(stdout, style::ResetColor)?;
            
            // Move back to the cursor position
            execute!(stdout, cursor::MoveTo(cursor_x, cursor_y))?;
        }
        
        Ok(())
    }
    
    /// Render the window border
    fn render_window_border(&self, window: &Window) -> UiResult<()> {
        let mut stdout = io::stdout();
        
        // Only render borders if there are multiple windows
        if self.tab_manager.current_tab().map_or(false, |tab| tab.window_manager.windows().len() <= 1) {
            return Ok(());
        }
        
        // Set the border color
        let is_current = self.tab_manager.current_tab().map_or(false, |tab| {
            window.id == tab.window_manager.current_window_id()
        });
        
        let border_color = if is_current {
            Color::Blue
        } else {
            Color::DarkGrey
        };
        
        execute!(stdout, style::SetForegroundColor(border_color))?;
        
        // Render the top border
        execute!(stdout, cursor::MoveTo(window.rect.x, window.rect.y))?;
        write!(stdout, "┌")?;
        for _ in 1..window.rect.width - 1 {
            write!(stdout, "─")?;
        }
        write!(stdout, "┐")?;
        
        // Render the side borders
        for i in 1..window.rect.height - 1 {
            execute!(stdout, cursor::MoveTo(window.rect.x, window.rect.y + i))?;
            write!(stdout, "│")?;
            
            execute!(stdout, cursor::MoveTo(window.rect.x + window.rect.width - 1, window.rect.y + i))?;
            write!(stdout, "│")?;
        }
        
        // Render the bottom border
        execute!(stdout, cursor::MoveTo(window.rect.x, window.rect.y + window.rect.height - 1))?;
        write!(stdout, "└")?;
        for _ in 1..window.rect.width - 1 {
            write!(stdout, "─")?;
        }
        write!(stdout, "┘")?;
        
        // Reset the color
        execute!(stdout, style::ResetColor)?;
        
        Ok(())
    }
    
    /// Render the window status line
    fn render_window_status_line(&self, window: &Window, buffer: &Buffer, mode: Mode) -> UiResult<()> {
        let mut stdout = io::stdout();
        
        // Use the enhanced status line renderer
        self.status_line.render_window_status_line(
            &mut stdout,
            &window.rect,
            buffer,
            mode,
            window.cursor.line,
            window.cursor.column,
        )
    }
}