use super::category::extract_frontmatter;

#[test]
fn test_extract_frontmatter_valid() {
    let content = "---\ntitle: Test\ncategory: concept\n---\n\n# Body";
    let (fm_opt, body) = extract_frontmatter(content);
    assert!(fm_opt.is_some());
    let fm = fm_opt.expect("Expected frontmatter");
    assert_eq!(fm.get("title").expect("Expected title"), "Test");
    assert_eq!(fm.get("category").expect("Expected category"), "concept");
    assert_eq!(body.trim(), "# Body");
}

#[test]
fn test_extract_frontmatter_empty() {
    let content = "---\n---\n# Body";
    let (fm_opt, body) = extract_frontmatter(content);
    assert!(fm_opt.is_some());
    let fm = fm_opt.unwrap();
    assert!(fm.is_empty());
    assert_eq!(body.trim(), "# Body");
}

#[test]
fn test_extract_frontmatter_missing() {
    let content = "# Body without frontmatter\nLine 2";
    let (fm_opt, body) = extract_frontmatter(content);
    assert!(fm_opt.is_none());
    assert_eq!(body.trim(), "# Body without frontmatter\nLine 2");
}

#[test]
fn test_extract_frontmatter_unclosed() {
    let content = "---\ntitle: Test\n\n# Body";
    let (fm_opt, body) = extract_frontmatter(content);
    assert!(fm_opt.is_none());
    assert_eq!(body.trim(), "---\ntitle: Test\n\n# Body");
}

#[test]
fn test_extract_frontmatter_with_colon_in_value() {
    let content = "---\ntitle: Hello: World\ndescription: A test: with colons\n---\nBody";
    let (fm_opt, body) = extract_frontmatter(content);
    assert!(fm_opt.is_some());
    let fm = fm_opt.unwrap();
    assert_eq!(fm.get("title").unwrap(), "Hello: World");
    assert_eq!(fm.get("description").unwrap(), "A test: with colons");
    assert_eq!(body.trim(), "Body");
}

#[test]
fn test_extract_frontmatter_no_colon_lines() {
    let content = "---\njust a line without colon\n---\nBody";
    let (fm_opt, _body) = extract_frontmatter(content);
    assert!(fm_opt.is_some());
    let fm = fm_opt.unwrap();
    assert!(fm.is_empty());
}

#[test]
fn test_extract_frontmatter_crlf() {
    let content = "---\r\ntitle: Test\r\n---\r\nBody";
    let (fm_opt, _body) = extract_frontmatter(content);
    assert!(fm_opt.is_some());
    let fm = fm_opt.unwrap();
    assert_eq!(fm.get("title").unwrap(), "Test");
}
