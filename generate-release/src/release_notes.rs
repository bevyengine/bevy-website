use anyhow::Context;

use crate::{
    github_client::GithubClient,
    helpers::{get_contributors, get_merged_prs},
};
use std::{collections::HashSet, io::Write as IoWrite, path::PathBuf};

pub fn generate_release_notes(
    from: &str,
    to: &str,
    path: PathBuf,
    client: &GithubClient,
    overwrite_existing: bool,
) -> anyhow::Result<()> {
    // Get all PRs that need release notes
    let prs = get_merged_prs(client, from, to, Some("C-Needs-Release-Note"))?;

    // Create the directory that will contain all the release notes
    std::fs::create_dir_all(&path).context(format!("Failed to create {path:?}"))?;

    // We'll write the file once at the end when all the metdaata is generated
    let mut notes_metadata = Vec::new();

    for (pr, commit, title) in prs {
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

        // get the list of contributors to this PR
        let mut contributors = HashSet::new();
        let author = format!("@{}", pr.user.login);

        if let Ok(pr_contributors) = get_contributors(client, &commit, &pr) {
            for c in pr_contributors {
                contributors.insert(c);
            }
        }

        contributors.remove(&author);

        let mut authors = vec![author];
        authors.extend(contributors);

        notes_metadata.push(generate_metadata_block(
            &title, &authors, pr.number, &file_name,
        ));

        let file_path = path.join(format!("{file_name}.md"));
        if file_path.exists() && !overwrite_existing {
            // Skip existing files because we don't want to overwrite changes when regenerating
            continue;
        }

        let file =
            std::fs::File::create(&file_path).context(format!("Failed to create {file_path:?}"))?;

        writeln!(&file, "TODO")?;
    }

    // Write the metadata file
    let mut notes_toml = std::fs::File::create(path.join("_release-notes.toml"))
        .context("Failed to create _guides.toml")?;
    for metadata in notes_metadata {
        writeln!(&mut notes_toml, "{metadata}")?;
    }

    Ok(())
}

fn generate_metadata_block(
    title: &str,
    authors: &[String],
    pr_number: i32,
    file_name: &str,
) -> String {
    // TODO should probably add some weight for sorting
    format!(
        r#"[[release_notes]]
title = "{title}"
authors = [{authors}]
url = "https://github.com/bevyengine/bevy/pull/{pr_number}"
file_name = "{file_name}.md"
"#,
        authors = authors
            .iter()
            .map(|author| format!("\"{author}\""))
            .collect::<Vec<_>>()
            .join(","),
        title = title.trim().replace('"', "\\\"")
    )
}
