use anyhow::bail;
use serde::Deserialize;

const BASE_URL: &str = "https://gitlab.com/api/v4/projects";

#[derive(Deserialize)]
pub struct GitlabProjectSearchResponse {
    pub id: usize,
    pub default_branch: String,
}

#[derive(Deserialize)]
struct GitlabContentResponse {
    encoding: String,
    content: String,
}

pub struct GitlabClient {
    agent: ureq::Agent,
    // This is not currently used because we have so few assets using gitlab that we don't need it.
    _token: String,
}

impl GitlabClient {
    pub fn new(token: String) -> Self {
        let agent: ureq::Agent = ureq::AgentBuilder::new()
            .user_agent("bevy-website-generate-assets")
            .build();

        Self {
            agent,
            _token: token,
        }
    }

    /// Finds a list of repo based on their name
    /// Useful to get the repo id and default_branch
    pub fn search_project_by_name(
        &self,
        repository_name: &str,
    ) -> anyhow::Result<Vec<GitlabProjectSearchResponse>> {
        let reponse: Vec<GitlabProjectSearchResponse> = self
            .agent
            .get(&format!("{BASE_URL}?search={repository_name}"))
            .set("Accept", "application/json")
            // .set("Authorization", &format!("Bearer {}", self.token))
            .call()?
            .into_json()?;
        Ok(reponse)
    }

    /// Gets the content of a file from a gitlab repo
    pub fn get_content(
        &self,
        id: usize,
        default_branch: &str,
        content_path: &str,
    ) -> anyhow::Result<String> {
        let reponse: GitlabContentResponse = self
            .agent
            .get(&format!(
                "{BASE_URL}/{id}/repository/files/{content_path}?ref={default_branch}"
            ))
            .set("Accept", "application/json")
            // .set("Authorization", &format!("Bearer {}", self.token))
            .call()?
            .into_json()?;

        if reponse.encoding == "base64" {
            let data = base64::decode(reponse.content.replace('\n', "").trim())?;
            Ok(String::from_utf8(data)?)
        } else {
            bail!("Content is not in base64");
        }
    }
}
