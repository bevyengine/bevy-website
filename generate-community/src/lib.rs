use serde::Deserialize;
use std::path::Path;
use std::{collections::HashMap, fs, io, path::PathBuf};

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Member {
    pub name: String,
    #[serde(default, deserialize_with = "extract_profile_picture")]
    pub profile_picture: Option<ProfilePicture>,
    pub sponsor: Option<String>,

    pub bio: Option<String>,

    // social links
    pub discord: Option<String>,
    pub discord_userid: Option<String>,
    pub github: Option<String>,
    #[serde(default, deserialize_with = "extract_mastodon")]
    pub mastodon: Option<Mastodon>,
    pub twitter: Option<String>,
    pub itch_io: Option<String>,
    pub steam_developer: Option<String>,
    pub website: Option<String>,

    // this field is not read from the toml file
    #[serde(skip)]
    pub original_path: Option<PathBuf>,
    pub roles: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Roles {
    pub project_lead: Vec<String>,
    pub maintainer: Vec<String>,
    pub sme: Vec<Sme>,
}

impl Roles {
    pub fn into_map(self) -> HashMap<String, Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::default();
        for id in self.project_lead {
            let roles = map.entry(id).or_default();
            roles.push("Project Lead".to_string());
        }
        for id in self.maintainer {
            let roles = map.entry(id).or_default();
            roles.push("Maintainer".to_string());
        }

        for sme in self.sme {
            let roles = map.entry(sme.id).or_default();
            roles.push(format!("SME-{}", sme.area));
        }
        map
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Sme {
    pub area: String,
    pub id: String,
}

#[derive(Debug, Clone)]
pub enum ProfilePicture {
    GitHub,
    File(String),
}

fn extract_profile_picture<'de, D>(deserializer: D) -> Result<Option<ProfilePicture>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    match buf.as_str() {
        "GitHub" => Ok(Some(ProfilePicture::GitHub)),
        _ => Ok(Some(ProfilePicture::File(buf.replace("./", "")))),
    }
}

#[derive(Debug, Clone)]
pub struct Mastodon {
    pub username: String,
    pub instance: String,
}

fn extract_mastodon<'de, D>(deserializer: D) -> Result<Option<Mastodon>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    let mut details = buf.split('@');
    details.next();
    Ok(Some(Mastodon {
        username: details.next().unwrap().to_string(),
        instance: details.next().unwrap().to_string(),
    }))
}

pub fn parse_members(dir: &Path) -> io::Result<Vec<Member>> {
    let mut members = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.file_name().unwrap() == "_category.toml"
            || path.extension().expect("file must have an extension") != "toml"
        {
            continue;
        }
        let mut member: Member = toml::de::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        // Do we even use that
        member.original_path = Some(path);
        members.push(member);
    }
    Ok(members)
}
