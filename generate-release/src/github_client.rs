use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use anyhow::bail;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use serde::Deserialize;
use thiserror::Error;
use ureq::Response;

/// A GitHub repository in the `bevyengine` organization.
#[derive(Debug, Clone, Copy)]
pub enum BevyRepo {
    Bevy,
    BevyWebsite,
}

impl BevyRepo {
    fn as_str(&self) -> &'static str {
        match self {
            BevyRepo::Bevy => "bevy",
            BevyRepo::BevyWebsite => "bevy-website",
        }
    }
}

impl Display for BevyRepo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct GithubCommitResponse {
    pub sha: String,
    pub commit: GithubCommitContent,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Committer {
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
pub struct GithubCompareResponse {
    pub commits: Vec<GithubCommitResponse>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GithubLabel {
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GithubIssuesResponsePullRequest {
    pub merged_at: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GithubIssuesResponse {
    pub title: String,
    pub number: u64,
    pub body: Option<String>,
    pub labels: Vec<GithubLabel>,
    pub user: GithubUser,
    pub closed_at: Option<DateTime<Utc>>,
    pub pull_request: Option<GithubIssuesResponsePullRequest>,
}

/// See the [response schema](https://docs.github.com/en/rest/issues/issues?apiVersion=2022-11-28#create-an-issue)
///
/// Not all fields are included, only the ones that are needed. Feel free to add more as needed!
#[derive(Deserialize, Clone, Debug)]
pub struct GithubIssueOpenedResponse {
    /// The human-friendly HTML URL of the freshly opened issue
    pub html_url: String,
}

pub struct GithubClient {
    agent: ureq::Agent,
    token: String,
    pub repo: BevyRepo,
}

impl GithubClient {
    pub fn new(token: String, repo: BevyRepo) -> Self {
        let agent: ureq::Agent = ureq::AgentBuilder::new()
            .user_agent("bevy-website-generate-release")
            .build();

        Self { agent, token, repo }
    }

    /// Submits a GET request to `bevyengine/{repo}`
    fn get(&self, path: &str, repo: BevyRepo) -> ureq::Request {
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
            .get(&format!("compare/{from}...{to}"), BevyRepo::Bevy)
            .query("per_page", "250")
            .query("page", &page.to_string());
        Ok(request.call()?.into_json()?)
    }

    /// Gets a filtered list of issues and PRs from `bevyengine/{repo}`.
    ///
    /// If `since` is provided, it should be a date in the YYYY-MM-DD format.
    pub fn get_issues_and_prs(
        &self,
        repo: BevyRepo,
        state: IssueState,
        since: Option<&str>,
        label: Option<&str>,
    ) -> anyhow::Result<Vec<GithubIssuesResponse>> {
        let datetime_utc = if let Some(since) = since {
            let naive_datetime = NaiveDate::parse_from_str(since, "%Y-%m-%d")?
                .and_hms_opt(0, 0, 0)
                .expect("invalid time");
            Some(Utc.from_utc_datetime(&naive_datetime))
        } else {
            None
        };

        let mut prs = vec![];
        let mut page = 1;
        // The github rest API is limited to 100 prs per page,
        // so to get all the prs we need to iterate on every page available.
        loop {
            if page == 100 {
                // We can't fetch more than 99 pages of data or GitHub will
                // fail with a 422.
                break;
            }
            let mut prs_in_page =
                self.get_issues_and_prs_by_page(page, repo, state, since, label)?;
            println!("Page: {} ({} prs)", page, prs_in_page.len());
            if prs_in_page.is_empty() {
                break;
            }

            prs.append(&mut prs_in_page);
            page += 1;
        }

        // Make sure the older PRs from the last page aren't returned
        if let Some(datetime_utc) = datetime_utc {
            println!("Filtering PRs closed before the target datetime {datetime_utc}");

            prs.retain(|pr| {
                pr.closed_at
                    .is_some_and(|closed_at| closed_at >= datetime_utc)
            });
        }

        Ok(prs)
    }

    /// Request issues and PRs by the page returned by the Github API
    fn get_issues_and_prs_by_page(
        &self,
        page: i32,
        repo: BevyRepo,
        state: IssueState,
        date: Option<&str>,
        label: Option<&str>,
    ) -> anyhow::Result<Vec<GithubIssuesResponse>> {
        let mut request = self
            .get("issues", repo)
            .query("state", state.as_github_str())
            .query("base", "main")
            .query("per_page", "100")
            .query("page", &page.to_string());

        if let Some(date) = date {
            request = request.query("since", &format!("{date}T00:00:00Z"));
        }

        if let Some(label) = label {
            request = request.query("labels", label);
        }
        let mut responses: Vec<GithubIssuesResponse> = request.call()?.into_json()?;

        // Filter the PRs based on the requested state
        match state {
            IssueState::Open => responses.retain(|pr| pr.closed_at.is_none()),
            IssueState::Closed => responses.retain(|pr| pr.closed_at.is_some()),
            IssueState::Merged => responses.retain(|pr| {
                pr.pull_request
                    .as_ref()
                    .is_some_and(|pr| pr.merged_at.is_some())
            }),
            IssueState::All => (),
        };

        Ok(responses)
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
    pub fn get_commit(
        &self,
        git_ref: &str,
        repo: BevyRepo,
    ) -> anyhow::Result<GithubCommitResponse> {
        let request = self.get(&format!("commits/{git_ref}"), repo);
        Ok(request.call()?.into_json()?)
    }

    /// Opens a new issue on the specified repo.
    ///
    /// See [the Github API documentation](https://docs.github.com/en/rest/issues/issues?apiVersion=2022-11-28#create-an-issue) for more information.
    #[allow(clippy::result_large_err)]
    pub fn open_issue(
        &self,
        repo: BevyRepo,
        issue_title: &str,
        issue_body: &str,
        labels: Vec<&str>,
    ) -> Result<GithubIssueOpenedResponse, IssueError> {
        let response = self
            .agent
            .post(&format!(
                "https://api.github.com/repos/bevyengine/{repo}/issues"
            ))
            .set("Authorization", &format!("Bearer {}", self.token))
            .set("Accept", "application/vnd.github+json")
            .set("X-GitHub-Api-Version", "2022-11-28")
            .send_json(ureq::json!({
                "title": issue_title,
                "body": issue_body,
                // TODO: add the milestone. Tracked in https://github.com/bevyengine/bevy-website/issues/1269
                // Note that this must be provided as an integer, so we'll have to look up the milestone ID.
                "labels": labels,
            }))?;

        // Make sure the issue was created successfully
        if response.status() != 201 {
            Err(IssueError::FailedToCreateIssue(response))
        } else {
            let parsed_response: GithubIssueOpenedResponse = response.into_json()?;

            Ok(parsed_response)
        }
    }

    /// Leaves a comment on the specified issue or pull request.
    ///
    /// See the [Github API documentation](https://docs.github.com/en/rest/issues/comments?apiVersion=2022-11-28) for more information.
    #[allow(clippy::result_large_err)]
    pub fn leave_comment(
        &self,
        repo: BevyRepo,
        issue_number: u64,
        comment: &str,
    ) -> Result<Response, ureq::Error> {
        let response = self
            .agent
            .post(&format!(
                "https://api.github.com/repos/bevyengine/{repo}/issues/{issue_number}/comments",
            ))
            .set("Authorization", &format!("Bearer {}", self.token))
            .set("Accept", "application/vnd.github+json")
            .set("X-GitHub-Api-Version", "2022-11-28")
            .send_json(ureq::json!({
                "body": comment,
            }))?;

        Ok(response)
    }
}

/// An issue that occurred while opening an issue on Github.
#[derive(Error, Debug)]
pub enum IssueError {
    #[error("error making request")]
    Ureq(#[from] ureq::Error),
    #[error("failed to create issue")]
    FailedToCreateIssue(Response),
    #[error("failed to parse response")]
    FailedToParseResponse(#[from] std::io::Error),
}

/// The status of an issue or PR on Github.
#[derive(Debug, Clone, Copy)]
pub enum IssueState {
    #[allow(dead_code)]
    Open,
    #[allow(dead_code)]
    Closed,
    Merged,
    All,
}

impl IssueState {
    /// The string representation of the issue state.
    ///
    /// This is requested by the Github API,
    /// as documented [here](https://docs.github.com/en/rest/issues/issues?apiVersion=2022-11-28#list-repository-issues).
    fn as_github_str(&self) -> &'static str {
        match self {
            IssueState::Open => "open",
            // All merged PRs are considered closed,
            // but not all closed PRs are merged.
            IssueState::Closed | IssueState::Merged => "closed",
            IssueState::All => "all",
        }
    }
}
