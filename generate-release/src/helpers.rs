use crate::github_client::{GithubClient, GithubCommitResponse, GithubIssuesResponse};
use anyhow::{bail, Context};
use regex::Regex;

pub fn get_merged_prs(
    client: &GithubClient,
    from: &str,
    to: &str,
    label: Option<&str>,
) -> anyhow::Result<Vec<(GithubIssuesResponse, GithubCommitResponse, String)>> {
    println!("Getting list of all commits from {from} to {to}");
    // We use the list of commits to make sure the PRs are only on main
    let commits = client
        .compare_commits(from, to)
        .context("Failed to get commits")?;
    println!("Found {} commits", commits.len());

    println!("Getting list of all merged PRs from {from} to {to} with label {label:?}");

    let base_commit = client.get_commit(from)?;
    let base_commit_date = &base_commit.commit.committer.date[0..10];

    // We also get the list of merged PRs in batches instead of getting them separately for each commit
    let prs = client.get_merged_prs(base_commit_date, label)?;
    println!(
        "Found {} merged PRs since {} (the base commit date)",
        prs.len(),
        base_commit_date
    );

    let mut out = vec![];
    for commit in &commits {
        let Some(title) = get_pr_title_from_commit(commit)else {
            continue;
        };

        // Get the PR associated with the commit based on it's title
        let Some(pr) = prs.iter().find(|pr| pr.title.contains(&title)) else {
            // If there's no label, then not finding a PR is an issue because this means we want all PRs
            // If there's a label then it just means the commit is not a PR with the label
            if label.is_none() {
                println!("\x1b[93mPR not found for {title} sha: {}\x1b[0m", commit.sha);
            }
            continue;
        };
        out.push((pr.clone(), commit.clone(), title));
    }

    Ok(out)
}

fn get_pr_title_from_commit(commit: &GithubCommitResponse) -> Option<String> {
    let mut message_lines = commit.commit.message.lines();

    // Title is always the first line of a commit message
    let title = message_lines.next().expect("Commit message empty");

    // Get the pr number at the end of the title
    let re = Regex::new(r"\(#([\d]*)\)").unwrap();
    let Some(cap) = re.captures_iter(title).last() else {
        // This means there wasn't a PR associated with the commit
        return None;
    };
    // remove PR number from title
    let title = title.replace(&cap[0].to_string(), "");
    let title = title.trim_end();
    Some(title.to_string())
}

pub fn get_pr_area(pr: &GithubIssuesResponse) -> String {
    let areas: Vec<String> = pr
        .labels
        .iter()
        .map(|l| l.name.clone())
        .filter(|l| l.starts_with("A-"))
        .collect();
    if areas.is_empty() {
        String::from("No area label")
    } else {
        areas.join(" + ")
    }
}

pub fn get_contributors(
    client: &mut GithubClient,
    commit: &GithubCommitResponse,
    pr: &GithubIssuesResponse,
) -> anyhow::Result<Vec<String>> {
    // Find authors and co-authors
    // TODO this could probably be done with multiple threads to speed it up
    match client.get_contributors(&commit.sha) {
        Ok(logins) => {
            if logins.is_empty() {
                bail!(
                    "\x1b[93mNo contributors found for https://github.com/bevyengine/{}/pull/{} sha: {}\x1b[0m",
                    client.repo,
                    pr.number,
                    commit.sha
                );
            }
            Ok(logins)
        }
        Err(err) => {
            bail!(
                "\x1b[93m{err:?}\nhttps://github.com/bevyengine/bevy/pull/{}\n{}\x1b[0m",
                pr.number,
                commit.sha
            );
        }
    }
}
