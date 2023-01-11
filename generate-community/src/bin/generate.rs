use rand::{prelude::SliceRandom, thread_rng};
use serde::Serialize;
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{self, prelude::*},
    path::{Path, PathBuf},
    str::FromStr,
};

use generate_community::*;

fn main() -> io::Result<()> {
    let community_dir = std::env::args().nth(1).unwrap();
    let content_dir = std::env::args().nth(2).unwrap();
    let content_sub_dir = std::env::args().nth(3).unwrap();
    let _ = fs::create_dir(content_dir.clone());
    let mut people_root_section = parse_members(&community_dir)?;

    let mut roles_path = PathBuf::from_str(&community_dir).unwrap();
    roles_path.push("_roles.toml");
    let roles: Roles = toml::de::from_str(&fs::read_to_string(&roles_path).unwrap()).unwrap();
    let role_map = roles.into_map();
    people_root_section.apply_roles(&role_map);

    people_root_section.write(Path::new(&content_dir), Path::new(&content_sub_dir), 0)?;
    Ok(())
}

trait FrontMatterWriter {
    fn write(&self, root_path: &Path, current_path: &Path, weight: usize) -> io::Result<()>;
}

#[derive(Serialize)]
struct FrontMatterMember {
    title: String,
    weight: usize,
    extra: FrontMatterMemberExtra,
}

#[derive(Serialize)]
struct FrontMatterMemberExtra {
    profile_picture: Option<String>,
    sponsor: Option<String>,
    bio: Option<String>,
    discord: Option<String>,
    discord_userid: Option<String>,
    github: Option<String>,
    mastodon_user: Option<String>,
    mastodon_instance: Option<String>,
    twitter: Option<String>,
    itch_io: Option<String>,
    steam_developer: Option<String>,
    website: Option<String>,
    roles: Option<Vec<String>>,
}

impl From<&Member> for FrontMatterMember {
    fn from(member: &Member) -> Self {
        FrontMatterMember {
            title: member.name.clone(),
            weight: 0,
            extra: FrontMatterMemberExtra {
                profile_picture: match member.profile_picture.as_ref() {
                    Some(ProfilePicture::GitHub) => Some(format!(
                        "https://github.com/{}.png",
                        member.github.as_ref().unwrap()
                    )),
                    Some(ProfilePicture::File(file)) => Some(file.clone()),
                    None => None,
                },
                bio: member.bio.clone(),
                sponsor: member.sponsor.clone(),
                discord: member.discord.clone(),
                discord_userid: member.discord_userid.clone(),
                github: member.github.clone(),
                mastodon_user: member.mastodon.as_ref().map(|m| m.username.clone()),
                mastodon_instance: member.mastodon.as_ref().map(|m| m.instance.clone()),
                twitter: member.twitter.clone(),
                itch_io: member.itch_io.clone(),
                steam_developer: member.steam_developer.clone(),
                website: member.website.clone(),
                roles: member.roles.clone(),
            },
        }
    }
}

impl FrontMatterWriter for Member {
    fn write(&self, root_path: &Path, current_path: &Path, weight: usize) -> io::Result<()> {
        let path = root_path.join(&current_path);

        let mut frontmatter = FrontMatterMember::from(self);
        frontmatter.weight = weight;
        if let Some(ProfilePicture::File(file)) = self.profile_picture.as_ref() {
            let image_file_path = path.join(file);
            let image_file_link = current_path.join(file);
            let original_image = self
                .original_path
                .as_ref()
                .unwrap()
                .clone()
                .with_file_name(file);

            frontmatter.extra.profile_picture =
                image_file_link.to_str().map(|link| link.to_string());
            let _ = fs::copy(original_image, image_file_path);
        }

        let mut file = File::create(path.join(format!(
            "{}.md",
            self.name.to_ascii_lowercase().replace("/", "-")
        )))?;
        file.write_all(
            format!(
                r#"+++
{}
+++
"#,
                toml::to_string(&frontmatter).unwrap(),
            )
            .as_bytes(),
        )?;

        Ok(())
    }
}

impl FrontMatterWriter for CommunityNode {
    fn write(&self, root_path: &Path, current_path: &Path, weight: usize) -> io::Result<()> {
        match self {
            CommunityNode::Section(content) => content.write(root_path, current_path, weight),
            CommunityNode::Member(content) => content.write(root_path, current_path, weight),
        }
    }
}

#[derive(Serialize)]
struct FrontMatterSection {
    title: String,
    sort_by: String,
    template: Option<String>,
    weight: usize,
    extra: FrontMatterSectionExtra,
}

#[derive(Serialize)]
struct FrontMatterSectionExtra {
    header_message: Option<String>,
    sort_order_reversed: bool,
}

impl From<&Section> for FrontMatterSectionExtra {
    fn from(section: &Section) -> Self {
        FrontMatterSectionExtra {
            header_message: section.header.clone(),
            sort_order_reversed: section.sort_order_reversed,
        }
    }
}

impl From<&Section> for FrontMatterSection {
    fn from(section: &Section) -> Self {
        FrontMatterSection {
            title: section.name.clone(),
            sort_by: "weight".to_string(),
            template: section.template.clone(),
            weight: section.order.unwrap_or(0),
            extra: section.into(),
        }
    }
}

impl FrontMatterWriter for Section {
    fn write(&self, root_path: &Path, current_path: &Path, weight: usize) -> io::Result<()> {
        let section_path = current_path.join(self.name.to_ascii_lowercase());
        let path = root_path.join(&section_path);
        fs::create_dir(path.clone())?;

        let mut frontmatter = FrontMatterSection::from(self);
        if self.order.is_none() {
            frontmatter.weight = weight;
        }

        let mut file = File::create(path.join("_index.md"))?;
        file.write_all(
            format!(
                r#"+++
{}
+++
"#,
                toml::to_string(&frontmatter).unwrap(),
            )
            .as_bytes(),
        )?;

        let mut sorted_section = vec![];
        for content in self.content.iter() {
            if let CommunityNode::Section(section) = content {
                sorted_section.push(CommunityNode::Section(section.clone()));
            }
        }
        sorted_section.sort_by_key(|section| format!("{}-{}", section.order(), section.name()));

        let mut order_groups: BTreeMap<usize, Vec<CommunityNode>> = BTreeMap::default();
        for content in self.content.iter() {
            if let CommunityNode::Member(_member) = content {
                let order = content.order();
                let members = order_groups.entry(order).or_default();
                members.push(content.clone());
            }
        }

        for members in order_groups.values_mut() {
            members.shuffle(&mut thread_rng());
        }

        let mut ordered_members = Vec::new();
        for (_, mut members) in order_groups {
            ordered_members.append(&mut members);
        }

        for (i, content) in sorted_section
            .iter()
            .chain(ordered_members.iter())
            .enumerate()
        {
            content.write(root_path, &section_path, i)?
        }
        Ok(())
    }
}
