use anyhow::Result;
use std::path::Path;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Parser, Query, QueryCursor};

/// Extracted AST symbol (function, struct, impl) from source code
#[derive(Debug, Clone)]
pub struct AstSymbol {
    pub name: String,
    pub symbol_type: String,
    pub source_path: String,
    pub line_number: usize,
}

/// Parses a Rust file and extracts functions, structs, and impl blocks.
pub fn extract_rust_symbols(file_path: &Path, content: &str) -> Result<Vec<AstSymbol>> {
    let mut parser = Parser::new();
    let language = tree_sitter_rust::LANGUAGE.into();
    parser
        .set_language(&language)
        .map_err(|e| anyhow::anyhow!("Error loading Rust grammar: {}", e))?;

    let tree = parser
        .parse(content, None)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse file"))?;

    // Query to extract struct names, function names, and trait/impl names
    let query_src = r#"
        (struct_item name: (type_identifier) @struct)
        (function_item name: (identifier) @function)
        (impl_item type: (type_identifier) @impl)
    "#;

    let query = Query::new(&language, query_src)
        .map_err(|e| anyhow::anyhow!("Invalid tree-sitter query: {:?}", e))?;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

    let mut symbols = Vec::new();
    let source_path = file_path.to_string_lossy().to_string();

    while let Some(m) = matches.next() {
        for cap in m.captures {
            let node = cap.node;
            if let Ok(name_str) = node.utf8_text(content.as_bytes()) {
                let symbol_type = match query.capture_names()[cap.index as usize] {
                    "struct" => "Struct",
                    "function" => "Function",
                    "impl" => "Impl",
                    _ => "Unknown",
                };

                symbols.push(AstSymbol {
                    name: name_str.to_string(),
                    symbol_type: symbol_type.to_string(),
                    source_path: source_path.clone(),
                    line_number: node.start_position().row + 1, // 1-indexed
                });
            }
        }
    }

    Ok(symbols)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn verifies_ast_extraction() {
        let code = "struct Foo; fn bar() {} impl Foo {}";
        let symbols = extract_rust_symbols(&PathBuf::from("test.rs"), code).unwrap();
        assert_eq!(symbols.len(), 3);
        assert_eq!(symbols[0].name, "Foo");
        assert_eq!(symbols[1].name, "bar");
        assert_eq!(symbols[2].name, "Foo");
    }
}
