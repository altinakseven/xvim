//! Text objects module
//!
//! This module implements text objects for xvim, which are used to select
//! specific portions of text like words, sentences, paragraphs, and blocks.

use crate::buffer::Buffer;
use crate::buffer::BufferResult;

/// Types of text objects
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextObjectType {
    /// A word (alphanumeric characters)
    Word,
    /// A WORD (non-whitespace characters)
    BigWord,
    /// A sentence
    Sentence,
    /// A paragraph
    Paragraph,
    /// A block delimited by parentheses
    ParenBlock,
    /// A block delimited by braces
    BraceBlock,
    /// A block delimited by brackets
    BracketBlock,
    /// A block delimited by angle brackets
    AngleBlock,
    /// A block delimited by single quotes
    SingleQuoteBlock,
    /// A block delimited by double quotes
    DoubleQuoteBlock,
    /// A block delimited by backticks
    BacktickBlock,
    /// A tag block (HTML/XML)
    TagBlock,
}

/// A text object represents a range of text in a buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextObject {
    /// The type of text object
    pub object_type: TextObjectType,
    /// The start position (character index)
    pub start: usize,
    /// The end position (character index)
    pub end: usize,
    /// Whether to include the surrounding delimiters
    pub include_delimiters: bool,
}

impl TextObject {
    /// Create a new text object
    pub fn new(object_type: TextObjectType, start: usize, end: usize, include_delimiters: bool) -> Self {
        Self {
            object_type,
            start,
            end,
            include_delimiters,
        }
    }
    
