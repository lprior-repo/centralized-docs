use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

pub struct MarkdownMetadata {
    pub title: Option<String>,
    pub headings: Vec<crate::analyze::Heading>,
    pub links: Vec<crate::analyze::Link>,
    pub first_paragraph: String,
    pub has_code: bool,
    pub has_tables: bool,
}

#[derive(Default)]
#[allow(clippy::struct_excessive_bools)]
struct MetadataState {
    title: Option<String>,
    headings: Vec<crate::analyze::Heading>,
    links: Vec<crate::analyze::Link>,
    first_paragraph: String,
    has_code: bool,
    has_tables: bool,
    current_heading: Option<crate::analyze::Heading>,
    current_link: Option<crate::analyze::Link>,
    in_first_paragraph: bool,
    found_first_paragraph: bool,
}

// AST event accumulation: pulldown-cmark events require sequential stateful
// traversal (heading/link text spans multiple events). Persistent structures
// would add O(log n) overhead per event with no functional benefit.
#[allow(unused_mut)]
#[allow(clippy::too_many_lines)]
pub fn extract_markdown_metadata(content: &str) -> MarkdownMetadata {
    use crate::analyze::{Heading, Link, LinkKind};

    let line_starts: Vec<usize> = std::iter::once(0)
        .chain(
            content
                .match_indices('\n')
                .map(|(i, _)| i.saturating_add(1)),
        )
        .collect();

    let parser = Parser::new(content).into_offset_iter();

    let final_state = parser.fold(MetadataState::default(), |mut state, (event, range)| {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                let level_num = match level {
                    HeadingLevel::H1 => 1,
                    HeadingLevel::H2 => 2,
                    HeadingLevel::H3 => 3,
                    HeadingLevel::H4 => 4,
                    HeadingLevel::H5 => 5,
                    HeadingLevel::H6 => 6,
                };
                let line_num = line_starts.partition_point(|&x| x <= range.start);
                state.current_heading = Some(Heading {
                    level: level_num,
                    text: String::new(),
                    line: line_num.saturating_sub(1),
                });
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(mut h) = state.current_heading.take() {
                    h.text = h.text.trim().to_string();
                    if h.level == 1 && state.title.is_none() {
                        state.title = Some(h.text.clone());
                    }
                    state.headings.push(h);
                }
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                let target = dest_url.to_string();
                let kind = if target.starts_with("http://")
                    || target.starts_with("https://")
                    || target.starts_with("mailto:")
                {
                    LinkKind::External
                } else {
                    LinkKind::Internal
                };
                state.current_link = Some(Link {
                    text: String::new(),
                    target,
                    kind,
                });
            }
            Event::End(TagEnd::Link) => {
                if let Some(l) = state.current_link.take() {
                    state.links.push(l);
                }
            }
            Event::Start(Tag::Paragraph) => {
                if !state.found_first_paragraph {
                    state.in_first_paragraph = true;
                }
            }
            Event::End(TagEnd::Paragraph) => {
                if state.in_first_paragraph {
                    state.in_first_paragraph = false;
                    state.found_first_paragraph = true;
                }
            }
            Event::Start(Tag::CodeBlock(_)) => state.has_code = true,
            Event::Start(Tag::Table(_)) => state.has_tables = true,
            Event::Text(text) | Event::Code(text) => {
                if let Some(h) = &mut state.current_heading {
                    h.text.push_str(&text);
                }
                if let Some(l) = &mut state.current_link {
                    l.text.push_str(&text);
                }
                if state.in_first_paragraph && state.first_paragraph.len() < 200 {
                    state.first_paragraph.push_str(&text);
                    state.first_paragraph.push(' ');
                }
            }
            _ => {}
        }
        state
    });

    MarkdownMetadata {
        title: final_state.title,
        headings: final_state.headings,
        links: final_state.links,
        first_paragraph: final_state.first_paragraph.trim().to_string(),
        has_code: final_state.has_code,
        has_tables: final_state.has_tables,
    }
}
