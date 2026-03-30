// Enhanced TextBuffer with undo/redo and line/column conversion utilities

use std::cmp::{max, min};

#[derive(Clone, Debug)]
pub struct EditOperation {
    pub start: usize,
    pub end: usize,
    pub inserted: String,
    pub deleted: String,
}

#[derive(Debug, Default)]
pub struct TextBuffer {
    pub content: String,
    undo_stack: Vec<EditOperation>,
    redo_stack: Vec<EditOperation>,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    /// Insert text at byte offset `idx`
    pub fn insert(&mut self, idx: usize, text: &str) {
        let idx = min(idx, self.content.len());
        let deleted = String::new();
        self.content.insert_str(idx, text);
        self.undo_stack.push(EditOperation {
            start: idx,
            end: idx + text.len(),
            inserted: text.to_string(),
            deleted,
        });
        self.redo_stack.clear();
    }

    /// Delete range [start, end) (byte offsets)
    pub fn delete_range(&mut self, start: usize, end: usize) {
        let start = min(start, self.content.len());
        let end = min(end, self.content.len());
        if start >= end {
            return;
        }
        let deleted = self.content[start..end].to_string();
        self.content.replace_range(start..end, "");
        self.undo_stack.push(EditOperation {
            start,
            end,
            inserted: String::new(),
            deleted,
        });
        self.redo_stack.clear();
    }

    /// Undo last operation
    pub fn undo(&mut self) {
        if let Some(op) = self.undo_stack.pop() {
            // reverse the operation
            if !op.inserted.is_empty() {
                // remove inserted text
                self.content.replace_range(op.start..op.start + op.inserted.len(), "");
            }
            if !op.deleted.is_empty() {
                // re‑insert deleted text
                self.content.insert_str(op.start, &op.deleted);
            }
            self.redo_stack.push(op);
        }
    }

    /// Redo last undone operation
    pub fn redo(&mut self) {
        if let Some(op) = self.redo_stack.pop() {
            if !op.deleted.is_empty() {
                self.content.replace_range(op.start..op.start + op.deleted.len(), "");
            }
            if !op.inserted.is_empty() {
                self.content.insert_str(op.start, &op.inserted);
            }
            self.undo_stack.push(op);
        }
    }

    /// Convert (line, column) → byte offset
    pub fn line_col_to_offset(&self, line: usize, column: usize) -> usize {
        let mut offset = 0usize;
        let mut current_line = 0usize;
        for (i, ch) in self.content.char_indices() {
            if current_line == line {
                // reached target line, now add column chars
                let col_offset = self.content[line..]
                    .char_indices()
                    .nth(column)
                    .map(|(idx, _)| idx)
                    .unwrap_or(0);
                return i + col_offset;
            }
            if ch == '\n' {
                current_line += 1;
                if current_line > line {
                    break;
                }
                offset = i + ch.len_utf8();
            }
        }
        offset
    }

    /// Convert byte offset → (line, column)
    pub fn offset_to_line_col(&self, offset: usize) -> (usize, usize) {
        let mut line = 0usize;
        let mut col = 0usize;
        let mut byte_count = 0usize;
        for ch in self.content.chars() {
            let ch_len = ch.len_utf8();
            if byte_count + ch_len > offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
            byte_count += ch_len;
        }
        (line, col)
    }
}
