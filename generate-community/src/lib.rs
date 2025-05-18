use serde::Deserialize;
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

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
    pub bluesky: Option<String>,
    pub instagram: Option<String>,
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
        _ => Ok(Some(ProfilePicture::File(buf))),
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

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub filename: Option<String>,
    pub content: Vec<CommunityNode>,
    pub template: Option<String>,
    pub header: Option<String>,
    pub order: Option<usize>,
    pub sort_order_reversed: bool,
}

impl Section {
    pub fn apply_roles(&mut self, roles: &HashMap<String, Vec<String>>) {
        for content in &mut self.content {
            match content {
                CommunityNode::Section(section) => section.apply_roles(roles),
                CommunityNode::Member(member) => {
                    member.roles = member
                        .github
                        .as_ref()
                        .and_then(|github| roles.get(github).cloned());
                }
            }
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum CommunityNode {
    Section(Section),
    Member(Member),
}

impl CommunityNode {
    pub fn name(&self) -> String {
        match self {
            CommunityNode::Section(content) => content.name.clone(),
            CommunityNode::Member(content) => content.name.clone(),
        }
    }
    pub fn order(&self) -> usize {
        match self {
            CommunityNode::Section(content) => content.order.unwrap_or(99999),
            CommunityNode::Member(content) => {
                if let Some(roles) = &content.roles {
                    if roles.iter().any(|p| p == "Project Lead") {
                        0
                    } else if roles.iter().any(|p| p == "Maintainer") {
                        1
                    } else if !roles.is_empty() {
                        2
                    } else {
                        99999
                    }
                } else {
                    99999
                }
            }
        }
    }
}

fn visit_dirs(dir: PathBuf, section: &mut Section) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.file_name().unwrap() == ".git" || path.file_name().unwrap() == ".github" {
                continue;
            }
            if path.is_dir() {
                let folder = path.file_name().unwrap();
                let (order, sort_order_reversed) = if path.join("_category.toml").exists() {
                    let from_file: toml::Value = toml::de::from_str(
                        &fs::read_to_string(path.join("_category.toml")).unwrap(),
                    )
                    .unwrap();
                    (
                        from_file
                            .get("order")
                            .and_then(|v| v.as_integer())
                            .map(|v| v as usize),
                        from_file
                            .get("sort_order_reversed")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false),
                    )
                } else {
                    (None, false)
                };
                let mut new_section = Section {
                    name: folder.to_str().unwrap().to_string(),
                    filename: None,
                    content: vec![],
                    template: None,
                    header: None,
                    order,
                    sort_order_reversed,
                };
                visit_dirs(path.clone(), &mut new_section)?;
                section.content.push(CommunityNode::Section(new_section));
            } else {
                if path.file_name().unwrap() == "_category.toml"
                    || path.file_name().unwrap() == "_roles.toml"
                    || path.extension().expect("file must have an extension") != "toml"
                {
                    continue;
                }
                let mut member: Member =
                    toml::de::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
                member.original_path = Some(path);
                section.content.push(CommunityNode::Member(member));
            }
        }
    }
    Ok(())
}

pub fn parse_members(community_dir: &Path) -> io::Result<Section> {
    let mut people_root_section = Section {
        name: "People".to_string(),
        filename: None,
        content: vec![],
        template: Some("people.html".to_string()),
        header: Some("People".to_string()),
        order: None,
        sort_order_reversed: false,
    };

    visit_dirs(community_dir.to_path_buf(), &mut people_root_section)?;

    Ok(people_root_section)
}
