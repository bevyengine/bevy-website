use anyhow::Context;

use crate::{
    github_client::{GithubClient, GithubIssuesResponse},
    helpers::{get_merged_prs, get_pr_area},
    markdown::write_markdown_section,
};
use std::{collections::BTreeMap, fmt::Write, io::Write as IoWrite, path::PathBuf};

pub fn generate_migration_guide(
    title: &str,
    weight: i32,
    from: &str,
    to: &str,
    path: PathBuf,
    client: &mut GithubClient,
) -> anyhow::Result<()> {
    let mut output = String::new();

    // Write the frontmatter based on given parameters
    write!(
        &mut output,
        r#"+++
title = "{title}"
insert_anchor_links = "right"
[extra]
weight = {weight}
long_title = "Migration Guide: {title}"
+++

Bevy relies heavily on improvements in the Rust language and compiler.
As a result, the Minimum Supported Rust Version (MSRV) is "the latest stable release" of Rust."#
    )?;
    writeln!(&mut output)?;
    writeln!(&mut output, "<div class=\"migration-guide\">")?;

    let mut areas = BTreeMap::<String, Vec<(String, GithubIssuesResponse)>>::new();

    let merged_prs = get_merged_prs(client, from, to, None)?;
    let mut count = 0;
    for (pr, _, title) in &merged_prs {
        let Some(body) = pr.body.as_ref() else {
            // If the body is empty then there's no migration guide so we can safely skip it
            continue;
        };

        let has_migration_guide_section = body.to_lowercase().contains("## migration guide");
        let has_breaking_label = pr
            .labels
            .iter()
            .any(|l| l.name.contains("C-Breaking-Change"));

        // We want to check for PRs with the breaking label but without the guide section
        // to make it easier to track down missing guides
        if has_migration_guide_section || has_breaking_label {
            let area = get_pr_area(pr);
            areas
                .entry(area)
                .or_default()
                .push((title.clone(), pr.clone()));
            count += 1;
        }
    }

    let dir = "./migration-guides";
    std::fs::create_dir_all(dir).context(format!("Failed to create {dir}"))?;

    for (area, prs) in areas {
        println!("Area: {area}");

        let area = area.replace("A-", "");
        let areas = area.split(" + ").collect::<Vec<_>>();

        let dir = &format!(
            "{dir}/{}",
            areas
                .first()
                .context("There should always be at least one area")?
        );
        std::fs::create_dir_all(dir).context(format!("Failed to create {dir}"))?;

        let mut prs = prs;
        prs.sort_by_key(|k| k.1.closed_at);

        for (title, pr) in prs {
            println!("# {title}");

            let fs_friendly_title = title
                .replace(' ', "_")
                .replace(|c: char| !c.is_alphanumeric() && c != '_', "");

            // PR number needs to be first so sort remains consistent.
            // This is fine because github PR numbers are monotonic
            let mut filename = format!("{}_{}", pr.number, fs_friendly_title);

            // Shorten the filename because we don't want really long file names
            // Some OS still have file path length limits in 2024...
            // 64 is completely arbitrary but felt long enough and is a nice power of 2
            filename.truncate(64);

            let file_path = &format!("{dir}/{filename}.md");

            // TODO this should probably return if file already exists, so we don't overwrite changes
            // Maybe add a flag for this because overwriting is useful while developing this tool
            let mut file = std::fs::File::create(file_path)
                .context(format!("Failed to create {file_path}"))?;

            // Write title for the PR with correct heading and github url
            writeln!(
                &mut file,
                "\n### [{}](https://github.com/bevyengine/bevy/pull/{})",
                title, pr.number
            )?;
            // Write custom HTML to show area tag on each section
            write!(&mut file, "\n<div class=\"migration-guide-area-tags\">")?;
            for area in &areas {
                write!(
                    &mut file,
                    "\n    <div class=\"migration-guide-area-tag\">{area}</div>"
                )?;
            }
            write!(&mut file, "\n</div>")?;
            writeln!(&mut file)?;

            let (section, _) = write_markdown_section(
                pr.body.as_ref().context("PR has no body")?,
                "migration guide",
                true,
            )?;
            write!(file, "{}", section)?;
        }
    }
    writeln!(&mut output, "</div>")?;

    println!("\nFound {} breaking PRs merged", count);

    std::fs::write(path, output)?;

    Ok(())
}
