use std::time::Duration;

use crate::github_client::{
    BevyRepo, GithubClient, GithubCommitResponse, GithubIssuesResponse, IssueState,
};
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

    let base_commit = client.get_commit(from, BevyRepo::Bevy)?;
    let base_commit_date = &base_commit.commit.committer.date[0..10];

    // We also get the list of merged PRs in batches instead of getting them separately for each commit
    let prs = client.get_issues_and_prs(
        BevyRepo::Bevy,
        IssueState::Merged,
        Some(base_commit_date),
        label,
    )?;
    println!(
        "Found {} merged PRs and {} commits since {} (the base commit date)",
        prs.len(),
        commits.len(),
        base_commit_date
    );

    let mut out = vec![];
    for commit in &commits {
        let Some(title) = get_pr_title_from_commit(commit) else {
            continue;
        };

        // Get the PR associated with the commit based on it's title
        let Some(pr) = prs.iter().find(|pr| pr.title.contains(&title)) else {
            // If there's no label, then not finding a PR is an issue because this means we want all PRs
            // If there's a label then it just means the commit is not a PR with the label
            if label.is_none() {
                println!(
                    "\x1b[93mPR not found for {title} sha: {}\x1b[0m",
                    commit.sha
                );
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

/// Returns all the area label for a PR as a list separated with ' + '
pub fn get_pr_area(pr: &GithubIssuesResponse) -> Vec<String> {
    let mut areas: Vec<String> = pr
        .labels
        .iter()
        .map(|l| l.name.clone())
        .filter(|l| l.starts_with("A-"))
        .map(|l| l.replace("A-", ""))
        .collect();

    areas.sort_by_key(|a| a.to_lowercase());

    areas
}

/// Gets a list of all authors and co-authors for the given commit
/// Will retry the query automatically a few times
pub fn get_contributors(
    client: &GithubClient,
    commit: &GithubCommitResponse,
    pr: &GithubIssuesResponse,
) -> anyhow::Result<Vec<String>> {
    let get_contributors_internal = || match client.get_contributors(&commit.sha) {
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
        err => err,
    };

    let mut retry_count = 0;
    match get_contributors_internal() {
        Ok(logins) => Ok(logins),
        Err(err) => {
            while retry_count < 20 {
                println!("\x1b[93mFailed to get contributors waiting and retrying: {err:?}\x1b[0m",);
                std::thread::sleep(Duration::from_secs(2));
                match get_contributors_internal() {
                    Ok(logins) => return Ok(logins),
                    Err(_) => retry_count += 1,
                }
            }
            bail!("Too many retries");
        }
    }
}