    /// Get the length of the text object
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    
    /// Check if the text object is empty
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

/// Find a text object at the given position
pub fn find_text_object(
    buffer: &Buffer,
    position: usize,
    object_type: TextObjectType,
    include_delimiters: bool,
) -> BufferResult<Option<TextObject>> {
    match object_type {
        TextObjectType::Word => find_word(buffer, position, include_delimiters),
        TextObjectType::BigWord => find_big_word(buffer, position, include_delimiters),
        TextObjectType::Sentence => find_sentence(buffer, position, include_delimiters),
        TextObjectType::Paragraph => find_paragraph(buffer, position, include_delimiters),
        TextObjectType::ParenBlock => find_delimited_block(buffer, position, '(', ')', include_delimiters),
        TextObjectType::BraceBlock => find_delimited_block(buffer, position, '{', '}', include_delimiters),
        TextObjectType::BracketBlock => find_delimited_block(buffer, position, '[', ']', include_delimiters),
        TextObjectType::AngleBlock => find_delimited_block(buffer, position, '<', '>', include_delimiters),
        TextObjectType::SingleQuoteBlock => find_delimited_block(buffer, position, '\'', '\'', include_delimiters),
        TextObjectType::DoubleQuoteBlock => find_delimited_block(buffer, position, '"', '"', include_delimiters),
        TextObjectType::BacktickBlock => find_delimited_block(buffer, position, '`', '`', include_delimiters),
        TextObjectType::TagBlock => find_tag_block(buffer, position, include_delimiters),
    }
}

/// Find a word at the given position
fn find_word(buffer: &Buffer, position: usize, _include_delimiters: bool) -> BufferResult<Option<TextObject>> {
    let content = buffer.content();
    if position >= content.len() {
        return Ok(None);
    }
    
    // Find the start of the word
    let mut start = position;
    while start > 0 && is_word_char(content.chars().nth(start - 1).unwrap_or(' ')) {
        start -= 1;
    }
    
    // Find the end of the word
    let mut end = position;
    while end < content.len() && is_word_char(content.chars().nth(end).unwrap_or(' ')) {
        end += 1;
    }
    
    // If we didn't find a word, return None
    if start == end {
        return Ok(None);
    }
    
    Ok(Some(TextObject::new(TextObjectType::Word, start, end, false)))
}

/// Find a WORD at the given position
fn find_big_word(buffer: &Buffer, position: usize, _include_delimiters: bool) -> BufferResult<Option<TextObject>> {
    let content = buffer.content();
    if position >= content.len() {
        return Ok(None);
    }
    
    // Find the start of the WORD
    let mut start = position;
    while start > 0 && !is_whitespace(content.chars().nth(start - 1).unwrap_or(' ')) {
        start -= 1;
    }
    
    // Find the end of the WORD
    let mut end = position;
    while end < content.len() && !is_whitespace(content.chars().nth(end).unwrap_or(' ')) {
        end += 1;
    }
    
    // If we didn't find a WORD, return None
    if start == end {
        return Ok(None);
    }
    
    Ok(Some(TextObject::new(TextObjectType::BigWord, start, end, false)))
}

/// Find a sentence at the given position
fn find_sentence(buffer: &Buffer, position: usize, _include_delimiters: bool) -> BufferResult<Option<TextObject>> {
    let content = buffer.content();
    if position >= content.len() {
        return Ok(None);
    }
    
    // Find the start of the sentence
    let mut start = position;
    while start > 0 {
        let ch = content.chars().nth(start - 1).unwrap_or(' ');
        if ch == '.' || ch == '!' || ch == '?' {
            // Found the end of the previous sentence
            if start < content.len() {
                // Skip whitespace after the end of the previous sentence
                let mut pos = start;
                while pos < content.len() && is_whitespace(content.chars().nth(pos).unwrap_or(' ')) {
                    pos += 1;
                }
                start = pos;
            }
            break;
        } else if start == 1 {
            // Beginning of the content
            start = 0;
            break;
        }
        start -= 1;
    }
    
    // Find the end of the sentence
    let mut end = position;
    while end < content.len() {
        let ch = content.chars().nth(end).unwrap_or(' ');
        if ch == '.' || ch == '!' || ch == '?' {
            // Found the end of the sentence
            end += 1;
            break;
        }
        end += 1;
    }
    
    // If we didn't find a sentence, return None
    if start == end {
        return Ok(None);
    }
    
    Ok(Some(TextObject::new(TextObjectType::Sentence, start, end, false)))
}

/// Find a paragraph at the given position
fn find_paragraph(buffer: &Buffer, position: usize, _include_delimiters: bool) -> BufferResult<Option<TextObject>> {
    let content = buffer.content();
    if position >= content.len() {
        return Ok(None);
    }
    
    // Find the start of the paragraph
    let mut start = position;
    let mut prev_empty = false;
    while start > 0 {
        let line_start = find_line_start(&content, start);
        let line_end = find_line_end(&content, start);
        let line_empty = line_start == line_end || (line_start < line_end && is_whitespace(content.chars().nth(line_start).unwrap_or(' ')));
        
        if line_empty && !prev_empty {
            // Found an empty line before a non-empty line
            start = find_line_start(&content, line_end + 1);
            break;
        }
        
        if start == line_start {
            // Move to the previous line
            if line_start == 0 {
                // Beginning of the content
                break;
            }
            start = find_line_end(&content, line_start - 1);
        } else {
            start = line_start;
        }
        
        prev_empty = line_empty;
    }
    
    // Find the end of the paragraph
    let mut end = position;
    let mut prev_empty = false;
    while end < content.len() {
        let line_start = find_line_start(&content, end);
        let line_end = find_line_end(&content, end);
        let line_empty = line_start == line_end || (line_start < line_end && is_whitespace(content.chars().nth(line_start).unwrap_or(' ')));
        
        if line_empty && !prev_empty {
            // Found an empty line after a non-empty line
            end = line_start;
            break;
        }
        
        if end == line_end {
            // Move to the next line
            if line_end >= content.len() - 1 {
                // End of the content
                end = content.len();
                break;
            }
            end = find_line_start(&content, line_end + 1);
        } else {
            end = line_end;
        }
        
        prev_empty = line_empty;
    }
    
    // If we didn't find a paragraph, return None
    if start == end {
        return Ok(None);
    }
    
    Ok(Some(TextObject::new(TextObjectType::Paragraph, start, end, false)))
}

/// Find a delimited block at the given position
fn find_delimited_block(
    buffer: &Buffer,
    position: usize,
    open_delimiter: char,
    close_delimiter: char,
    include_delimiters: bool,
) -> BufferResult<Option<TextObject>> {
    let content = buffer.content();
    if position >= content.len() {
        return Ok(None);
    }
    
    // Find the opening delimiter
    let mut open_pos = position;
    let mut nesting = 0;
    while open_pos > 0 {
        let ch = content.chars().nth(open_pos).unwrap_or(' ');
        if ch == close_delimiter {
            nesting += 1;
        } else if ch == open_delimiter {
            if nesting == 0 {
                break;
            }
            nesting -= 1;
        }
        open_pos -= 1;
    }
    
    // If we didn't find an opening delimiter, return None
    if open_pos == 0 && content.chars().nth(0).unwrap_or(' ') != open_delimiter {
        return Ok(None);
    }
    
    // Find the closing delimiter
    let mut close_pos = position;
    let mut nesting = 0;
    while close_pos < content.len() {
        let ch = content.chars().nth(close_pos).unwrap_or(' ');
        if ch == open_delimiter {
            nesting += 1;
        } else if ch == close_delimiter {
            if nesting == 0 {
                break;
            }
            nesting -= 1;
        }
        close_pos += 1;
    }
    
    // If we didn't find a closing delimiter, return None
    if close_pos >= content.len() {
        return Ok(None);
    }
    
    // Adjust the range based on whether to include delimiters
    let start = if include_delimiters { open_pos } else { open_pos + 1 };
    let end = if include_delimiters { close_pos + 1 } else { close_pos };
    
    let object_type = match (open_delimiter, close_delimiter) {
        ('(', ')') => TextObjectType::ParenBlock,
        ('{', '}') => TextObjectType::BraceBlock,
        ('[', ']') => TextObjectType::BracketBlock,
        ('<', '>') => TextObjectType::AngleBlock,
        ('\'', '\'') => TextObjectType::SingleQuoteBlock,
        ('"', '"') => TextObjectType::DoubleQuoteBlock,
        ('`', '`') => TextObjectType::BacktickBlock,
        _ => return Ok(None),
    };
    
    Ok(Some(TextObject::new(object_type, start, end, include_delimiters)))
}

/// Find a tag block at the given position
fn find_tag_block(buffer: &Buffer, position: usize, include_delimiters: bool) -> BufferResult<Option<TextObject>> {
    let content = buffer.content();
    if position >= content.len() {
        return Ok(None);
    }
    
    // Find the opening tag
    let mut open_start = position;
    while open_start > 0 && content.chars().nth(open_start).unwrap_or(' ') != '<' {
        open_start -= 1;
    }
    
    // If we didn't find an opening tag, return None
    if open_start == 0 && content.chars().nth(0).unwrap_or(' ') != '<' {
        return Ok(None);
    }
    
    // Find the end of the opening tag
    let mut open_end = open_start;
    while open_end < content.len() && content.chars().nth(open_end).unwrap_or(' ') != '>' {
        open_end += 1;
    }
    
    // If we didn't find the end of the opening tag, return None
    if open_end >= content.len() {
        return Ok(None);
    }
    
    // Extract the tag name
    let mut tag_name = String::new();
    let mut i = open_start + 1;
    while i < open_end && !is_whitespace(content.chars().nth(i).unwrap_or(' ')) {
        tag_name.push(content.chars().nth(i).unwrap_or(' '));
        i += 1;
    }
    
    // Find the closing tag
    let mut close_start = open_end + 1;
    let closing_tag = format!("</{}>", tag_name);
    while close_start + closing_tag.len() <= content.len() {
        let mut found = true;
        for (i, ch) in closing_tag.chars().enumerate() {
            if content.chars().nth(close_start + i).unwrap_or(' ') != ch {
                found = false;
                break;
            }
        }
        if found {
            break;
        }
        close_start += 1;
    }
    
    // If we didn't find the closing tag, return None
    if close_start + closing_tag.len() > content.len() {
        return Ok(None);
    }
    
    // Adjust the range based on whether to include delimiters
    let start = if include_delimiters { open_start } else { open_end + 1 };
    let end = if include_delimiters { close_start + closing_tag.len() } else { close_start };
    
    Ok(Some(TextObject::new(TextObjectType::TagBlock, start, end, include_delimiters)))
}

/// Check if a character is a word character (alphanumeric or underscore)
fn is_word_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

/// Check if a character is whitespace
fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace()
}

