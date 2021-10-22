// string_utils_mod.rs

use crate::s;
use unwrap::unwrap;

/// returns string between the start end end delimiters without delimiters  
/// and the new cursor position  
pub fn get_delimited_text(source_str: &str, pos_cursor: usize, start_delimiter: &str, end_delimiter: &str) -> Option<(String, usize)> {
    if let Some(pos_start) = find_pos_after_delimiter(source_str, pos_cursor, start_delimiter) {
        if let Some(pos_end) = find_pos_before_delimiter(source_str, pos_start, end_delimiter) {
            let new_text = s!(unwrap!(source_str.get(pos_start..pos_end)));
            let new_pos_cursor = pos_end + end_delimiter.len();
            return Some((new_text, new_pos_cursor));
        }
    }
    // return
    None
}

/// find and return the range of the first occurrence including start and end delimiters  
/// Success: mutates also the cursor position, so the next find will continue from there  
/// Fail: return None if not found and don't mutate pos_cursor  
/// I use type Range to avoid references &str and lifetimes. But the programmer can make  
/// the error to apply the range to the wrong vector.  
pub fn find_range_including_delimiters(source_str: &str, pos_cursor: &mut usize, start_delimiter: &str, end_delimiter: &str) -> Option<std::ops::Range<usize>> {
    if let Some(pos_start) = find_pos_before_delimiter(source_str, *pos_cursor, start_delimiter) {
        // dbg!(&pos_start);
        if let Some(pos_end) = find_pos_after_delimiter(source_str, pos_start, end_delimiter) {
            // dbg!(&pos_end);
            *pos_cursor = pos_end;
            return Some(pos_start..pos_end);
        }
    }
    // return
    None
}

/// find and return the range of the first occurrence between start and end delimiters  
/// Success: mutates also the cursor position, so the next find will continue from there  
/// Fail: return None if not found and don't mutate pos_cursor  
/// I use type Range to avoid references &str and lifetimes. But the programmer can make  
/// the error to apply the range to the wrong vector.  
pub fn find_range_between_delimiters(source_str: &str, pos_cursor: &mut usize, start_delimiter: &str, end_delimiter: &str) -> Option<std::ops::Range<usize>> {
    if let Some(pos_start) = find_pos_after_delimiter(source_str, *pos_cursor, start_delimiter) {
        // dbg!(&pos_start);
        if let Some(pos_end) = find_pos_before_delimiter(source_str, pos_start, end_delimiter) {
            // dbg!(&pos_end);
            *pos_cursor = pos_end + end_delimiter.len();
            return Some(pos_start..pos_end);
        }
    }
    // return
    None
}

/// return the position after the delimiter or None  
/// Does NOT mutate the pos_cursor, because that is for a higher level logic to decide.  
pub fn find_pos_after_delimiter(source_str: &str, pos_cursor: usize, delimiter: &str) -> Option<usize> {
    if let Some(pos) = find_from(source_str, pos_cursor, delimiter) {
        let pos = pos + delimiter.len();
        return Some(pos);
    }
    // return
    None
}

/// return the position before the delimiter or None  
/// Does NOT mutate the pos_cursor, because that is for a higher level logic to decide.  
pub fn find_pos_before_delimiter(source_str: &str, pos_cursor: usize, delimiter: &str) -> Option<usize> {
    if let Some(pos) = find_from(source_str, pos_cursor, delimiter) {
        return Some(pos);
    }
    // return
    None
}

/// find str from pos_cursor low level  
/// panics if pos_cursor is incorrect: Check for bugs in calling functions.  
pub fn find_from(source_str: &str, pos_cursor: usize, find_str: &str) -> Option<usize> {
    let slice01 = unwrap!(source_str.get(pos_cursor..));
    let option_pos_found = slice01.find(find_str);
    if let Some(pos_found) = option_pos_found {
        // return Option with usize
        Some(pos_cursor + pos_found)
    } else {
        // return
        None
    }
}

/// returns a new String without the delimited text  
pub fn get_text_without_delimited_fragment(source_str: &str, pos_cursor: usize, start_delimiter: &str, end_delimiter: &str) -> String {
    if let Some(pos_start) = find_pos_before_delimiter(source_str, pos_cursor, start_delimiter) {
        if let Some(pos_end) = find_pos_after_delimiter(source_str, pos_start, end_delimiter) {
            let mut new_text = s!(source_str[..pos_start]);
            new_text.push_str(&source_str[pos_end..]);
            return new_text;
        }
    }
    // return
    s!(source_str)
}
