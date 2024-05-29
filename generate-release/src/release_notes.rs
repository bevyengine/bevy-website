use anyhow::Context;

use crate::{
    github_client::{GithubClient, GithubIssuesResponse, IssueState},
    helpers::{get_contributors, get_merged_prs},
};
use std::{
    collections::HashSet,
    io::Write as IoWrite,
    path::{Path, PathBuf},
};

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
    // This will generally be something like 0.5 or 0.13
    let current_release = from;
    let release_parts: Vec<&str> = current_release.split('.').collect();
    // TODO: this will need to change when we hit 1.0
    let next_release = format!(
        "{}.{}",
        release_parts[0],
        release_parts[1].parse::<i32>().unwrap() + 1
    );
    let milestone = format!("Release {}", next_release);

    // Get all PRs that need release notes
    let prs = get_merged_prs(client, from, to, Some("C-Needs-Release-Note"))?;

    // Create the directory that will contain all the release notes
    std::fs::create_dir_all(&path).context(format!("Failed to create {path:?}"))?;

    // We'll write the file once at the end when all the metdaata is generated
    let mut notes_metadata = Vec::new();

    // Generate the list of all issues so we don't spam the repo with duplicates
    // This is done outside of the loop because we don't want to request this information anew for every PR
    println!("Getting list of the issues from the `bevy-website` repo to check for duplicates.");
    let open_issues = client.get_issues_and_prs("bevy-website", IssueState::All, None, None)?;
    let issue_titles = open_issues
        .iter()
        .map(|issue| issue.title.clone())
        .collect::<HashSet<_>>();
    println!("Found {} issues", issue_titles.len());

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
        generate_and_open_issue(
            client,
            &issue_titles,
            &pr,
            &title,
            &authors,
            &file_path,
            &milestone,
            dry_run,
        );
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

#[allow(clippy::too_many_arguments)]
fn generate_and_open_issue(
    client: &GithubClient,
    issue_titles: &HashSet<String>,
    pr: &GithubIssuesResponse,
    title: &str,
    authors: &[String],
    file_path: &Path,
    milestone: &str,
    dry_run: bool,
) {
    let pr_number = pr.number;
    let file_path = file_path.to_string_lossy();
    let issue_title = format!("Write release notes for PR #{pr_number}: {title}");

    // Check if this issue already exists
    // If it does, we don't want to spam the repo with duplicate issues
    if issue_titles.contains(&issue_title) {
        return;
    }

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

    let labels = vec!["A-Release-Notes", "C-Content", "S-Ready-For-Implementation"];

    if dry_run {
        println!("Would open issue on GitHub with the title and body:");
        println!("Title: {}", issue_title);
        println!("Body: {}", issue_body);
        println!("Milestone: {}", milestone);
        println!("Labels: {:?}", labels);
    } else {
        client
            .open_issue(&issue_title, &issue_body, milestone, labels)
            .unwrap();
        println!("Opened issue for PR #{}: {}", pr_number, title);
    }
}
