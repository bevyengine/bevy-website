use anyhow::Context;

use crate::{
    github_client::{BevyRepo, GithubClient, GithubIssuesResponse, IssueState},
    helpers::{get_contributors, get_merged_prs},
};
use std::{
    collections::HashSet,
    io::Write,
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
    local: bool,
) -> anyhow::Result<()> {
    // Get all PRs that need release notes
    let prs = get_merged_prs(client, from, to, Some("C-Needs-Release-Note"))?;

    // Create the directory that will contain all the release notes
    std::fs::create_dir_all(&path).context(format!("Failed to create {path:?}"))?;

    // We'll write the file once at the end when all the metdaata is generated
    let mut notes_metadata = Vec::new();

    // Generate the list of all issues so we don't spam the repo with duplicates
    // This is done outside of the loop because we don't want to request this information anew for every PR
    println!("Getting list of the issues from the `bevy-website` repo to check for duplicates.");
    let issue_titles = client
        .get_issues_and_prs(BevyRepo::BevyWebsite, IssueState::All, None, None)?
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
        generate_and_open_issue(client, &issue_titles, &pr, &title, &file_path, local);
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
    pr_number: u64,
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

/// This function:
///
/// 1. Generates a new issue on the `bevy-website` repo for the given PR.
/// 2. Leaves a comment in the original PR linking to the new issue.
///
/// If the issue already exists, no action is taken.
#[allow(clippy::too_many_arguments)]
fn generate_and_open_issue(
    client: &GithubClient,
    existing_issue_titles: &HashSet<String>,
    pr: &GithubIssuesResponse,
    title: &str,
    file_path: &Path,
    local: bool,
) {
    let pr_number = pr.number;
    let issue_title = format!("Write release notes for PR #{pr_number}: {title}");

    // Check if this issue already exists
    // If it does, we don't want to spam the repo with duplicate issues
    if existing_issue_titles.contains(&issue_title) {
        println!("Issue already exists for PR #{}: {}", pr_number, title);
        return;
    }

    let pr_url = format!("https://github.com/bevyengine/bevy/pull/{pr_number}",);
    let file_path = file_path.to_string_lossy();

    // The weird indentation is intentional.
    // Otherwise, the text is not formatted correctly on GitHub as the tabs are copied into the issue body.
    let issue_body = format!(
        "{pr_url} needs release notes for the upcoming Bevy release!

Please reply below if you'd like to write these notes. 
While the author(s) of the PR often have the context, knowledge and motivation to draft the release notes for their feature, anyone can contribute release notes!

------

Release notes should:

1. Clearly motivate the change.
2. Be written in a way that is understandable by the average Bevy user: some programming background and a general understanding of games.
3. Show off the coolest features of the PR. Screenshots are awesome, but elegant APIs are also welcome!
4. If this was a perf-centric PR, quantify the performance improvements. Graphs and statistics work well for this.

We can help you revise the release notes: a rough draft alone is incredibly useful :)
Your expertise is invaluable for contextualizing the changes; we'll work with you to bring the technical writing up to par.

To submit your release notes, modify `{file_path}` and submit a PR.
In that PR, please mention this issue with the `Fixes #ISSUE_NUMBER` keyphrase so it gets closed automatically.");

    let labels = vec!["A-Release-Notes", "C-Content", "S-Ready-For-Implementation"];

    if local {
        println!("Would open issue on GitHub:");
        println!("Title: {}", issue_title);
        println!("Body: {}", issue_body);
        println!("Labels: {:?}", labels);
    } else {
        // Open an issue on the `bevy-website` repo
        let response = client
            .open_issue(BevyRepo::BevyWebsite, &issue_title, &issue_body, labels)
            .unwrap_or_else(|err| {
                eprintln!("Failed to open issue for PR #{}: {}", pr_number, title);
                eprintln!("Error: {:?}", err);
                std::process::exit(1);
            });
        println!("Opened issue for PR #{}: {}", pr_number, title);
        // Pause between opening issues to avoid getting rate-limited.
        // See https://docs.github.com/en/rest/using-the-rest-api/best-practices-for-using-the-rest-api?apiVersion=2022-11-28#pause-between-mutative-requests
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Leave a comment on the PR linking to the new issue
        let issue_url = response.html_url;

        let comment = format!("Thank you to everyone involved with the authoring or reviewing of this PR! This work is relatively important and needs release notes! Head over to {issue_url} if you'd like to help out.",);

        // Warn the user if the comment fails
        client
            .leave_comment(BevyRepo::Bevy, pr_number, &comment)
            .unwrap_or_else(|err| {
                eprintln!("Failed to leave a comment on PR #{}: {}", pr_number, title);
                eprintln!("Error: {:?}", err);
                std::process::exit(1);
            });
    }
}
