//! This will generate a markdown file (out.md) containing all the migration guides
//! from PRs marked as `C-Breaking-Change`.
//!
//! Example used to generate for 0.9:
//! cargo r -- --date 2022-07-31 --title "0.8 to 0.9" -w 5

use clap::Parser as ClapParser;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use serde::Deserialize;
use std::fmt::Write;

#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Date of the release of the previous version
    /// Format: YYYY-MM-DD
    #[arg(short, long)]
    date: String,

    /// Title of the frontmatter
    #[arg(short, long)]
    title: String,

    /// Title of the frontmatter
    #[arg(short, long)]
    weight: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let _ = dotenv::dotenv();

    let mut output = String::new();
    write!(
        &mut output,
        r#"+++
title = "{}"
weight = {}
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
[extra]
long_title = "Migration Guide: {}"
+++"#,
        args.title, args.weight, args.title
    )?;
    writeln!(&mut output)?;

    let prs = get_breaking_prs(&args.date)?;
    for pr in &prs {
        println!("## {}", pr.title.replace("[Merged by Bors] - ", "").trim());

        writeln!(
            &mut output,
            "\n### [{}](https://github.com/bevyengine/bevy/pull/{})",
            pr.title.replace("[Merged by Bors] - ", "").trim(),
            pr.number
        )?;
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);
        let mut markdown = Parser::new_ext(&pr.body, options);
        let mut guide_found = false;
        while let Some(event) = markdown.next() {
            if let Event::Start(Tag::Heading(migration_guide_level, _, _)) = event {
                if let Some(Event::Text(heading_text)) = markdown.next() {
                    if !heading_text.to_lowercase().contains("migration guide") {
                        continue;
                    }
                }
                guide_found = true;
                markdown.next(); // skip heading end
                while let Some(event) = markdown.next() {
                    if let Event::Start(Tag::Heading(level, _, _)) = event {
                        if level >= migration_guide_level {
                            // go until next heading
                            break;
                        }
                    }
                    match event {
                        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => writeln!(
                            &mut output,
                            "\n```{}",
                            if lang.is_empty() {
                                "rust".to_string()
                            } else {
                                lang.to_string()
                            }
                        )?,
                        Event::End(Tag::CodeBlock(_)) => writeln!(&mut output, "```")?,
                        Event::Start(Tag::Emphasis) | Event::End(Tag::Emphasis) => {
                            write!(&mut output, "_")?
                        }
                        Event::Start(Tag::List(_)) => {}
                        Event::End(Tag::List(_)) => writeln!(&mut output)?,
                        Event::Start(Tag::Item) => write!(&mut output, "\n* ")?,
                        Event::End(Tag::Item) => {}
                        Event::Start(tag) | Event::End(tag) if matches!(tag, Tag::Paragraph) => {
                            writeln!(&mut output)?
                        }
                        Event::Text(text) => write!(&mut output, "{text}")?,
                        Event::Code(text) => write!(&mut output, "`{text}`")?,
                        Event::SoftBreak => writeln!(&mut output)?,
                        _ => println!("unknown event {:?}", event),
                    };
                }
            }
        }

        if !guide_found {
            writeln!(&mut output, "\n<!-- TODO -->")?;
            println!("\x1b[93mMigration Guide not found!\x1b[0m");
        }
    }

    std::fs::write("./out.md", output)?;

    println!("\nFound {} breaking PRs merged by bors", prs.len());

    Ok(())
}

#[derive(Deserialize, Clone)]
struct GithubIssuesResponse {
    title: String,
    number: i32,
    body: String,
}

fn get_breaking_prs(date: &str) -> anyhow::Result<Vec<GithubIssuesResponse>> {
    let token = std::env::var("GITHUB_TOKEN")
        .expect("GITHUB_TOKEN not found, github links will be skipped");
    let agent: ureq::Agent = ureq::AgentBuilder::new()
        .user_agent("bevy-website-generate-migration-guide")
        .build();
    let response: Vec<GithubIssuesResponse> = agent
        .get(&format!(
            "https://api.github.com/repos/bevyengine/bevy/issues"
        ))
        .set("Accept", "application/json")
        .set("Authorization", &format!("Bearer {}", token))
        .query("state", "closed")
        .query("labels", "C-Breaking-Change")
        // release date of 0.8, could probably be automated
        .query("since", &format!("{}T00:00:00Z", date))
        .query("per_page", "100")
        .call()?
        .into_json()?;
    Ok(response
        .iter()
        .filter(|pr| pr.title.starts_with("[Merged by Bors] - "))
        .cloned()
        .collect())
}
