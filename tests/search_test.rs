use xvim::buffer::Buffer;
use xvim::search::{SearchState, SearchDirection};

#[test]
fn test_search_functionality() {
    // Create a buffer with some content
    let mut buffer = Buffer::new(1);
    buffer.insert(0, "Line 1: This is a test\nLine 2: Another test line\nLine 3: Final test line").unwrap();
    
    // Test search
    let results = buffer.search("test", true).unwrap();
    assert_eq!(results.len(), 3);
    assert_eq!(results[0], (0, 18, "test".to_string()));
    assert_eq!(results[1], (1, 16, "test".to_string()));
    assert_eq!(results[2], (2, 14, "test".to_string()));
    
    // Test case-insensitive search
    let results = buffer.search("TEST", false).unwrap();
    assert_eq!(results.len(), 3);
    
    // Test find_next
    let result = buffer.find_next("test", 0, 0, true).unwrap();
    assert!(result.is_some());
    let (line, col, text) = result.unwrap();
    assert_eq!(line, 0);
    assert_eq!(col, 18);
    assert_eq!(text, "test");
    
    // Test find_next from middle of buffer
    let result = buffer.find_next("test", 1, 0, true).unwrap();
    assert!(result.is_some());
    let (line, col, text) = result.unwrap();
    assert_eq!(line, 1);
    assert_eq!(col, 16);
    assert_eq!(text, "test");
    
    // Test find_prev
    let result = buffer.find_prev("test", 2, 0, true).unwrap();
    assert!(result.is_some());
    let (line, col, text) = result.unwrap();
    assert_eq!(line, 1);
    assert_eq!(col, 16);
    assert_eq!(text, "test");
}

#[test]
fn test_search_state() {
    let mut state = SearchState::new();
    
    // Test setting pattern
    state.set_pattern("test".to_string());
    assert_eq!(state.pattern(), Some("test"));
    
    // Test setting direction
    state.set_direction(SearchDirection::Backward);
    assert_eq!(state.direction(), SearchDirection::Backward);
    
    // Test setting case sensitivity
    state.set_case_sensitive(true);
    assert_eq!(state.case_sensitive(), true);
    
    // Test search history
    state.set_pattern("pattern1".to_string());
    state.set_pattern("pattern2".to_string());
    state.set_pattern("pattern3".to_string());
    
    // Test history navigation
    assert_eq!(state.history_next(), Some("pattern3"));
    assert_eq!(state.history_next(), Some("pattern2"));
    assert_eq!(state.history_next(), Some("pattern1"));
    assert_eq!(state.history_next(), Some("test")); // Wrap around
    
    assert_eq!(state.history_prev(), Some("pattern1"));
    assert_eq!(state.history_prev(), Some("pattern2"));
    assert_eq!(state.history_prev(), Some("pattern3")); // Wrap around
    
    // Test search results
    let results = vec![
        (0, 5, "test1".to_string()),
        (1, 10, "test2".to_string()),
        (2, 15, "test3".to_string()),
    ];
    
    state.set_results(results);
    assert_eq!(state.results().len(), 3);
    
    // Test result navigation
    let result = state.next_result();
    assert!(result.is_some());
    let (line, col, text) = result.unwrap();
    assert_eq!(line, 1);
    assert_eq!(col, 10);
    assert_eq!(text, "test2");
    
    let result = state.next_result();
    assert!(result.is_some());
    let (line, col, text) = result.unwrap();
    assert_eq!(line, 2);
    assert_eq!(col, 15);
    assert_eq!(text, "test3");
    
    let result = state.next_result();
    assert!(result.is_some());
    let (line, col, text) = result.unwrap();
    assert_eq!(line, 0);
    assert_eq!(col, 5);
    assert_eq!(text, "test1");
    
    let result = state.prev_result();
    assert!(result.is_some());
    let (line, col, text) = result.unwrap();
    assert_eq!(line, 2);
    assert_eq!(col, 15);
    assert_eq!(text, "test3");
}