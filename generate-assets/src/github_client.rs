use anyhow::bail;
use serde::Deserialize;

const BASE_URL: &str = "https://api.github.com";

#[derive(Deserialize)]
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
            let data = base64::decode(response.content.replace('\n', "").trim())?;
            Ok(String::from_utf8(data)?)
        } else {
            bail!("Content is not in base64");
        }
    }

    /// Gets the license from a github repo
    /// Technically, github supports multiple licenses, but the api only returns one
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

        Ok(response.license.spdx_id)
    }
}
