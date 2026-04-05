#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

//! # llms-txt-parser
//!
//! Parser for llms.txt files following the llms.txt specification.
//!
//! ## Usage
//!
//! ```rust
//! use llms_txt_parser::parse_content;
//!
//! let content = r#"# My Project
//! > A great project
//! ## Getting Started
//! - [Intro](./intro.md)
//! "#;
//!
//! let llms_txt = parse_content(content)?;
//! println!("Project: {}", llms_txt.project_name);
//! println!("Sections: {}", llms_txt.sections.len());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod parse;
mod types;

pub use parse::{parse_content, parse_file};
pub use types::{Frontmatter, Link, LlmsTxt, Section};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() -> anyhow::Result<()> {
        let content = r"# My Project

> A great project

## Getting Started

- [Introduction](./intro.md): Getting started guide

## Core Concepts

- [Concepts](./concepts.md)
";

        let llms_txt = parse_content(content)?;
        assert_eq!(llms_txt.project_name, "My Project");
        assert_eq!(llms_txt.description, Some("A great project".to_string()));
        assert_eq!(llms_txt.sections.len(), 2);
        assert_eq!(llms_txt.sections[0].title, "Getting Started");
        assert_eq!(llms_txt.sections[0].links.len(), 1);
        Ok(())
    }

    #[test]
    fn test_parse_with_frontmatter() -> anyhow::Result<()> {
        let content = r"---
version: 1.0
project: Test Project
documents: 42
---

# Test Project

> Description

## Getting Started
";

        let llms_txt = parse_content(content)?;
        assert!(llms_txt.frontmatter.is_some());
        let fm = llms_txt
            .frontmatter
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("frontmatter is None"))?;
        assert_eq!(fm.version, Some("1.0".to_string()));
        assert_eq!(fm.project, Some("Test Project".to_string()));
        assert_eq!(fm.documents, Some(42));
        Ok(())
    }

    #[test]
    fn test_parse_link() -> anyhow::Result<()> {
        let link = parse::parse_link("[Title](./path.md): Description")
            .ok_or_else(|| anyhow::anyhow!("parse_link returned None"))?;
        assert_eq!(link.text, "Title");
        assert_eq!(link.url, "./path.md");
        assert_eq!(link.description, Some("Description".to_string()));
        Ok(())
    }

    #[test]
    fn test_get_section() -> anyhow::Result<()> {
        let content = r"# Project

## Getting Started

Content here
";

        let llms_txt = parse_content(content)?;
        assert!(llms_txt.get_section("Getting Started").is_some());
        assert!(llms_txt.get_section("Missing").is_none());
        Ok(())
    }

    #[test]
    fn test_required_sections() -> anyhow::Result<()> {
        let content = r"# Project

## Getting Started
## Core Concepts
## API Reference
";

        let llms_txt = parse_content(content)?;
        assert!(llms_txt.has_required_sections());
        Ok(())
    }

    #[test]
    fn test_parse_link_inline_markdown() -> anyhow::Result<()> {
        let link = parse::parse_link("[Title](./path.md): Description with `code` and **bold**")
            .ok_or_else(|| anyhow::anyhow!("parse_link returned None"))?;
        assert_eq!(link.text, "Title");
        assert_eq!(link.url, "./path.md");
        assert_eq!(
            link.description,
            Some("Description with code and bold".to_string())
        );

        let nested_link = parse::parse_link(
            "[Primary](./primary.md): See [Secondary](./secondary.md) for details",
        );
        let Some(nested_link) = nested_link else {
            panic!("parse_link should not fail for valid input");
        };
        assert_eq!(nested_link.text, "Primary");
        assert_eq!(nested_link.url, "./primary.md");
        assert_eq!(
            nested_link.description,
            Some("See Secondary for details".to_string())
        );

        Ok(())
    }
}
