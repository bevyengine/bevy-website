use clap::{Parser as ClapParser, Subcommand};
use github_client::GithubClient;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
    path::PathBuf,
};
mod github_client;

/// Generates markdown files used for a bevy releases.
///
/// Migration Guide:
/// * Gets all PRs with the `C-Breaking-Change` label and that were merged by bors.
/// * For each PR:
///     * Generate the title with a link to the relevant PR and
///     * Generate the migration guide section. This parses the markdown and generates valid makrdown that should pass markdownlint rules.
///
/// Release notes:
/// * Gets all PRs merged by bors
/// * Collect each author of closed PRs (Should this just list all contributors?)
/// * Sort each PR per area label
/// * Generate the list of merge PR
///
/// Requires a valid GITHUB_TOKEN environment variable, you can use a .env file or use your prefered method of passing env arguments.
///
/// Example used to generate for 0.9:
/// cargo run -- migration-guide --date 2022-07-31 --title "0.8 to 0.9" --weight 5
/// cargo run -- release-note --date 2022-07-31
#[derive(ClapParser)]
#[command(author, version, about)]
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

    let mut client = github_client::GithubClient::new(
        std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found"),
    );

    let args = Args::parse();
    match args.command {
        Commands::MigrationGuide {
            date,
            title,
            weight,
            path,
        } => generate_migration_guide(
            &title,
            weight,
            &date,
            path.unwrap_or_else(|| PathBuf::from("./migration-guide.md")),
            &mut client,
        )?,
        Commands::ReleaseNote { date, path } => generate_release_note(
            &date,
            path.unwrap_or_else(|| PathBuf::from("./release-note.md")),
            &mut client,
        )?,
    };

    Ok(())
}

/// Generates the list of contributors and a list of all closed PRs sorted by area labels
fn generate_release_note(
    date: &str,
    path: PathBuf,
    client: &mut GithubClient,
) -> anyhow::Result<()> {
    let prs = client.get_merged_prs(date, None)?;

    let mut authors = HashSet::new();
    let mut pr_map = HashMap::new();
    let mut areas = HashMap::<String, Vec<i32>>::new();
    for pr in &prs {
        authors.insert(pr.user.login.clone());
        pr_map.insert(pr.number, pr.clone());

        let area = if let Some(label) = pr.labels.iter().find(|l| l.name.starts_with("A-")) {
            label.name.clone()
        } else {
            String::from("No area label")
        };

        areas.entry(area).or_default().push(pr.number);
    }

    println!("Found {} prs merged by bors since {}", prs.len(), date);

    let mut output = String::new();

    writeln!(&mut output, "## Contributors\n")?;
    writeln!(&mut output, "A huge thanks to the {} contributors that made this release (and associated docs) possible! In random order:\n", authors.len())?;
    for author in &authors {
        writeln!(&mut output, "- @{}", author)?;
    }
    writeln!(&mut output)?;

    writeln!(&mut output, "## Full Changelog")?;

    for (area, prs) in &areas {
        writeln!(&mut output)?;
        writeln!(&mut output, "## {}", area)?;
        writeln!(&mut output)?;

        for pr_number in prs {
            let Some(pr) = pr_map.get(pr_number) else {
                continue;
            };
            let pr_title = pr
                .title
                .replace("[Merged by Bors] - ", "")
                .trim()
                .to_string();

            writeln!(&mut output, "- [{}][{}]", pr_title, pr_number)?;
        }
    }

    writeln!(&mut output)?;

    for pr in prs {
        writeln!(
            &mut output,
            "[{}]: https://github.com/bevyengine/bevy/pull/{}",
            pr.number, pr.number
        )?;
    }

    std::fs::write(path, output)?;

    Ok(())
}

fn generate_migration_guide(
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
title = "{}"
weight = {}
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
[extra]
long_title = "Migration Guide: {}"
+++

Bevy relies heavily on improvements in the Rust language and compiler.
As a result, the Minimum Supported Rust Version (MSRV) is "the latest stable release" of Rust."#,
        title, weight, title
    )?;
    writeln!(&mut output)?;

    let prs = client.get_merged_prs(date, Some("C-Breaking-Change"))?;
    for pr in &prs {
        let pr_title = pr
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

        write_markdown_section(pr.body.as_ref().unwrap(), "migration guide", &mut output)?;
    }

    println!("\nFound {} breaking PRs merged by bors", prs.len());

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
        println!("\x1b[93m{} not found!\x1b[0m", section_header);
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
        _ => println!("unknown event {:?}", event),
    };
    Ok(())
}
