use xvim::{
    buffer::Buffer,
    cursor::CursorPosition,
    editor::{Editor, EditorResult},
    visual::{VisualMode, VisualFunctions},
};

#[test]
fn test_visual_mode_start_end() -> EditorResult<()> {
    let mut editor = Editor::new()?;
    
    // Start visual mode
    editor.start_visual_mode(VisualMode::Char)?;
    
    // Check that visual mode is active
    assert!(editor.visual_state().active);
    assert_eq!(editor.visual_state().mode, VisualMode::Char);
    
    // End visual mode
    editor.end_visual_mode()?;
    
    // Check that visual mode is inactive
    assert!(!editor.visual_state().active);
    
    Ok(())
}

#[test]
fn test_visual_mode_toggle() -> EditorResult<()> {
    let mut editor = Editor::new()?;
    
    // Start visual mode
    editor.toggle_visual_mode(VisualMode::Char)?;
    
    // Check that visual mode is active
    assert!(editor.visual_state().active);
    assert_eq!(editor.visual_state().mode, VisualMode::Char);
    
    // Toggle to line mode
    editor.toggle_visual_mode(VisualMode::Line)?;
    
    // Check that visual mode is still active but in line mode
    assert!(editor.visual_state().active);
    assert_eq!(editor.visual_state().mode, VisualMode::Line);
    
    // Toggle line mode again to end it
    editor.toggle_visual_mode(VisualMode::Line)?;
    
    // Check that visual mode is inactive
    assert!(!editor.visual_state().active);
    
    Ok(())
}

#[test]
fn test_visual_mode_swap_corners() -> EditorResult<()> {
    let mut editor = Editor::new()?;
    
    // Create a buffer with some content
    let buffer_id = editor.buffer_manager.create_buffer()?;
    editor.buffer_manager.set_current_buffer(buffer_id)?;
    
    let buffer = editor.buffer_manager.get_buffer_mut(buffer_id)?;
    buffer.insert(0, "Hello, world!")?;
    
    // Start visual mode
    editor.start_visual_mode(VisualMode::Char)?;
    
    // Set cursor position
    let start_pos = CursorPosition::new(0, 0);
    editor.cursor_manager.set_position(start_pos);
    
    // Move cursor to create a selection
    let end_pos = CursorPosition::new(0, 5);
    editor.cursor_manager.set_position(end_pos);
    editor.selection_manager.update_selection(end_pos);
    
    // Save the current positions
    let original_cursor = editor.cursor_position();
    let original_start = editor.visual_state().start;
    
    // Swap corners
    editor.swap_visual_corners(false)?;
    
    // Check that the positions are swapped
    assert_eq!(editor.cursor_position(), original_start);
    assert_eq!(editor.visual_state().start, original_cursor);
    
    Ok(())
}

#[test]
fn test_visual_mode_reselect() -> EditorResult<()> {
    let mut editor = Editor::new()?;
    
    // Create a buffer with some content
    let buffer_id = editor.buffer_manager.create_buffer()?;
    editor.buffer_manager.set_current_buffer(buffer_id)?;
    
    let buffer = editor.buffer_manager.get_buffer_mut(buffer_id)?;
    buffer.insert(0, "Hello, world!\nSecond line\nThird line")?;
    
    // Start visual mode
    editor.start_visual_mode(VisualMode::Char)?;
    
    // Set cursor position
    let start_pos = CursorPosition::new(0, 0);
    editor.cursor_manager.set_position(start_pos);
    editor.visual_state_mut().start = start_pos;
    
    // Move cursor to create a selection
    let end_pos = CursorPosition::new(0, 5);
    editor.cursor_manager.set_position(end_pos);
    editor.selection_manager.update_selection(end_pos);
    
    // End visual mode (this should save the visual area)
    editor.end_visual_mode()?;
    
    // Move cursor to a different position
    let new_pos = CursorPosition::new(1, 0);
    editor.cursor_manager.set_position(new_pos);
    
    // Reselect the visual area
    editor.reselect_visual_area()?;
    
    // Check that the visual mode is active again with the same selection
    assert!(editor.visual_state().active);
    assert_eq!(editor.visual_state().start, start_pos);
    assert_eq!(editor.cursor_position(), end_pos);
    
    Ok(())
}