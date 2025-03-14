use xvim::buffer::{Buffer, BufferManager};

#[test]
fn test_buffer_creation() {
    let buffer = Buffer::new(1);
    assert_eq!(buffer.id(), 1);
    assert_eq!(buffer.name(), "[No Name]");
    assert_eq!(buffer.line_count(), 1);
    assert!(!buffer.is_modified());
}

#[test]
fn test_buffer_insert_delete() {
    let mut buffer = Buffer::new(1);
    
    // Insert text
    buffer.insert(0, "Hello, world!").unwrap();
    assert_eq!(buffer.content(), "Hello, world!");
    assert!(buffer.is_modified());
    
    // Delete text
    buffer.delete(0, 7).unwrap();
    assert_eq!(buffer.content(), "world!");
}

#[test]
fn test_buffer_manager() {
    let mut manager = BufferManager::new();
    
    // Create a buffer
    let id1 = manager.create_buffer().unwrap();
    assert_eq!(id1, 1);
    
    // Create another buffer
    let id2 = manager.create_buffer().unwrap();
    assert_eq!(id2, 2);
    
    // Get buffer by ID
    let buffer = manager.get_buffer(id1).unwrap();
    assert_eq!(buffer.id(), id1);
    
    // Set current buffer
    manager.set_current_buffer(id2).unwrap();
    assert_eq!(manager.current_buffer_id(), Some(id2));
    
    // Close a buffer
    manager.close_buffer(id1).unwrap();
    assert!(manager.get_buffer(id1).is_err());
    assert_eq!(manager.buffer_count(), 1);
}