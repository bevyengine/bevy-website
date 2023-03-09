use rand::{prelude::SliceRandom, thread_rng};
use serde::Serialize;
use std::{
    fs::{self},
    io::{self},
    path::PathBuf,
};

use generate_community::*;

const AVATAR_PATH: &str = "community_profile_picture";

fn main() -> io::Result<()> {
    let community_dir = PathBuf::from(&std::env::args().nth(1).unwrap());
    let root_dir = PathBuf::from(&std::env::args().nth(2).unwrap());
    let org_members_path = community_dir.join("The Bevy Organization");
    let community_members_path = community_dir.join("Community Members");

    let org_members = parse_members(&org_members_path)?;
    let mut community_members = parse_members(&community_members_path)?;
    let mut rng = thread_rng();
    community_members.shuffle(&mut rng);

    let roles: Roles =
        toml::de::from_str(&fs::read_to_string(&community_dir.join("_roles.toml")).unwrap())
            .unwrap();
    let role_map = roles.into_map();

    let mut buckets = vec![
        // Project leads
        Vec::new(),
        // Maintainers
        Vec::new(),
        // Other roles
        Vec::new(),
        // No roles
        Vec::new(),
    ];
    let mut assets_to_copy = Vec::new();
    for org_member in &org_members {
        if let Some(ProfilePicture::File(file)) = org_member.profile_picture.as_ref() {
            let filename = PathBuf::from(file).file_name().unwrap().to_os_string();
            assets_to_copy.push(org_members_path.join(filename));
        }
    }

    for member in &community_members {
        if let Some(ProfilePicture::File(file)) = member.profile_picture.as_ref() {
            let filename = PathBuf::from(file).file_name().unwrap().to_os_string();
            assets_to_copy.push(community_members_path.join(filename));
        }
    }

    for mut org_member in org_members {
        let roles = org_member
            .github
            .as_ref()
            .and_then(|github| role_map.get(github).cloned());
        org_member.roles = roles.clone();
        if let Some(r) = roles {
            if r.iter().find(|p| *p == "Project Lead").is_some() {
                buckets[0].push(org_member);
            } else if r.iter().find(|p| *p == "Maintainer").is_some() {
                buckets[1].push(org_member);
            } else if r.is_empty() {
                buckets[2].push(org_member);
            } else {
                buckets[3].push(org_member);
            }
        } else {
            buckets[3].push(org_member);
        }
    }
    let mut org_members = Vec::new();
    for mut group in buckets {
        group.shuffle(&mut rng);
        org_members.extend(group);
    }

    // TODO: collect files to copy to static dir here

    let org_members: Vec<_> = org_members
        .into_iter()
        .map(MemberOutput::from_member)
        .collect();
    let community_members: Vec<_> = community_members
        .into_iter()
        .map(MemberOutput::from_member)
        .collect();

    fs::write(
        root_dir.join("org_members.json"),
        serde_json::to_string_pretty(&org_members).unwrap(),
    )
    .unwrap();
    fs::write(
        root_dir.join("community_members.json"),
        serde_json::to_string_pretty(&community_members).unwrap(),
    )
    .unwrap();

    let asset_output_path = root_dir.join("static").join(AVATAR_PATH);
    fs::create_dir_all(&asset_output_path).unwrap();

    for asset in assets_to_copy {
        let filename = asset.file_name().unwrap().to_os_string();
        fs::copy(asset, asset_output_path.join(filename)).unwrap();
    }

    Ok(())
}

#[derive(Serialize)]
struct MemberOutput {
    title: String,
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

impl MemberOutput {
    pub fn from_member(member: Member) -> Self {
        Self {
            title: member.name.clone(),
            profile_picture: match member.profile_picture.as_ref() {
                Some(ProfilePicture::GitHub) => Some(format!(
                    "https://github.com/{}.png",
                    member.github.as_ref().unwrap()
                )),
                Some(ProfilePicture::File(file)) => Some(format!("{AVATAR_PATH}/{file}")),
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
        }
    }
}