/// Find the start of the line containing the given position
fn find_line_start(content: &str, position: usize) -> usize {
    let mut start = position;
    while start > 0 && content.chars().nth(start - 1).unwrap_or(' ') != '\n' {
        start -= 1;
    }
    start
}

/// Find the end of the line containing the given position
fn find_line_end(content: &str, position: usize) -> usize {
    let mut end = position;
    while end < content.len() && content.chars().nth(end).unwrap_or(' ') != '\n' {
        end += 1;
    }
    end
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Buffer;
    
    #[test]
    fn test_find_word() {
        let mut buffer = Buffer::new(1);
        buffer.insert(0, "Hello, world!").unwrap();
        
        // Find word at the beginning
        let word = find_word(&buffer, 0, false).unwrap().unwrap();
        assert_eq!(word.start, 0);
        assert_eq!(word.end, 5);
        assert_eq!(word.object_type, TextObjectType::Word);
        
        // Find word in the middle
        let word = find_word(&buffer, 7, false).unwrap().unwrap();
        assert_eq!(word.start, 7);
        assert_eq!(word.end, 12);
        assert_eq!(word.object_type, TextObjectType::Word);
        
        // Find word at a non-word character
        let word = find_word(&buffer, 5, false).unwrap();
        assert!(word.is_none());
    }
    
    #[test]
    fn test_find_big_word() {
        let mut buffer = Buffer::new(1);
        buffer.insert(0, "Hello, world!").unwrap();
        
        // Find WORD at the beginning
        let word = find_big_word(&buffer, 0, false).unwrap().unwrap();
        assert_eq!(word.start, 0);
        assert_eq!(word.end, 6);
        assert_eq!(word.object_type, TextObjectType::BigWord);
        
        // Find WORD in the middle
        let word = find_big_word(&buffer, 7, false).unwrap().unwrap();
        assert_eq!(word.start, 7);
        assert_eq!(word.end, 13);
        assert_eq!(word.object_type, TextObjectType::BigWord);
    }
    
    #[test]
    fn test_find_sentence() {
        let mut buffer = Buffer::new(1);
        buffer.insert(0, "Hello, world! This is a test. Another sentence.").unwrap();
        
        // Find sentence at the beginning
        let sentence = find_sentence(&buffer, 0, false).unwrap().unwrap();
        assert_eq!(sentence.start, 0);
        assert_eq!(sentence.end, 14);
        assert_eq!(sentence.object_type, TextObjectType::Sentence);
        
        // Find sentence in the middle
        let sentence = find_sentence(&buffer, 15, false).unwrap().unwrap();
        assert_eq!(sentence.start, 15);
        assert_eq!(sentence.end, 29);
        assert_eq!(sentence.object_type, TextObjectType::Sentence);
    }
    
    #[test]
    fn test_find_paragraph() {
        let mut buffer = Buffer::new(1);
        buffer.insert(0, "Paragraph 1.\n\nParagraph 2.\nStill paragraph 2.\n\nParagraph 3.").unwrap();
        
        // Find paragraph at the beginning
        let paragraph = find_paragraph(&buffer, 0, false).unwrap().unwrap();
        assert_eq!(paragraph.start, 0);
        assert_eq!(paragraph.end, 12);
        assert_eq!(paragraph.object_type, TextObjectType::Paragraph);
        
        // Find paragraph in the middle
        let paragraph = find_paragraph(&buffer, 15, false).unwrap().unwrap();
        assert_eq!(paragraph.start, 14);
        assert_eq!(paragraph.end, 46);
        assert_eq!(paragraph.object_type, TextObjectType::Paragraph);
    }
    
    #[test]
    fn test_find_delimited_block() {
        let mut buffer = Buffer::new(1);
        buffer.insert(0, "function(arg1, arg2) { return arg1 + arg2; }").unwrap();
        
        // Find parenthesis block
        let block = find_delimited_block(&buffer, 10, '(', ')', false).unwrap().unwrap();
        assert_eq!(block.start, 9);
        assert_eq!(block.end, 19);
        assert_eq!(block.object_type, TextObjectType::ParenBlock);
        
        // Find brace block
        let block = find_delimited_block(&buffer, 25, '{', '}', false).unwrap().unwrap();
        assert_eq!(block.start, 21);
        assert_eq!(block.end, 41);
        assert_eq!(block.object_type, TextObjectType::BraceBlock);
        
        // Find with include_delimiters
        let block = find_delimited_block(&buffer, 10, '(', ')', true).unwrap().unwrap();
        assert_eq!(block.start, 8);
        assert_eq!(block.end, 20);
        assert_eq!(block.object_type, TextObjectType::ParenBlock);
    }
    
    #[test]
    fn test_find_tag_block() {
        let mut buffer = Buffer::new(1);
        buffer.insert(0, "<div>This is a <span>test</span> of tag blocks</div>").unwrap();
        
        // Find span tag
        let block = find_tag_block(&buffer, 15, false).unwrap().unwrap();
        assert_eq!(block.start, 16);
        assert_eq!(block.end, 20);
        assert_eq!(block.object_type, TextObjectType::TagBlock);
        
        // Find div tag
        let block = find_tag_block(&buffer, 5, false).unwrap().unwrap();
        assert_eq!(block.start, 5);
        assert_eq!(block.end, 45);
        assert_eq!(block.object_type, TextObjectType::TagBlock);
        
        // Find with include_delimiters
        let block = find_tag_block(&buffer, 15, true).unwrap().unwrap();
        assert_eq!(block.start, 10);
        assert_eq!(block.end, 27);
        assert_eq!(block.object_type, TextObjectType::TagBlock);
    }
}