use tree_sitter::{Language, Parser, Tree, Query, QueryCursor};

pub struct Highlighter {
    parser: Parser,
    language: Language,
}

impl Highlighter {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        let language = tree_sitter_rust::LANGUAGE.into(); // or tree_sitter_rust::language()
        // tree_sitter_rust v0.20 uses tree_sitter_rust::language()
        let language = tree_sitter_rust::language();
        parser.set_language(language).expect("Failed to load Rust grammar");
        Self { parser, language }
    }

    /// Parse the source text and return a Tree‑sitter `Tree`
    pub fn parse(&mut self, source: &str) -> Option<Tree> {
        self.parser.parse(source, None)
    }

    /// Extracted highlight logic using queries
    pub fn highlight(&mut self, source: &str) -> Vec<(usize, usize, String)> {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return Vec::new(),
        };

        // Simplified highlight query for Rust
        let query_string = r#"
            (string_literal) @string
            (line_comment) @comment
            (block_comment) @comment
            (identifier) @variable
            (function_item name: (identifier) @function)
            "pub" @keyword "fn" @keyword "let" @keyword "mut" @keyword
            "struct" @keyword "enum" @keyword "use" @keyword "impl" @keyword
            "match" @keyword "if" @keyword "else" @keyword "return" @keyword
        "#;

        let query = match Query::new(self.language, query_string) {
            Ok(q) => q,
            Err(_) => return Vec::new(), // Silently skip if query fails to parse
        };

        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&query, tree.root_node(), source.as_bytes());

        let mut highlights = Vec::new();

        for m in matches {
            for capture in m.captures {
                let capture_name = query.capture_names()[capture.index as usize].as_str();
                let start = capture.node.start_byte();
                let end = capture.node.end_byte();
                let style = match capture_name {
                    "string" => "String",
                    "comment" => "Comment",
                    "function" => "Function",
                    "keyword" => "Keyword",
                    "variable" => "Variable",
                    _ => "Normal",
                };
                highlights.push((start, end, style.to_string()));
            }
        }
        
        // Sort highlights by starting offset
        highlights.sort_by_key(|h| h.0);
        highlights
    }
}
