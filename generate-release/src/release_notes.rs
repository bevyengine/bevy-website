use anyhow::Context;

use crate::{
    github_client::{GithubClient, GithubIssuesResponse},
    helpers::{get_contributors, get_merged_prs},
};
use std::{collections::HashSet, io::Write as IoWrite, path::PathBuf};

pub fn generate_release_notes(
    from: &str,
    to: &str,
    path: PathBuf,
    client: &GithubClient,
    overwrite_existing: bool,
    // If this value is true, no issues will be opened.
    // This is useful for testing the release notes generation without spamming the repo.
    dry_run: bool,
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

        // Make sure the author is always the first in the list
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

        writeln!(&file, "<!-- {} -->", title)?;
        writeln!(
            &file,
            "<!-- https://github.com/bevyengine/bevy/pull/{} -->",
            pr.number
        )?;
        writeln!(&file, "\n<!-- TODO -->")?;

        // Open an issue to remind the author(s) to write the release notes
        generate_and_open_issue(client, &pr, &title, &authors, &file_path, dry_run);
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

fn generate_and_open_issue(
    client: &GithubClient,
    pr: &GithubIssuesResponse,
    title: &str,
    authors: &[String],
    file_path: &PathBuf,
    dry_run: bool,
) {
    let pr_number = pr.number;
    let file_path = file_path.to_string_lossy();

    let issue_title = format!("Write release notes for PR #{pr_number}: {title}");

    let authors = authors
        .iter()
        .map(|author| format!("@{}", author))
        .collect::<Vec<_>>()
        .join(", ");

    let issue_body = format!(
        "This PR needs release notes for the upcoming Bevy release!
        Please reply below if you'd like to volunteer do so, whether or not you're the author of the PR.
        
        Release notes should:
        
        1. Clearly motivate the change.
        2. Be written in a way that is understandable by the average Bevy user: some programming background and a general understanding of games.
        3. Show off the coolest features of the PR. Screenshots are awesome, but elegant APIs are also welcome!
        4. If this was a perf-centric PR, quantify the performance improvements. Graphs and statistics work well for this.

        We can help you revise the release notes: a rough draft alone is incredibly useful :)
        Your expertise is invaluable for contextualizing the changes; we'll work with you to bring the technical writing up to par.

        To submit your release notes, modify `{file_path}.md` and submit a PR.
        In that PR, please mention this issue with the `Fixes #{pr_number}` keyphrase so it gets closed automatically.

        Pinging: {authors}
    )");

    if dry_run {
        println!("Would open issue on GitHub with the title and body:");
        println!("Title: {}", issue_title);
        println!("Body: {}", issue_body);
    } else {
        todo!("Open issue on GitHub with the title and body");
    }
}
