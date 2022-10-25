//! This will generate a markdown file (out.md) containing all the migration guides
//! from PRs marked as `C-Breaking-Change`.
//!
//! Requires a valid GITHUB_TOKEN, you can use a .env file or use your prefered method of passing env arguments
//!
//! Example used to generate for 0.9:
//! cargo r -- migration-guide --date 2022-07-31 --title "0.8 to 0.9" --weight 5

use clap::{Parser as ClapParser, Subcommand};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use serde::Deserialize;
use std::{any, fmt::Write, path::PathBuf};

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    MigrationGuide {
        /// Date of the release of the previous version. Format: YYYY-MM-DD
        #[arg(short, long)]
        date: String,

        /// Title of the frontmatter
        #[arg(short, long)]
        title: String,

        /// Weight used for sorting
        #[arg(short, long)]
        weight: i32,

        /// Path used to output the generated file. Defaults to ./migration-guide.md
        #[arg(short, long)]
        path: Option<std::path::PathBuf>,
    },
    ReleaseNote {
        /// Date of the release of the previous version. Format: YYYY-MM-DD
        #[arg(short, long)]
        date: String,

        /// Path used to output the generated file. Defaults to ./release-note.md
        #[arg(short, long)]
        path: Option<std::path::PathBuf>,
    },
}

fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();
    let args = Args::parse();
    let args = match args.command {
        Commands::MigrationGuide {
            date,
            title,
            weight,
            path,
        } => generate_migration_guide(
            &title,
            weight,
            &date,
            path.unwrap_or(PathBuf::from("./migration-guide.md")),
        )?,
        Commands::ReleaseNote { date, path } => {
            generate_release_note(&date, path.unwrap_or(PathBuf::from("./release-note.md")))?
        }
    };

    Ok(())
}

fn generate_release_note(date: &str, path: PathBuf) -> anyhow::Result<()> {
    let mut prs = Vec::<GithubIssuesResponse>::with_capacity(100);
    let mut page = 1;
    loop {
        println!("Page: {}", page);
        let mut prs_in_page = get_merged_prs(date, page, None)?;
        if prs_in_page.is_empty() {
            break;
        } else {
            page += 1;
        }

        prs.append(&mut prs_in_page);
    }

    let mut output = String::new();
    writeln!(&mut output, "# Full Changelog")?;
    for pr in &prs {
        let mut pr_title = pr
            .title
            .replace("[Merged by Bors] - ", "")
            .trim()
            .to_string();
        writeln!(&mut output, "\n## {}\n", pr_title)?;
        writeln!(
            &mut output,
            "Labels: {:?}",
            pr.labels.iter().map(|l| l.name.clone()).collect::<Vec<_>>()
        )?;
        writeln!(&mut output, "Created by: {}", pr.user.login)?;
        writeln!(
            &mut output,
            "link: <https://github.com/bevyengine/bevy/pull/{}>",
            pr.number
        )?;
    }

    println!("Found {} prs merged by bors since {}", prs.len(), date);

    std::fs::write(path, output)?;

    Ok(())
}

fn generate_migration_guide(
    title: &str,
    weight: i32,
    date: &str,
    path: PathBuf,
) -> anyhow::Result<()> {
    let mut output = String::new();

    // Write the frontmatter based on given parameters
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
        title, weight, title
    )?;
    writeln!(&mut output)?;

    let prs = get_merged_prs(&date, 0, Some("C-Breaking-Change"))?;
    for pr in &prs {
        let mut pr_title = pr
            .title
            .replace("[Merged by Bors] - ", "")
            .trim()
            .to_string();
        println!("# {}", pr_title);

        // Write title for the PR with correct heading and github url
        writeln!(
            &mut output,
            "\n### [{}](https://github.com/bevyengine/bevy/pull/{})",
            pr_title, pr.number
        )?;

        // Parse the body of the PR
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);
        let mut markdown = Parser::new_ext(&pr.body.as_ref().unwrap(), options);
        let mut guide_found = false;

        while let Some(event) = markdown.next() {
            if let Event::Start(Tag::Heading(migration_guide_level, _, _)) = event {
                // Find the migration guide section
                if let Some(Event::Text(heading_text)) = markdown.next() {
                    if !heading_text.to_lowercase().contains("migration guide") {
                        continue;
                    }
                }
                guide_found = true;
                markdown.next(); // skip heading end

                // Write the migration guide section
                while let Some(event) = markdown.next() {
                    if let Event::Start(Tag::Heading(level, _, _)) = event {
                        if level >= migration_guide_level {
                            // go until next heading
                            break;
                        }
                    }
                    // Write the markdown Event based on the Tag
                    // This handles some edge cases like some code blocks not having a specified lang
                    // This also makes sure the result has a more consistent formatting
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
                        // FIXME List currently always assume they are unordered
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
            // Someone didn't write a migration guide 😢
            writeln!(&mut output, "\n<!-- TODO -->")?;
            println!("\x1b[93mMigration Guide not found!\x1b[0m");
        }
    }

    println!("\nFound {} breaking PRs merged by bors", prs.len());

    std::fs::write(path, output)?;

    Ok(())
}

#[derive(Deserialize, Clone, Debug)]
struct GithubIssuesResponse {
    title: String,
    number: i32,
    body: Option<String>,
    labels: Vec<GithubLabel>,
    user: GithubUser,
}

#[derive(Deserialize, Clone, Debug)]
struct GithubLabel {
    name: String,
}

#[derive(Deserialize, Clone, Debug)]
struct GithubUser {
    login: String,
}

// TODO handle pages
fn get_merged_prs(
    date: &str,
    page: i32,
    label: Option<&str>,
) -> anyhow::Result<Vec<GithubIssuesResponse>> {
    let token = std::env::var("GITHUB_TOKEN")
        .expect("GITHUB_TOKEN not found, github links will be skipped");
    let agent: ureq::Agent = ureq::AgentBuilder::new()
        .user_agent("bevy-website-generate-migration-guide")
        .build();
    let mut request = agent
        .get(&format!(
            "https://api.github.com/repos/bevyengine/bevy/issues"
        ))
        .set("Accept", "application/json")
        .set("Authorization", &format!("Bearer {}", token))
        .query("state", "closed")
        .query("since", &format!("{}T00:00:00Z", date))
        .query("per_page", "100")
        .query("page", &page.to_string());
    if let Some(label) = label {
        request = request.query("labels", label);
    }
    let response: Vec<GithubIssuesResponse> = request.call()?.into_json()?;
    Ok(response
        .iter()
        // Make sure to only get the PRs that were merged by bors
        .filter(|pr| pr.title.starts_with("[Merged by Bors] - "))
        .cloned()
        .collect())
}
