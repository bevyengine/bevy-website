use crate::github_client::{GithubClient, GithubCommitResponse, GithubIssuesResponse};
use anyhow::Context;
use regex::Regex;

pub fn get_merged_prs(
    client: &GithubClient,
    since: &str,
    sha: &str,
    label: Option<&str>,
) -> anyhow::Result<Vec<(GithubIssuesResponse, GithubCommitResponse, String)>> {
    println!("Getting list of all commits since: {since}");
    // We use the list of commits to make sure the PRs are only on main
    let commits = client
        .get_commits(since, sha)
        .context("Failed to get commits for branch")?;
    println!("Found {} commits", commits.len());

    println!("Getting list of all merged PRs since {since} with label {label:?}");
    // We also get the list of merged PRs in batches instead of getting them separately for each commit
    let prs = client.get_merged_prs(since, label)?;
    println!("Found {} merged PRs", prs.len());

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

    // Get the pr number added by bors at the end of the title
    let re = Regex::new(r"\(#([\d]*)\)").unwrap();
    let Some(cap) = re.captures_iter(title).last() else {
        // This means there wasn't a PR associated with the commit
        // Or bors didn't add a pr number
        return None;
    };
    // remove PR number from title
    let title = title.replace(&cap[0].to_string(), "");
    let title = title.trim_end();
    Some(title.to_string())
}
