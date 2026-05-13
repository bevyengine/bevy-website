use rand::{prelude::SliceRandom, rng};
use serde::Serialize;
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{self, prelude::*},
    path::{Path, PathBuf},
};

use generate_community::*;

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);

    // Read CLI arguments
    let community_dir: PathBuf = args
        .next()
        .expect("Expected first argument to be the path to the community directory.")
        .into();
    let content_dir: PathBuf = args
        .next()
        .expect("Expected second argument to be the path to the website content directory.")
        .into();
    let content_sub_dir: PathBuf = args
        .next()
        .expect("Expected third argument to be the name of the community directory.")
        .into();

    // Create the content directory if it does not exist.
    // This is unlikely to ever fail.
    fs::create_dir_all(&content_dir)?;

    // Read a list of all people from the bevy-community directory.
    let mut people_root_section = parse_members(&community_dir)?;

    let roles_path = community_dir.join("_roles.toml");

    people_root_section.apply_roles({
        let contents = fs::read_to_string(roles_path).expect("Could not read _roles.toml.");
        let roles: Roles = toml::from_str(&contents).expect("_roles.toml is not valid TOML.");
        &roles.into_map()
    });

    people_root_section.write(&content_dir, &content_sub_dir, 0)?;

    let Some(CommunityNode::Section(org)) = people_root_section
        .content
        .iter()
        .find(|node| node.name() == "The Bevy Organization")
    else {
        panic!("unexpected kind of node or missing for The Bevy Organization");
    };

    let mut donate = org.clone();

    donate.name = "Supporting Bevy Development".to_string();
    donate.filename = Some("donate".to_string());
    donate.header = Some("Supporting Bevy".to_string());
    donate.template = Some("donate-community.html".to_string());

    donate.content.retain(|node| {
        let CommunityNode::Member(member) = node else {
            panic!("got an unexpected subsection");
        };

        member.sponsor.is_some()
    });

    donate.write(&content_dir, &content_sub_dir, 0)?;

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
    bluesky: Option<String>,
    instagram: Option<String>,
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
                bluesky: member.bluesky.clone(),
                instagram: member.instagram.clone(),
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
        let path = root_path.join(current_path);

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

        let file_name = self
            .original_path
            .as_ref()
            .and_then(|f| f.file_name())
            .map(|f| f.to_string_lossy().replace(".toml", ""))
            .expect("Failed to get file_name");
        let mut file = File::create(path.join(format!("{file_name}.md")))?;
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
        let section_path = current_path.join(
            self.filename
                .as_ref()
                .unwrap_or(&self.name.to_ascii_lowercase()),
        );
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
            members.shuffle(&mut rng());
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
            content.write(root_path, &section_path, i)?;
        }
        Ok(())
    }
}
