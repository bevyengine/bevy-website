use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use std::fmt::Write;

/// Writes the markdown section of the given section header to the output.
/// The header name needs to be in lower case.
pub fn write_markdown_section(
    body: &str,
    section_header: &str,
    write_todo: bool,
) -> anyhow::Result<(String, bool)> {
    let mut output = String::new();
    // Parse the body of the PR
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    let mut markdown = Parser::new_ext(body, options);
    let mut section_found = false;

    while let Some(event) = markdown.next() {
        if section_found {
            break;
        }

        let Event::Start(Tag::Heading(heading_level, _, _)) = event else {
            continue;
        };

        // Find the section header
        // Sometimes people will write code in the header
        if let Some(Event::Text(heading_text) | Event::Code(heading_text)) = markdown.next() {
            if !heading_text.to_lowercase().contains(section_header) {
                continue;
            }
        }

        section_found = true;
        markdown.next(); // skip heading end event

        // Write the section's content
        let mut list_item_level = 0;
        for event in markdown.by_ref() {
            match event {
                Event::Start(Tag::Heading(level, _, _)) => {
                    if level <= heading_level {
                        // go until next heading
                        break;
                    }
                }
                Event::Start(Tag::List(_)) => list_item_level += 1,
                Event::End(Tag::List(_)) => list_item_level -= 1,
                Event::End(Tag::Heading(level, _, _)) => {
                    if level == heading_level {
                        println!("!!! end of heading !!!");
                    }
                }
                Event::Start(Tag::Link(_, _, _)) => {
                    write!(output, "[")?;
                    continue;
                }
                Event::End(Tag::Link(_, ref link, _)) => {
                    write!(output, "]({link})")?;
                    continue;
                }
                _ => {}
            }
            let event = write_markdown_event(&event, list_item_level - 1)?;
            write!(output, "{event}")?;
        }
    }

    if !section_found {
        // Someone didn't write a migration guide 😢
        if write_todo {
            writeln!(output, "\n<!-- TODO -->")?;
        }
        Ok((output, false))
    } else {
        Ok((output, true))
    }
}

/// Write the markdown Event based on the Tag
/// This handles some edge cases like some code blocks not having a specified lang
/// This also makes sure the result has a more consistent formatting
fn write_markdown_event(event: &Event, list_item_level: i32) -> anyhow::Result<String> {
    let mut output = String::new();
    #[allow(clippy::match_same_arms)]
    match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => writeln!(
            output,
            "\n```{}",
            if lang.is_empty() {
                "rust".to_string()
            } else {
                lang.to_string()
            }
        )?,
        Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => writeln!(output, "```")?,
        Event::Start(Tag::CodeBlock(CodeBlockKind::Indented)) => writeln!(output, "\n```",)?,
        Event::End(Tag::CodeBlock(CodeBlockKind::Indented)) => writeln!(output, "```")?,
        Event::Start(Tag::Emphasis) | Event::End(Tag::Emphasis) => write!(output, "_")?,
        Event::Start(Tag::Strong) | Event::End(Tag::Strong) => write!(output, "**",)?,
        Event::Start(Tag::Heading(_, _, _)) => {
            // A few guides used headings for emphasis,
            // since we use headings for the actual header of the guide, we need to use a different way to convey emphasis
            write!(output, "\n__")?;
        }
        Event::End(Tag::Heading(_, _, _)) => writeln!(output, "__")?,
        // FIXME List currently always assume they are unordered
        Event::Start(Tag::List(_)) => writeln!(output)?,
        Event::End(Tag::List(_)) => {}
        Event::Start(Tag::Item) => {
            // Add indentation
            for _ in 0..list_item_level {
                write!(output, "  ")?;
            }
            write!(output, "- ")?;
        }
        Event::End(Tag::Item) => writeln!(output)?,
        Event::Start(Tag::Paragraph) => writeln!(output)?,
        Event::End(Tag::Paragraph) => writeln!(output)?,
        Event::Text(text) => write!(output, "{text}")?,
        Event::Code(text) => write!(output, "`{text}`")?,
        Event::SoftBreak => writeln!(output)?,
        Event::Start(Tag::BlockQuote) => write!(output, "\n> ")?,
        Event::End(Tag::BlockQuote) => writeln!(output)?,
        Event::Html(html) => write!(output, "{html}")?,
        Event::Rule => writeln!(output, "---")?,
        _ => println!("\x1b[93mUnknown event: {event:?}\x1b[0m"),
    };
    Ok(output)
}
