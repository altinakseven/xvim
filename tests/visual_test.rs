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
    let buffer_id = editor.get_buffer_manager_mut().create_buffer()?;
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id)?;
    
    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
    buffer.insert(0, "Hello, world!")?;
    
    // Start visual mode
    editor.start_visual_mode(VisualMode::Char)?;
    
    // Set cursor position
    let start_pos = CursorPosition::new(0, 0);
    editor.get_cursor_manager_mut().set_position(start_pos);
    
    // Start visual mode at the current position
    editor.start_visual_mode(VisualMode::Char)?;
    
    // Move cursor to create a selection
    let end_pos = CursorPosition::new(0, 5);
    editor.get_cursor_manager_mut().set_position(end_pos);
    
    // In a real editor, moving the cursor in visual mode would automatically update the selection
    // For the test, we need to simulate this by moving the cursor
    // The selection is updated in the process_key_legacy method when in visual mode
    // We'll use the swap_visual_corners method to force an update of the selection
    editor.swap_visual_corners(false)?;
    editor.swap_visual_corners(false)?;
    
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
    let buffer_id = editor.get_buffer_manager_mut().create_buffer()?;
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id)?;
    
    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
    buffer.insert(0, "Hello, world!\nSecond line\nThird line")?;
    
    // Start visual mode
    editor.start_visual_mode(VisualMode::Char)?;
    
    // Set cursor position
    let start_pos = CursorPosition::new(0, 0);
    editor.get_cursor_manager_mut().set_position(start_pos);
    // We need to set the visual state start position
    // This is a bit tricky since there's no direct setter method
    // We'll use start_visual_mode which sets the start position to the current cursor position
    editor.start_visual_mode(VisualMode::Char)?;
    
    // Move cursor to create a selection
    let end_pos = CursorPosition::new(0, 5);
    editor.get_cursor_manager_mut().set_position(end_pos);
    // In a real editor, moving the cursor in visual mode would automatically update the selection
    // For the test, we need to simulate this by moving the cursor
    // The selection is updated in the process_key_legacy method when in visual mode
    // We'll use the swap_visual_corners method to force an update of the selection
    editor.swap_visual_corners(false)?;
    editor.swap_visual_corners(false)?;
    
    // End visual mode (this should save the visual area)
    editor.end_visual_mode()?;
    
    // Move cursor to a different position
    let new_pos = CursorPosition::new(1, 0);
    editor.get_cursor_manager_mut().set_position(new_pos);
    
    // Reselect the visual area
    editor.reselect_visual_area()?;
    
    // Check that the visual mode is active again with the same selection
    assert!(editor.visual_state().active);
    assert_eq!(editor.visual_state().start, start_pos);
    assert_eq!(editor.cursor_position(), end_pos);
    
    Ok(())
}