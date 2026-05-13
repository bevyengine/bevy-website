use anyhow::Context;
use serde::Deserialize;

use crate::{
    github_client::{BevyRepo, GithubClient, GithubIssuesResponse, IssueState},
    helpers::{get_contributors, get_merged_prs},
};
use std::{
    collections::HashSet,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Clone)]
struct ReleaseNotes {
    release_notes: Vec<ReleaseNote>,
}

#[expect(dead_code)]
#[derive(Deserialize, Clone)]
struct ReleaseNote {
    title: String,
    authors: Vec<String>,
    contributors: Vec<String>,
    prs: Vec<u64>,
    file_name: String,
}

pub fn generate_release_notes(
    from: &str,
    to: &str,
    path: PathBuf,
    client: &GithubClient,
    overwrite_existing: bool,
    // If this value is false, no issues will be opened.
    create_issues: bool,
) -> anyhow::Result<()> {
    // Get all PRs that need release notes
    let prs = get_merged_prs(client, from, to, Some("M-Needs-Release-Note"))?;

    // Create the directory that will contain all the release notes
    std::fs::create_dir_all(&path).context(format!("Failed to create {path:?}"))?;

    // We'll write the file once at the end when all the metadata is generated
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

    // If there is metadata that already exists,
    // and would contain info such as which PR already
    // has an entry, then get it and use it for that.
    let preexisting_metadata_file = fs::read_to_string(path.join("_release-notes.toml")).ok();
    // Deserializes the file inside the option into the `ReleaseNotes` struct,
    // and then transposes / swaps the internal result of that operation to external,
    // and returns the error of that result if there is one,
    // else we have our preexisting metadata, ready to use.
    let preexisting_metadata: Option<ReleaseNotes> = preexisting_metadata_file
        .as_deref()
        .map(toml::from_str)
        .transpose()?;

    eprintln!("metadata exists? {}", preexisting_metadata.is_some());

    let mut new_prs = false;

    for (pr, commit, title) in prs {
        // If a PR is already included in the release notes,
        // then do not generate anything for this PR.
        //
        // If overwrite_existing is true, then ignore
        // if the PRs may have already been generated.
        if preexisting_metadata.is_some() && !overwrite_existing {
            let preexisting_metadata = preexisting_metadata
                .clone()
                .expect("that preexisting metadata existed at the _release_notes.toml for this release version");
            let mut pr_already_generated = false;

            for release_note in preexisting_metadata.release_notes {
                if release_note.prs.contains(&pr.number) {
                    pr_already_generated = true;
                }
            }

            if pr_already_generated {
                eprintln!("PR #{} already exists", pr.number);
                continue;
            }
        }

        // If the code has reached this point then that means
        // there is new PRs to be recorded.
        new_prs = true;

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

        // Separate the contributors from authors for manual
        // filtering since contributors may include typo fixes
        // and other minor changes unwanted for the author position.
        let contributors: Vec<String> = contributors.into_iter().collect();

        notes_metadata.push(generate_metadata_block(
            &title,
            &author,
            &contributors,
            pr.number,
            &file_name,
        ));

        let file_path = path.join(format!("{file_name}.md"));

        let file =
            std::fs::File::create(&file_path).context(format!("Failed to create {file_path:?}"))?;

        writeln!(&file, "<!-- {title} -->")?;
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
            &file_path,
            create_issues,
        );
    }

    if !create_issues {
        println!(
            "No issues were created. If you would like to do so, add the `--create-issues` flag."
        );
    }

    eprintln!("new prs? {new_prs}");

    // Early return if there is no new PRs
    // to append to the metadata file.
    if !new_prs {
        return Ok(());
    }

    let mut notes_toml = if overwrite_existing {
        // Replace and overwrite file.
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path.join("_release-notes.toml"))
            .context("Failed to create _release-notes.toml")?
    } else {
        // Append to the metadata file,
        // creating it if necessary.
        OpenOptions::new()
            .append(true)
            .create(true)
            .open(path.join("_release-notes.toml"))
            .context("Failed to create _release-notes.toml")?
    };
    for metadata in notes_metadata {
        writeln!(&mut notes_toml, "{metadata}")?;
    }

    Ok(())
}

fn generate_metadata_block(
    title: &str,
    author: &str,
    contributors: &[String],
    pr_number: u64,
    file_name: &str,
) -> String {
    // TODO should probably add some weight for sorting
    format!(
        r#"[[release_notes]]
title = "{title}"
authors = ["{author}",]
contributors = [{contributors}]
prs = [{pr_number}]
file_name = "{file_name}.md"
"#,
        contributors = contributors
            .iter()
            .map(|contributor| format!("\"{contributor}\""))
            .collect::<Vec<_>>()
            .join(", "),
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
    create_issues: bool,
) {
    let pr_number = pr.number;
    let issue_title = format!("Write release notes for PR #{pr_number}: {title}");

    // Check if this issue already exists
    // If it does, we don't want to spam the repo with duplicate issues
    if existing_issue_titles.contains(&issue_title) {
        println!("Issue already exists for PR #{pr_number}: {title}");
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

    if !create_issues {
        println!("Would open issue on GitHub:");
        println!("Title: {issue_title}");
        println!("Body: {issue_body}");
        println!("Labels: {labels:?}\n\n");
    } else {
        // Open an issue on the `bevy-website` repo
        let response = client
            .open_issue(BevyRepo::BevyWebsite, &issue_title, &issue_body, labels)
            .unwrap_or_else(|err| {
                eprintln!("Failed to open issue for PR #{pr_number}: {title}");
                eprintln!("Error: {err:?}");
                std::process::exit(1);
            });
        println!("Opened issue for PR #{pr_number}: {title}");
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
                eprintln!("Failed to leave a comment on PR #{pr_number}: {title}");
                eprintln!("Error: {err:?}");
                std::process::exit(1);
            });
    }
}
