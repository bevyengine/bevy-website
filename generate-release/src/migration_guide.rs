use anyhow::Context;

use crate::{
    github_client::{GithubClient, GithubIssuesResponse},
    helpers::{get_merged_prs, get_pr_area},
    markdown::write_markdown_section,
};
use std::{collections::BTreeMap, fmt::Write as _, io::Write as IoWrite, path::PathBuf};

pub fn generate_migration_guide(
    from: &str,
    to: &str,
    path: PathBuf,
    client: &mut GithubClient,
    overwrite_existing: bool,
) -> anyhow::Result<()> {
    // Get all PR by area
    let mut areas = BTreeMap::<String, Vec<(String, GithubIssuesResponse)>>::new();
    {
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
        println!("\nFound {} breaking PRs merged", count);
    }

    std::fs::create_dir_all(&path).context(format!("Failed to create {path:?}"))?;

    let mut metadata = String::new();

    // Write all the separate migration guide files
    for (area, prs) in areas {
        let area = area.replace("A-", "");
        let areas = area.split(" + ").collect::<Vec<_>>();

        // let path = path.join(
        //     areas
        //         .first()
        //         .context("There should always be at least one area")?,
        // );
        std::fs::create_dir_all(&path).context(format!("Failed to create {path:?}"))?;

        let mut prs = prs;
        prs.sort_by_key(|k| k.1.closed_at);

        for (title, pr) in prs {
            let fs_friendly_title = title
                .replace(' ', "_")
                .replace(|c: char| !c.is_alphanumeric() && c != '_', "");

            // PR number needs to be first so sorting by name remains consistent.
            // This works because github's pr numbers are monotonic
            let mut file_name = format!("{}_{}", pr.number, fs_friendly_title);

            // Shorten the filename because we don't want really long file names
            // Some OS still have file path length limits in 2024...
            // 64 is completely arbitrary but felt long enough and is a nice power of 2
            file_name.truncate(64);

            // Generate the metadata for this file
            writeln!(
                &mut metadata,
                r#"[[guides]]
title = "{title}"
url = "https://github.com/bevyengine/bevy/pull/{pr_number}"
areas = [{areas}]
file_name = "{file_name}.md"
"#,
                areas = areas
                    .iter()
                    .map(|area| format!("\"{area}\""))
                    .collect::<Vec<_>>()
                    .join(","),
                pr_number = pr.number,
            )?;

            let file_path = path.join(format!("{file_name}.md"));
            if file_path.exists() && !overwrite_existing {
                // skip existing files because we don't want to overwrite changes when regenerating
                continue;
            }

            let mut file = std::fs::File::create(&file_path)
                .context(format!("Failed to create {file_path:?}"))?;

            let (section, _) = write_markdown_section(
                pr.body.as_ref().context("PR has no body")?,
                "migration guide",
                true,
            )?;
            let mut section = section;
            // some guide have a rule at the end so remove it
            if section.ends_with("\n---\n") {
                section = section.replace("\n---\n", "");
            }
            write!(file, "{}", section)?;
        }
    }

    std::fs::write(path.join("_guides.toml"), metadata)?;

    Ok(())
}
