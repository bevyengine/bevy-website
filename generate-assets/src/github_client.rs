use anyhow::bail;
use serde::Deserialize;

const BASE_URL: &str = "https://api.github.com";

#[derive(Deserialize, Debug)]
struct GithubContentResponse {
    encoding: String,
    content: String,
}

#[derive(Deserialize)]
struct GithubLicenseResponse {
    license: GithubLicenseLicense,
}

#[derive(Deserialize)]
struct GithubLicenseLicense {
    spdx_id: String,
}

#[derive(Deserialize, Debug)]
struct GithubSearchFile {
    total_count: u32,
    incomplete_results: bool,
    items: Vec<GithubSearchFileItem>,
}

#[derive(Deserialize, Debug)]
struct GithubSearchFileItem {
    path: std::path::PathBuf,
}

pub struct GithubClient {
    agent: ureq::Agent,
    token: String,
}

impl GithubClient {
    pub fn new(token: String) -> Self {
        let agent: ureq::Agent = ureq::AgentBuilder::new()
            .user_agent("bevy-website-generate-assets")
            .build();

        Self { agent, token }
    }

    /// Gets the content of a file from a github repo
    pub fn get_content(
        &self,
        username: &str,
        repository_name: &str,
        content_path: &str,
    ) -> anyhow::Result<String> {
        let response: GithubContentResponse = self
            .agent
            .get(&format!(
                "{BASE_URL}/repos/{username}/{repository_name}/contents/{content_path}"
            ))
            .set("Accept", "application/json")
            .set("Authorization", &format!("Bearer {}", self.token))
            .call()?
            .into_json()?;

        if response.encoding == "base64" {
            use base64::Engine;

            let data = base64::prelude::BASE64_STANDARD
                .decode(response.content.replace('\n', "").trim())?;
            Ok(String::from_utf8(data)?)
        } else {
            bail!("Content is not in base64");
        }
    }

    /// Gets the license from a github repo
    /// Technically, github supports multiple licenses, but the API only returns one
    #[allow(unused)]
    pub fn get_license(&self, username: &str, repository_name: &str) -> anyhow::Result<String> {
        let response: GithubLicenseResponse = self
            .agent
            .get(&format!(
                "{BASE_URL}/repos/{username}/{repository_name}/license"
            ))
            .set("Accept", "application/json")
            .set("Authorization", &format!("Bearer {}", self.token))
            .call()?
            .into_json()?;

        let license = response.license.spdx_id;

        if license != "NOASSERTION" {
            Ok(license)
        } else {
            bail!("No spdx license assertion")
        }
    }

    /// Search file by name
    pub fn search_file(
        &self,
        username: &str,
        repository_name: &str,
        file_name: &str,
    ) -> anyhow::Result<Vec<String>> {
        let response: GithubSearchFile = self
            .agent
            .get(&format!(
                "{BASE_URL}/search/code?q=repo:{username}/{repository_name}+filename:{file_name}"
            ))
            .set("Accept", "application/json")
            .set("Authorization", &format!("Bearer {}", self.token))
            .call()?
            .into_json()?;

        if response.incomplete_results {
            println!(
                "Too many {} files in repository, checking only the first {} ones.",
                file_name, response.total_count,
            );
        }

        let paths = response
            .items
            .iter()
            .filter_map(|i| {
                if let Some(path_string) = i.path.to_str() {
                    Some(path_string.to_string())
                } else {
                    println!("Path.to_str failed for {}", i.path.to_string_lossy());
                    None
                }
            })
            .collect();

        Ok(paths)
    }
}
