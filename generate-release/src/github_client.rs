use std::{collections::HashMap, fmt::Debug};

use anyhow::bail;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use serde::Deserialize;
use ureq::Response;

#[derive(Deserialize, Clone, Debug)]
pub struct GithubBranchesResponse {
    pub name: String,
    pub commit: GithubBranchesCommitResponse,
}
#[derive(Deserialize, Clone, Debug)]
pub struct GithubBranchesCommitResponse {
    pub sha: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GithubCommitResponse {
    pub sha: String,
    pub commit: GithubCommitContent,
    pub author: Option<GithubUser>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Committer {
    pub name: String,
    pub date: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GithubCommitContent {
    // First line is the title
    // If multiple authors, it will add "Co-Authored by: <author>" at the end
    pub message: String,
    pub committer: Committer,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GithubAuthor {
    pub name: String,
    pub user: Option<GithubUser>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GithubUser {
    pub login: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GithubCommitBranchResponse {
    pub name: String,
    pub commit: GithubCommitBranchCommitResponse,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GithubCommitBranchCommitResponse {
    pub sha: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GithubPullRequestResponse {
    pub title: String,
    pub number: i32,
    pub body: Option<String>,
    pub labels: Vec<GithubLabel>,
    pub user: GithubUser,
    pub closed_at: DateTime<Utc>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GithubCompareResponse {
    pub base_commit: GithubCommitResponse,
    pub commits: Vec<GithubCommitResponse>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GithubLabel {
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GithubUserSearchResponse {
    pub items: Vec<GithubUser>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GithubIssuesResponsePullRequest {
    pub merged_at: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GithubIssuesResponse {
    pub title: String,
    pub number: i32,
    pub body: Option<String>,
    pub labels: Vec<GithubLabel>,
    pub user: GithubUser,
    pub closed_at: DateTime<Utc>,
    pub pull_request: Option<GithubIssuesResponsePullRequest>,
}

pub struct GithubClient {
    agent: ureq::Agent,
    token: String,
    pub repo: String,
}

impl GithubClient {
    pub fn new(token: String, repo: String) -> Self {
        let agent: ureq::Agent = ureq::AgentBuilder::new()
            .user_agent("bevy-website-generate-release")
            .build();

        Self { agent, token, repo }
    }

    /// Submits a request to `bevyengine/bevy`
    fn get(&self, path: &str, repo: &str) -> ureq::Request {
        self.agent
            .get(&format!(
                "https://api.github.com/repos/bevyengine/{repo}/{path}",
            ))
            .set("Accept", "application/json")
            .set("Authorization", &format!("Bearer {}", self.token))
    }

    fn post_graphql(&self) -> ureq::Request {
        self.agent
            // WARN if this path ends with a / it will break
            .post("https://api.github.com/graphql")
            .set("Authorization", &format!("bearer {}", self.token))
    }

    /// Gets the list of all commits between two git ref
    pub fn compare_commits(
        &self,
        from: &str,
        to: &str,
    ) -> anyhow::Result<Vec<GithubCommitResponse>> {
        let mut commits = vec![];
        // The github page stuff is 1-based indexing and not 0-based.
        // Starting at 0 will give you the same page for 0 and 1.
        let mut page = 1;
        // To get all the prs we need to iterate on every page available.
        loop {
            let mut commits_in_page = self.compare_commits_page(from, to, page)?;
            println!("Page: {page} ({} commits)", commits_in_page.commits.len());
            // When it returns an empty page it means we have all the commits in the given range
            if commits_in_page.commits.is_empty() {
                break;
            }
            commits.append(&mut commits_in_page.commits);
            page += 1;
        }
        Ok(commits)
    }

    fn compare_commits_page(
        &self,
        from: &str,
        to: &str,
        page: i32,
    ) -> anyhow::Result<GithubCompareResponse> {
        let request = self
            .get(&format!("compare/{from}...{to}"), "bevy")
            .query("per_page", "250")
            .query("page", &page.to_string());
        Ok(request.call()?.into_json()?)
    }

    /// Gets a list of all merged PRs after the given date.
    /// The date needs to be in the YYYY-MM-DD format.
    pub fn get_merged_prs(
        &self,
        since: &str,
        label: Option<&str>,
    ) -> anyhow::Result<Vec<GithubIssuesResponse>> {
        let naive_datetime = NaiveDate::parse_from_str(since, "%Y-%m-%d")?
            .and_hms_opt(0, 0, 0)
            .expect("invalid time");
        let datetime_utc = Utc.from_utc_datetime(&naive_datetime);

        let mut prs = vec![];
        let mut page = 1;
        // The github rest API is limited to 100 prs per page,
        // so to get all the prs we need to iterate on every page available.
        loop {
            let mut prs_in_page = self.get_merged_prs_by_page(since, page, label)?;
            println!("Page: {} ({} prs)", page, prs_in_page.len());
            if prs_in_page.is_empty() {
                break;
            }

            prs.append(&mut prs_in_page);
            page += 1;
            if let Some(pr) = prs.last() {
                if pr.closed_at < datetime_utc {
                    println!(
                        "\x1b[93mSkipping PR closed before the target datetime {}\x1b[0m",
                        pr.closed_at
                    );
                    continue;
                }
            }
        }
        Ok(prs
            .iter()
            // Make sure the older PRs from the last page aren't returned
            .filter(|pr| pr.closed_at > datetime_utc)
            .cloned()
            .collect())
    }

    // Returns all PRs from the main branch that are merged.
    pub fn get_merged_prs_by_page(
        &self,
        date: &str,
        page: i32,
        label: Option<&str>,
    ) -> anyhow::Result<Vec<GithubIssuesResponse>> {
        let mut request = self
            .get("issues", "bevy")
            .query("since", &format!("{date}T00:00:00Z"))
            .query("state", "closed")
            .query("base", "main")
            .query("per_page", "100")
            .query("page", &page.to_string());
        if let Some(label) = label {
            request = request.query("labels", label);
        }
        let response: Vec<GithubIssuesResponse> = request.call()?.into_json()?;
        Ok(response
            .iter()
            // Make sure to only get the PRs that were merged
            .filter(|pr| {
                pr.pull_request
                    .as_ref()
                    .map(|pr| pr.merged_at.is_some())
                    .unwrap_or(false)
            })
            .cloned()
            .collect())
    }

    pub fn get_contributors(&self, commit_sha: &str) -> anyhow::Result<Vec<String>> {
        let query = format!(
            r#"
query {{
    resource(url: "https://github.com/bevyengine/{}/commit/{commit_sha}") {{
        ... on Commit {{
            authors(first: 10) {{
                nodes {{
                    user {{
                        login
                    }},
                    name
                }}
            }}
        }}
    }}
}}"#,
            self.repo
        );
        // for whatever reasons, github doesn't accept newlines in graphql queries
        let query = query.replace('\n', "");
        let resp = self
            .post_graphql()
            .send_json(ureq::json!({ "query": query }))?;
        let json: serde_json::Value = resp.into_json()?;

        let mut name_login_map = HashMap::new();

        // this returns an heavily nested struct so we parse it manually instead of having 6 intermediary struct
        let Some(nodes) = json["data"]["resource"]["authors"]["nodes"].as_array() else {
            bail!("nodes should be an array\n: {json}");
        };
        for node in nodes {
            let author: GithubAuthor = serde_json::from_value(node.clone())?;
            if let Some(user) = author.user {
                // If we find an already matching entry that had no login then use the login for that entry.
                // Otherwise if it doesn't exist just insert it.
                if matches!(name_login_map.get(&author.name), Some(None) | None) {
                    name_login_map.insert(author.name, Some(format!("@{}", user.login)));
                }
            } else {
                // Some entries have a name with no login but another entry with the same name and a login
                // So we first check if it already exists because we don't want to overwrite an entry that already had a login
                name_login_map.entry(author.name).or_insert(None);
            }
        }
        let contributors = name_login_map
            .iter()
            .map(|(name, login)| {
                if let Some(login) = login {
                    login
                } else {
                    println!(
                        "\x1b[93mUser login not found, using name '{name}' instead.\n{json}\x1b[0m"
                    );
                    name
                }
            })
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();
        Ok(contributors)
    }

    /// Gets the data for a specific commit on the provided `bevyengine` repo.
    pub fn get_commit(&self, git_ref: &str, repo: &str) -> anyhow::Result<GithubCommitResponse> {
        let request = self.get(&format!("commits/{git_ref}"), repo);
        Ok(request.call()?.into_json()?)
    }

    /// Get the list of all issues in the repo.
    ///
    /// This is useful to ensure that we don't open duplicate issues or PRs.
    /// Both issues and PRs are returned.
    /// Both open and closed issues are returned.
    pub fn get_issues(&self, repo: &str) -> anyhow::Result<Vec<GithubIssuesResponse>> {
        println!("Requesting a list of all issues on `bevyengine/bevy-website`");
        let response = self.get("issues", repo).set("state", "all").call()?;
        println!("Received response: {}", response.status_text());
        let issues: Vec<GithubIssuesResponse> = response.into_json()?;
        println!("Received {} issues", issues.len());
        Ok(issues)
    }

    /// Opens a new issue on the `bevyengine/bevy-website` repo.
    ///
    /// See [the Github API documentation](https://docs.github.com/en/rest/issues/issues?apiVersion=2022-11-28#create-an-issue) for more information.
    pub fn open_issue(
        &self,
        issue_title: &str,
        issue_body: &str,
        milestone: &str,
        labels: Vec<&str>,
    ) -> Result<(), IssueError> {
        let response = self
            .agent
            .post("https://api.github.com/repos/bevyengine/bevy-website/issues")
            .set("Accept", "application/json")
            .set("Authorization", &format!("Bearer {}", self.token))
            .send_json(ureq::json!({
                "title": issue_title,
                "body": issue_body,
                "milestone": milestone,
                "labels": labels,
            }))?;

        // Make sure the issue was created successfully
        if response.status() != 201 {
            return Err(IssueError::FailedToCreateIssue(response));
        } else {
            Ok(())
        }
    }
}

/// An issue that occurred while opening an issue on Github.
#[derive(Debug)]
pub enum IssueError {
    Ureq(ureq::Error),
    FailedToCreateIssue(Response),
}

impl From<ureq::Error> for IssueError {
    fn from(err: ureq::Error) -> Self {
        IssueError::Ureq(err)
    }
}
