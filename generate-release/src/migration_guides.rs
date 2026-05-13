use anyhow::Context;
use serde::Deserialize;

use crate::{
    github_client::{GithubClient, GithubIssuesResponse},
    helpers::{get_merged_prs, get_pr_area},
    markdown::write_markdown_section,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, OpenOptions},
    io::Write as IoWrite,
    path::PathBuf,
};

type PrsByAreaBTreeMap = BTreeMap<Vec<String>, Vec<(String, GithubIssuesResponse)>>;

#[derive(Deserialize, Clone)]
struct MigrationGuides {
    guides: Vec<MigrationGuide>,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
struct MigrationGuide {
    title: String,
    prs: Vec<u64>,
    areas: BTreeSet<String>,
    file_name: String,
}

pub fn generate_migration_guides(
    from: &str,
    to: &str,
    path: PathBuf,
    client: &GithubClient,
    overwrite_existing: bool,
) -> anyhow::Result<()> {
    // Get all PRs by area
    let areas = get_prs_by_areas(client, from, to)?;

    // Create the directory that will contain all the migration guides
    std::fs::create_dir_all(&path).context(format!("Failed to create {path:?}"))?;

    let mut guides_metadata = if overwrite_existing {
        vec![]
    } else {
        // If there is metadata that already exists,
        // and would contain info such as which PR already
        // has an entry, then get it and use it for that.
        let preexisting_metadata_file = fs::read_to_string(path.join("_guides.toml")).ok();
        // Deserializes the file inside the option into the `MigrationGuides` struct,
        // and then transposes / swaps the internal result of that operation to external,
        // and returns the error of that result if there is one,
        // else we have our preexisting metadata, ready to use.
        let preexisting_metadata: Option<MigrationGuides> = preexisting_metadata_file
            .as_deref()
            .map(toml::from_str)
            .transpose()?;

        eprintln!("metadata exists? {}", preexisting_metadata.is_some());
        // Populate the metadata to be written with the
        // preexisting metadata so that it is not lost,
        // or overwritten.
        preexisting_metadata
            .map(|metadata| metadata.guides)
            .unwrap_or_default()
    };

    // Write all the separate migration guide files
    for (area, prs) in areas {
        for (title, pr) in prs {
            // If a PR is already included in the migration guides,
            // then do not generate anything for this PR.
            let mut pr_already_generated = false;

            for migration_guide in guides_metadata.iter() {
                if migration_guide.prs.contains(&pr.number) {
                    pr_already_generated = true;
                }
            }

            if pr_already_generated {
                eprintln!("PR #{} already exists", pr.number);
                continue;
            }

            // Slugify the title
            let title_slug = title
                .replace(' ', "_")
                .replace(|c: char| !c.is_alphanumeric() && c != '_', "");

            // PR number needs to be first so sorting by name remains consistent.
            // This works because github's pr numbers are monotonic
            let mut file_name = format!("{}_{}", pr.number, title_slug);

            // Shorten the filename because we don't want really long file names
            // Some OS still have file path length limits in 2024...
            // 64 is completely arbitrary but felt long enough and is a nice power of 2
            file_name.truncate(64);

            // Add the markdown extension
            file_name = format!("{file_name}.md");

            let file_path = path.join(&file_name);

            if write_migration_file(
                &file_path,
                pr.body.as_ref().context("PR has no body")?,
                pr.number,
            )? {
                guides_metadata.push(MigrationGuide {
                    title,
                    prs: vec![pr.number],
                    areas: area.clone().into_iter().collect(),
                    file_name,
                });
            }
        }
    }

    // Sort by: Area in ascending order (empty areas at the end), and Title in ascending order
    guides_metadata.sort_by(|a, b| {
        let areas_cmp = match (a.areas.is_empty(), b.areas.is_empty()) {
            (false, false) => {
                let a_areas = a.areas.clone().into_iter().collect::<Vec<_>>().join(" ");
                let b_areas = b.areas.clone().into_iter().collect::<Vec<_>>().join(" ");

                a_areas.cmp(&b_areas)
            }
            (false, true) => std::cmp::Ordering::Less,
            (true, false) => std::cmp::Ordering::Greater,
            (true, true) => std::cmp::Ordering::Equal,
        };

        areas_cmp.then_with(|| a.title.cmp(&b.title))
    });

    // Create the metadata file, and overwrite it if it already exists.
    //
    // Note:
    // The file, while overwritten,
    // may still contain the same underlying data gotten from
    // the preexisting metadata earlier, if overwrite_existing is false,
    // thus preserving the data even if the file itself is overwritten.
    let mut guides_toml = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path.join("_guides.toml"))
        .context("Failed to create _guides.toml")?;

    for metadata in guides_metadata {
        // Generate the metadata block for this migration.
        //
        // We always freshly generate and write this data to the file,
        // rather than appending to the end-of-file,
        // so that we can maintain proper ordering of the entries.
        let metadata_block = generate_metadata_block(
            &metadata.title,
            &metadata.file_name,
            &metadata.areas.into_iter().collect::<Vec<_>>(),
            &metadata.prs,
        );

        writeln!(&mut guides_toml, "{metadata_block}")?;
    }

    Ok(())
}

/// Gets all PRs that have either a migration guide section or the Breaking-Change label
/// The PRs are stored in a [`BTreeMap`] where the key is the list of area labels of the PR
fn get_prs_by_areas(
    client: &GithubClient,
    from: &str,
    to: &str,
) -> Result<PrsByAreaBTreeMap, anyhow::Error> {
    let mut areas: PrsByAreaBTreeMap = BTreeMap::new();

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
            .any(|l| l.name.contains("M-Needs-Migration-Guide"));

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
    println!("\nFound {count} breaking PRs merged");

    Ok(areas)
}

/// Generates the metadata markdown section for the given values
fn generate_metadata_block(
    title: &str,
    file_name: &String,
    areas: &[String],
    pr_number: &[u64],
) -> String {
    format!(
        r#"[[guides]]
title = "{title}"
prs = [{pr_numbers}]
areas = [{areas}]
file_name = "{file_name}"
"#,
        pr_numbers = pr_number
            .iter()
            .map(|pr| pr.to_string())
            .collect::<Vec<_>>()
            .join(", "),
        areas = areas
            .iter()
            .map(|area| format!("\"{area}\""))
            .collect::<Vec<_>>()
            .join(", "),
        title = title.trim().replace('"', "\\\"")
    )
}

/// Write a file containing the body of the migration guide
///
/// Also does some clean ups like removing unnecessary characters
fn write_migration_file(
    file_path: &PathBuf,
    pr_body: &str,
    pr_number: u64,
) -> anyhow::Result<bool> {
    let mut file =
        std::fs::File::create(file_path).context(format!("Failed to create {file_path:?}"))?;

    let (section, _) = write_markdown_section(pr_body, "migration guide", true)?;
    let mut section = section;
    // some guides have a rule at the end so remove it
    if section.ends_with("\n---\n") {
        section = section.replace("\n---\n", "");
    }

    // Strip leading and trailing whitespace and add a newline at the end
    section = section.trim().to_string() + "\n";

    if section.trim().is_empty() {
        // Section is just whitespace so we can skip it
        // It's possible this is a false positive so log the url to check it manually
        println!("\x1b[93mMigration guide is empty for https://github.com/bevyengine/bevy/pull/{pr_number}\x1b[0m");
        return Ok(false);
    }

    write!(file, "{section}")?;
    Ok(true)
}
