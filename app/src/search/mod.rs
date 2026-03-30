// Search module – placeholder for find/replace functionality

pub fn find(_text: &str, _query: &str) -> Vec<(usize, usize)> {
    // Returns start/end byte offsets of matches
    Vec::new()
}

pub fn replace(_text: &mut String, _query: &str, _replacement: &str) {
    // Simple replace all occurrences – to be improved with regex support
    *_text = _text.replace(_query, _replacement);
}
