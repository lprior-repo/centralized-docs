//! AST context injection, H1 enforcement, and helper functions.

use crate::analyze::Analysis;
use crate::types::is_stopword;
use itertools::Itertools;
use pulldown_cmark::{CowStr, Event, Tag, TagEnd};
use unicode_segmentation::UnicodeSegmentation;

/// Ensure document has exactly one H1 heading on event stream.
pub(crate) fn ensure_h1_events<'a>(events: Vec<Event<'a>>, title: &str) -> Vec<Event<'a>> {
    let h1_count = events
        .iter()
        .filter(|e| {
            matches!(
                e,
                Event::Start(Tag::Heading {
                    level: pulldown_cmark::HeadingLevel::H1,
                    ..
                })
            )
        })
        .count();

    if h1_count == 1 {
        return events;
    }

    let bump_level = |level: pulldown_cmark::HeadingLevel| -> pulldown_cmark::HeadingLevel {
        match level {
            pulldown_cmark::HeadingLevel::H1 => pulldown_cmark::HeadingLevel::H2,
            pulldown_cmark::HeadingLevel::H2 => pulldown_cmark::HeadingLevel::H3,
            pulldown_cmark::HeadingLevel::H3 => pulldown_cmark::HeadingLevel::H4,
            pulldown_cmark::HeadingLevel::H4 => pulldown_cmark::HeadingLevel::H5,
            pulldown_cmark::HeadingLevel::H5 | pulldown_cmark::HeadingLevel::H6 => {
                pulldown_cmark::HeadingLevel::H6
            }
        }
    };

    let header_events = vec![
        Event::Start(Tag::Heading {
            level: pulldown_cmark::HeadingLevel::H1,
            id: None,
            classes: vec![],
            attrs: vec![],
        }),
        Event::Text(CowStr::from(title.to_string())),
        Event::End(TagEnd::Heading(pulldown_cmark::HeadingLevel::H1)),
        Event::SoftBreak,
        Event::SoftBreak,
        Event::SoftBreak,
    ];

    header_events
        .into_iter()
        .chain(events.into_iter().map(move |event| match event {
            Event::Start(Tag::Heading {
                level,
                id,
                classes,
                attrs,
            }) if h1_count > 1 => Event::Start(Tag::Heading {
                level: bump_level(level),
                id,
                classes,
                attrs,
            }),
            Event::End(TagEnd::Heading(level)) if h1_count > 1 => {
                Event::End(TagEnd::Heading(bump_level(level)))
            }
            other => other,
        }))
        .collect()
}

/// Ensure H1 (AST convenience wrapper)
#[allow(dead_code)]
pub(crate) fn ensure_h1_ast(content: &str, title: &str) -> String {
    let events = super::ast_transforms::parse_markdown(content);
    let h1_count = events
        .iter()
        .filter(|e| {
            matches!(
                e,
                Event::Start(Tag::Heading {
                    level: pulldown_cmark::HeadingLevel::H1,
                    ..
                })
            )
        })
        .count();
    if h1_count == 1 {
        return content.to_string();
    }
    super::ast_transforms::events_to_markdown(ensure_h1_events(events, title))
}

/// Check if events contain a context blockquote with "Context" text
pub(crate) fn events_have_blockquote_context(events: &[Event<'_>]) -> bool {
    events
        .iter()
        .fold((false, false), |(in_blockquote, found), event| {
            if found {
                (in_blockquote, true)
            } else {
                match event {
                    Event::Start(Tag::BlockQuote(_)) => (true, false),
                    Event::End(TagEnd::BlockQuote(_)) => (false, false),
                    Event::Text(text) if in_blockquote && text.contains("Context") => {
                        (in_blockquote, true)
                    }
                    _ => (in_blockquote, false),
                }
            }
        })
        .1
}

/// Check if content already has a context blockquote
#[allow(dead_code)]
pub(crate) fn content_has_blockquote_context(content: &str) -> bool {
    events_have_blockquote_context(&super::ast_transforms::parse_markdown(content))
}

/// Inject context blockquote after H1 on event stream.
pub(crate) fn inject_context_events<'a>(
    events: Vec<Event<'a>>,
    context_text: &str,
) -> Vec<Event<'a>> {
    let h1_end_pos = events.iter().position(|e| {
        matches!(
            e,
            Event::End(TagEnd::Heading(pulldown_cmark::HeadingLevel::H1))
        )
    });

    match h1_end_pos {
        None => events,
        Some(pos) => {
            let (before, after) = events.split_at(pos.saturating_add(1));
            let context_block: Vec<Event<'_>> = vec![
                Event::SoftBreak,
                Event::SoftBreak,
                Event::Start(Tag::BlockQuote(None)),
                Event::Start(Tag::Paragraph),
                Event::Start(Tag::Strong),
                Event::Text(CowStr::from("Context")),
                Event::End(TagEnd::Strong),
                Event::Text(CowStr::from(": ")),
                Event::Text(CowStr::from(context_text.to_string())),
                Event::End(TagEnd::Paragraph),
                Event::End(TagEnd::BlockQuote(None)),
                Event::SoftBreak,
                Event::SoftBreak,
            ];
            before
                .iter()
                .cloned()
                .chain(context_block)
                .chain(after.iter().cloned())
                .collect()
        }
    }
}

/// Inject context block after H1 (AST convenience wrapper)
#[allow(dead_code)]
pub(crate) fn inject_context_block_ast(content: &str, context_text: &str) -> String {
    let events = super::ast_transforms::parse_markdown(content);
    let h1_end_pos = events.iter().position(|e| {
        matches!(
            e,
            Event::End(TagEnd::Heading(pulldown_cmark::HeadingLevel::H1))
        )
    });
    match h1_end_pos {
        None => super::ast_transforms::events_to_markdown(events),
        Some(pos) => {
            let (before, after) = events.split_at(pos.saturating_add(1));
            let ctx: Vec<Event<'_>> = vec![
                Event::SoftBreak,
                Event::SoftBreak,
                Event::Start(Tag::BlockQuote(None)),
                Event::Start(Tag::Paragraph),
                Event::Start(Tag::Strong),
                Event::Text(CowStr::from("Context")),
                Event::End(TagEnd::Strong),
                Event::Text(CowStr::from(": ")),
                Event::Text(CowStr::from(context_text.to_string())),
                Event::End(TagEnd::Paragraph),
                Event::End(TagEnd::BlockQuote(None)),
                Event::SoftBreak,
                Event::SoftBreak,
            ];
            super::ast_transforms::events_to_markdown(
                before
                    .iter()
                    .cloned()
                    .chain(ctx)
                    .chain(after.iter().cloned())
                    .collect::<Vec<_>>(),
            )
        }
    }
}

/// Check if content already has "## See Also" section
pub(crate) fn content_has_see_also(content: &str) -> bool {
    content.contains("## See Also")
}

/// Safely truncate a string to a maximum number of Unicode grapheme clusters
pub(crate) fn safe_truncate_chars(text: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }
    text.graphemes(true).take(max_chars).collect::<String>()
}

/// Generate tags using functional composition
pub(crate) fn generate_tags(analysis: &Analysis) -> Vec<String> {
    std::iter::once(analysis.category.clone())
        .chain(
            analysis
                .headings
                .iter()
                .take(3)
                .flat_map(|h| h.text.split_whitespace())
                .filter(|word| word.len() > 4 && !is_stopword(word))
                .map(str::to_lowercase),
        )
        .sorted()
        .dedup()
        .take(5)
        .collect()
}
