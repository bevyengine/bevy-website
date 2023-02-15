use std::{collections::HashMap, fmt::Debug};

use anyhow::{bail, Ok};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

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
pub struct GithubIssuesResponse {
    pub title: String,
    pub number: i32,
    pub body: Option<String>,
    pub labels: Vec<GithubLabel>,
    pub user: GithubUser,
    pub closed_at: DateTime<Utc>,
}

pub struct GithubClient {
    agent: ureq::Agent,
    token: String,
    user_cache: HashMap<String, GithubUserSearchResponse>,
}

impl GithubClient {
    pub fn new(token: String) -> Self {
        let agent: ureq::Agent = ureq::AgentBuilder::new()
            .user_agent("bevy-website-generate-assets")
            .build();

        Self {
            agent,
            token,
            user_cache: Default::default(),
        }
    }

    fn get(&self, path: &str) -> ureq::Request {
        self.agent
            .get(path)
            .set("Accept", "application/json")
            .set("Authorization", &format!("Bearer {}", self.token))
    }

    fn post(&self, path: &str) -> ureq::Request {
        self.agent
            .post(path)
            .set("Authorization", &format!("bearer {}", self.token))
    }

    #[allow(unused)]
    pub fn get_branch_sha(&self, branch_name: &str) -> anyhow::Result<String> {
        let request = self.get("https://api.github.com/repos/bevyengine/bevy/branches");
        let reponse: Vec<GithubBranchesResponse> = request.call()?.into_json()?;
        for branch in &reponse {
            if branch.name == branch_name {
                return Ok(branch.commit.sha.clone());
            }
        }
        bail!("commit sha not found for main branch")
    }

    /// Gets a list of all PRs merged by bors after the given date.
    /// The date needs to be in the YYYY-MM-DD format
    /// To validate that bors merged the PR we simply check if the pr title contains "[Merged by Bors] - "
    pub fn get_commits(&self, since: &str, sha: &str) -> anyhow::Result<Vec<GithubCommitResponse>> {
        let mut commits = vec![];
        let mut page = 1;
        // The github rest api is limited to 100 prs per page,
        // so to get all the prs we need to iterate on every page available.
        loop {
            let mut commits_in_page = self.get_commits_by_page(since, page, sha)?;
            println!("Page: {} ({} commits)", page, commits_in_page.len());
            if commits_in_page.is_empty() {
                break;
            }
            commits.append(&mut commits_in_page);
            page += 1;
        }
        Ok(commits)
    }

    #[allow(unused)]
    pub fn get_commits_by_page(
        &self,
        since: &str,
        page: i32,
        sha: &str,
    ) -> anyhow::Result<Vec<GithubCommitResponse>> {
        let request = self
            .get("https://api.github.com/repos/bevyengine/bevy/commits")
            .query("since", &format!("{since}T00:00:00Z"))
            .query("per_page", "100")
            .query("page", &page.to_string())
            .query("sha", sha);
        Ok(request.call()?.into_json()?)
    }

    pub fn compare_commits(&self, from: &str, to: &str) -> anyhow::Result<GithubCompareResponse> {
        let request = self.get(&format!(
            "https://api.github.com/repos/bevyengine/bevy/compare/{from}...{to}"
        ));
        Ok(request.call()?.into_json()?)
    }

    #[allow(unused)]
    pub fn get_pr_by_number(&self, pr_number: &str) -> anyhow::Result<GithubPullRequestResponse> {
        let request = self.get(&format!(
            "https://api.github.com/repos/bevyengine/bevy/pulls/{pr_number}",
        ));
        Ok(request.call()?.into_json()?)
    }

    #[allow(unused)]
    pub fn get_user_by_email(&mut self, email: &str) -> anyhow::Result<GithubUserSearchResponse> {
        // This api is really slow so we keep a cache of responses
        if let Some(response) = self.user_cache.get(email) {
            return Ok(response.clone());
        }
        let request = self
            .get("https://api.github.com/search/users")
            .query("q", email);
        let response = request.call()?.into_json()?;
        self.user_cache.insert(email.to_string(), response);
        Ok(self.user_cache.get(email).unwrap().clone())
    }

    /// Gets a list of all PRs merged by bors after the given date.
    /// The date needs to be in the YYYY-MM-DD format
    /// To validate that bors merged the PR we simply check if the pr title contains "[Merged by Bors] - "
    pub fn get_merged_prs(
        &self,
        since: &str,
        label: Option<&str>,
    ) -> anyhow::Result<Vec<GithubIssuesResponse>> {
        let naive_datetime = NaiveDate::parse_from_str(since, "%Y-%m-%d")?.and_hms(0, 0, 0);
        let datetime_utc = DateTime::<Utc>::from_utc(naive_datetime, Utc);

        let mut prs = vec![];
        let mut page = 1;
        // The github rest api is limited to 100 prs per page,
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

    // Returns all PRs from the main branch that are closed
    pub fn get_merged_prs_by_page(
        &self,
        date: &str,
        page: i32,
        label: Option<&str>,
    ) -> anyhow::Result<Vec<GithubIssuesResponse>> {
        let mut request = self
            .get("https://api.github.com/repos/bevyengine/bevy/issues")
            .query("since", &format!("{date}T00:00:00Z"))
            .query("state", "closed")
            .query("base", "main")
            .query("per_page", "100")
            .query("page", &page.to_string());
        if let Some(label) = label {
            request = request.query("labels", label);
        }
        println!("{}", request.call()?.into_string()?);
        todo!();
        let response: Vec<GithubIssuesResponse> = request.call()?.into_json()?;
        Ok(response
            .iter()
            // Make sure to only get the PRs that were merged by bors
            .filter(|pr| pr.title.starts_with("[Merged by Bors] - "))
            .cloned()
            .collect())
    }

    pub fn get_contributors(&self, commit_sha: &str) -> anyhow::Result<Vec<String>> {
        let query = format!(
            r#"
query {{
    resource(url: "https://github.com/bevyengine/bevy/commit/{commit_sha}") {{
        ... on Commit {{
            authors(first: 10) {{
                nodes {{
                    user {{
                        login
                    }}
                }}
            }}
        }}
    }}
}}"#
        );
        // for whatever reasons, github doesn't accept newlines in graphql queries
        let query = query.replace('\n', "");
        let resp = self
            .post("https://api.github.com/graphql") // WARN if this ends in a / it will break
            .send_json(ureq::json!({ "query": query }))?;
        let json: serde_json::Value = resp.into_json()?;

        let mut logins = vec![];

        // this returns an heavily nested struct so we parse it manually instead of having 6 intermediary struct
        let nodes = &json["data"]["resource"]["authors"]["nodes"];

        if nodes.is_array() {
            for node in nodes.as_array().unwrap() {
                let login = &node["user"]["login"];
                let login = if login.is_array() {
                    login
                        .as_array()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .map(|l| l.as_str().unwrap().to_string())
                        .collect()
                } else if login.is_string() {
                    vec![login.as_str().unwrap().to_string()]
                } else {
                    bail!("Invalid login format. If it contains a null, it probably means we are being rate limited.\n{json}");
                };
                logins.extend(login);
            }
        } else {
            bail!("nodes should be an array\n: {json}");
        }

        Ok(logins)
    }
}
