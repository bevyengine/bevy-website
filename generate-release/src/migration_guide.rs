use crate::{github_client::GithubClient, helpers::get_merged_prs};
use anyhow::Context;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use std::{fmt::Write, path::PathBuf};

pub fn generate_migration_guide(
    title: &str,
    weight: i32,
    date: &str,
    path: PathBuf,
    client: &mut GithubClient,
) -> anyhow::Result<()> {
    let mut output = String::new();

    // Write the frontmatter based on given parameters
    write!(
        &mut output,
        r#"+++
title = "{title}"
weight = {weight}
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
[extra]
long_title = "Migration Guide: {title}"
+++

Bevy relies heavily on improvements in the Rust language and compiler.
As a result, the Minimum Supported Rust Version (MSRV) is "the latest stable release" of Rust."#
    )?;
    writeln!(&mut output)?;

    let main_sha = client
        .get_branch_sha("main")
        .context("Failed to get branch_sha")?;

    println!("commit sha for main: {main_sha}");

    let merged_breaking_prs = get_merged_prs(client, date, &main_sha, Some("C-Breaking-Change"))?;
    for (pr, _, title) in &merged_breaking_prs {
        println!("# {title}");

        // Write title for the PR with correct heading and github url
        writeln!(
            &mut output,
            "\n### [{}](https://github.com/bevyengine/bevy/pull/{})",
            title, pr.number
        )?;
        write_markdown_section(pr.body.as_ref().unwrap(), "migration guide", &mut output)?;
    }

    println!(
        "\nFound {} breaking PRs merged by bors",
        merged_breaking_prs.len()
    );

    std::fs::write(path, output)?;

    Ok(())
}

/// Writes the markdown section of the givent section header to the output.
/// The header name needs to be in lower case.
fn write_markdown_section(
    body: &str,
    section_header: &str,
    output: &mut String,
) -> anyhow::Result<bool> {
    // Parse the body of the PR
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    let mut markdown = Parser::new_ext(body, options);
    let mut section_found = false;

    while let Some(event) = markdown.next() {
        if let Event::Start(Tag::Heading(migration_guide_level, _, _)) = event {
            // Find the section header
            if let Some(Event::Text(heading_text)) = markdown.next() {
                if !heading_text.to_lowercase().contains(section_header) {
                    continue;
                }
            }
            section_found = true;
            markdown.next(); // skip heading end

            // Write the section's content
            for event in markdown.by_ref() {
                if let Event::Start(Tag::Heading(level, _, _)) = event {
                    if level >= migration_guide_level {
                        // go until next heading
                        break;
                    }
                }
                write_markdown_event(&event, output)?;
            }
        }
    }

    if !section_found {
        // Someone didn't write a migration guide 😢
        writeln!(output, "\n<!-- TODO -->")?;
        println!("\x1b[93m{section_header} not found!\x1b[0m");
        Ok(false)
    } else {
        Ok(true)
    }
}

/// Write the markdown Event based on the Tag
/// This handles some edge cases like some code blocks not having a specified lang
/// This also makes sure the result has a more consistent formatting
fn write_markdown_event(event: &Event, output: &mut String) -> anyhow::Result<()> {
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
        Event::End(Tag::CodeBlock(_)) => writeln!(output, "```")?,
        Event::Start(Tag::Emphasis) | Event::End(Tag::Emphasis) => write!(output, "_")?,
        // FIXME List currently always assume they are unordered
        Event::Start(Tag::List(_)) => {}
        Event::End(Tag::List(_)) => writeln!(output)?,
        Event::Start(Tag::Item) => write!(output, "\n* ")?,
        Event::End(Tag::Item) => {}
        Event::Start(tag) | Event::End(tag) if matches!(tag, Tag::Paragraph) => writeln!(output)?,
        Event::Text(text) => write!(output, "{text}")?,
        Event::Code(text) => write!(output, "`{text}`")?,
        Event::SoftBreak => writeln!(output)?,
        _ => println!("unknown event {event:?}"),
    };
    Ok(())
}
