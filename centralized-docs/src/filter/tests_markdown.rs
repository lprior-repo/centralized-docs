use super::content::{is_footer_line, is_nav_heading};
use super::*;

#[test]
fn test_is_nav_heading() {
    assert!(is_nav_heading("table of contents"));
    assert!(is_nav_heading("navigation"));
    assert!(!is_nav_heading("getting started"));
    assert!(!is_nav_heading("api reference"));
}

#[test]
fn test_is_footer_line() {
    assert!(is_footer_line("copyright 2024 example corp"));
    assert!(is_footer_line("powered by docusaurus"));
    assert!(!is_footer_line("this is regular content"));
}

#[test]
fn test_filter_markdown() {
    let md = "# Title\n\nContent here.\n\n## Table of Contents\n\n- Item 1\n- Item 2\n\n## Real Section\n\nMore content.";
    let config = FilterConfig::default();
    let filtered = filter_markdown(md, &config);
    assert!(filtered.contains("Real Section"));
    assert!(!filtered.contains("Table of Contents"));
}

#[test]
fn test_filter_markdown_nav_sections_skipped() {
    let md = r"# Getting Started

Intro paragraph with enough words to pass filtering.

## Navigation

- Home
- About

## On This Page

Contents list.

## Real Content

This is the real content with enough words to pass the minimum word count filter threshold.";
    let config = FilterConfig::default();
    let filtered = filter_markdown(md, &config);
    assert!(filtered.contains("Getting Started"));
    assert!(filtered.contains("Real Content"));
    assert!(!filtered.contains("Navigation"));
    assert!(!filtered.contains("On This Page"));
}

#[test]
fn test_filter_markdown_footer_lines_skipped() {
    let md = r"# Title

Content paragraph with enough words here to pass the minimum threshold.

Copyright 2024 Example Corp. All rights reserved.";
    let config = FilterConfig::default();
    let filtered = filter_markdown(md, &config);
    assert!(!filtered.contains("Copyright 2024"));
    assert!(!filtered.contains("All rights reserved"));
}

#[test]
fn test_filter_markdown_empty_input() {
    let config = FilterConfig::default();
    let filtered = filter_markdown("", &config);
    assert!(filtered.is_empty());
}

#[test]
fn test_filter_markdown_empty_first_section_kept() {
    let md = "# Short\n\nOnly a few words.";
    let config = FilterConfig::default();
    let filtered = filter_markdown(md, &config);
    assert!(filtered.contains("Short"));
}

#[test]
fn test_filter_markdown_custom_nav_patterns() {
    let md = r"# Guide

## Quickstart

Quickstart content with enough words to pass minimum threshold.

## API Reference

API details with enough words to pass the minimum threshold.";
    let mut config = FilterConfig::default();
    config.nav_patterns = vec!["api reference".to_string()];
    let filtered = filter_markdown(md, &config);
    assert!(filtered.contains("Quickstart"));
    assert!(!filtered.contains("API Reference"));
}

#[test]
fn test_is_footer_line_various_patterns() {
    assert!(is_footer_line("privacy policy"));
    assert!(is_footer_line("terms of service"));
    assert!(is_footer_line("cookie policy"));
    assert!(is_footer_line("built with hugo"));
    assert!(is_footer_line("last updated: 2024"));
    assert!(is_footer_line("© 2024 Company"));
}

#[test]
fn test_is_nav_heading_various() {
    assert!(is_nav_heading("table of contents"));
    assert!(is_nav_heading("menu"));
    assert!(is_nav_heading("see also"));
    assert!(is_nav_heading("related articles"));
    assert!(is_nav_heading("breadcrumb navigation"));
    assert!(!is_nav_heading("Introduction"));
    assert!(!is_nav_heading("API Reference"));
}
