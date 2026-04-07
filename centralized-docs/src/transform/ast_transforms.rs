//! AST-level transform functions: heading fixes, link rewriting, serialization.

use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd};

/// Parse markdown using pulldown-cmark with full CommonMark + GFM support
pub(crate) fn parse_markdown(content: &str) -> Vec<Event<'_>> {
    let options = Options::all();
    let parser = Parser::new_ext(content, options);
    parser.collect()
}

/// Fix heading structure on event stream (no parse/serialize roundtrip)
pub(crate) fn fix_headings_events(events: Vec<Event<'_>>) -> Vec<Event<'_>> {
    events
        .into_iter()
        .scan((None::<u32>, false), |state, event| {
            let in_code_block = state.1;
            let last_heading_level = state.0;

            let new_event = match event {
                Event::Start(Tag::CodeBlock(kind)) => {
                    state.1 = true;
                    Event::Start(Tag::CodeBlock(kind))
                }
                Event::End(TagEnd::CodeBlock) => {
                    state.1 = false;
                    Event::End(TagEnd::CodeBlock)
                }
                Event::Start(Tag::Heading {
                    level,
                    id,
                    classes,
                    attrs,
                }) if !in_code_block => {
                    let new_level = match last_heading_level {
                        Some(last) if heading_level_to_u32(level) > last.saturating_add(1) => {
                            from_u32_level(last.saturating_add(1))
                        }
                        _ => level,
                    };
                    let final_level = if heading_level_to_u32(new_level) > 4 {
                        from_u32_level(4)
                    } else {
                        new_level
                    };
                    state.0 = Some(heading_level_to_u32(final_level));
                    Event::Start(Tag::Heading {
                        level: final_level,
                        id,
                        classes,
                        attrs,
                    })
                }
                other => other,
            };
            Some(new_event)
        })
        .collect()
}

/// Fix heading structure (AST-based, convenience wrapper)
#[allow(dead_code)]
pub(crate) fn fix_headings_ast(content: &str) -> String {
    events_to_markdown(fix_headings_events(parse_markdown(content)))
}

pub(crate) fn from_u32_level(level: u32) -> pulldown_cmark::HeadingLevel {
    match level {
        1 => pulldown_cmark::HeadingLevel::H1,
        2 => pulldown_cmark::HeadingLevel::H2,
        3 => pulldown_cmark::HeadingLevel::H3,
        4 => pulldown_cmark::HeadingLevel::H4,
        5 => pulldown_cmark::HeadingLevel::H5,
        _ => pulldown_cmark::HeadingLevel::H6,
    }
}

pub(crate) fn heading_level_to_u32(level: pulldown_cmark::HeadingLevel) -> u32 {
    match level {
        pulldown_cmark::HeadingLevel::H1 => 1,
        pulldown_cmark::HeadingLevel::H2 => 2,
        pulldown_cmark::HeadingLevel::H3 => 3,
        pulldown_cmark::HeadingLevel::H4 => 4,
        pulldown_cmark::HeadingLevel::H5 => 5,
        pulldown_cmark::HeadingLevel::H6 => 6,
    }
}

/// Rewrite internal links on event stream.
pub(crate) fn rewrite_links_events<'a>(
    events: Vec<Event<'a>>,
    source_path: &str,
    _link_map: &std::collections::HashMap<String, crate::assign::IdMapping>,
    filename_map: &std::collections::HashMap<String, &crate::assign::IdMapping>,
) -> (Vec<Event<'a>>, Vec<String>) {
    use std::path::Path;
    let source_dir = Path::new(source_path)
        .parent()
        .map_or_else(|| Path::new(""), std::convert::identity);

    let results: Vec<(Event<'_>, Option<String>)> = events
        .into_iter()
        .scan(false, |in_code_block, event| {
            let icb = *in_code_block;
            let (new_event, new_broken_link, new_icb) = match event {
                Event::Start(Tag::CodeBlock(kind)) => {
                    (Event::Start(Tag::CodeBlock(kind)), None, true)
                }
                Event::End(TagEnd::CodeBlock) => (Event::End(TagEnd::CodeBlock), None, false),
                Event::Start(Tag::Link {
                    link_type,
                    dest_url,
                    title,
                    id,
                }) if !icb => {
                    let url_str = dest_url.to_string();
                    let (new_url, broken) = if url_str.starts_with("http://")
                        || url_str.starts_with("https://")
                        || url_str.starts_with("mailto:")
                        || url_str.starts_with('#')
                    {
                        (dest_url.clone(), None)
                    } else {
                        let resolved_path = if url_str.starts_with("./") {
                            source_dir.join(url_str.trim_start_matches("./"))
                        } else {
                            source_dir.join(&url_str)
                        };
                        let mapped_filename = resolved_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .and_then(|name| filename_map.get(name))
                            .map(|m| m.filename.clone());
                        match mapped_filename {
                            Some(new_filename) => (CowStr::from(format!("./{new_filename}")), None),
                            None => (dest_url.clone(), Some(url_str)),
                        }
                    };
                    (
                        Event::Start(Tag::Link {
                            link_type,
                            dest_url: new_url,
                            title,
                            id,
                        }),
                        broken,
                        icb,
                    )
                }
                other => (other, None, icb),
            };
            *in_code_block = new_icb;
            Some((new_event, new_broken_link))
        })
        .collect();

    let broken_links: Vec<String> = results.iter().filter_map(|(_, bl)| bl.clone()).collect();
    let transformed_events: Vec<Event<'_>> = results.into_iter().map(|(e, _)| e).collect();
    (transformed_events, broken_links)
}

/// Rewrite links (AST convenience wrapper)
#[allow(dead_code)]
pub(crate) fn rewrite_links_ast(
    content: &str,
    source_path: &str,
    _link_map: &std::collections::HashMap<String, crate::assign::IdMapping>,
    filename_map: &std::collections::HashMap<String, &crate::assign::IdMapping>,
) -> (String, Vec<String>) {
    let (events, broken) = rewrite_links_events(
        parse_markdown(content),
        source_path,
        _link_map,
        filename_map,
    );
    (events_to_markdown(events), broken)
}

/// Convert events to markdown using pulldown-cmark-to-cmark
pub(crate) fn events_to_markdown<'a, I>(events: I) -> String
where
    I: IntoIterator<Item = Event<'a>>,
{
    #[allow(unused_mut)]
    let mut buf = String::new();
    if let Err(e) = pulldown_cmark_to_cmark::cmark(events.into_iter(), &mut buf) {
        eprintln!("Warning: pulldown_cmark_to_cmark failed: {e}");
        if buf.is_empty() {
            return String::from("[ERROR: markdown serialization failed]");
        }
    }
    buf
}
