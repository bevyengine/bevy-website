use generate_community::*;

fn main() -> Result<(), String> {
    let community_dir = std::env::args().nth(1).unwrap();

    let people_root_section = parse_members(&community_dir).map_err(|err| err.to_string())?;

    validate_section(&people_root_section)?;

    Ok(())
}

fn validate_section(section: &Section) -> Result<(), String> {
    section
        .content
        .iter()
        .map(|node| validate_node(node))
        .find(|valid| valid.is_err())
        .unwrap_or(Ok(()))
}

fn validate_node(node: &CommunityNode) -> Result<(), String> {
    match node {
        CommunityNode::Section(section) => validate_section(section)?,
        CommunityNode::Member(member) => {
            match member.profile_picture.as_ref() {
                Some(ProfilePicture::File(file)) => {
                    if !member
                        .original_path
                        .as_ref()
                        .unwrap()
                        .clone()
                        .with_file_name(file)
                        .exists()
                    {
                        Err(format!(
                            "{:?}: Profile Picture set to a file, but file not found",
                            member.original_path.as_ref().unwrap()
                        ))?;
                    }
                }
                Some(ProfilePicture::GitHub) => {
                    if member.github.is_none() {
                        Err(format!(
                            "{:?}: Profile Picture set to GitHub, but no GitHub profile found",
                            member.original_path.as_ref().unwrap()
                        ))?;
                    }
                }
                None => (),
            };
            let mut social_links = 0;
            if member.github.is_some() {
                social_links += 1;
            }
            if member.discord.is_some() {
                social_links += 1;
            }
            if member.mastodon.is_some() {
                social_links += 1;
            }
            if member.twitter.is_some() {
                social_links += 1;
            }
            if member.itch_io.is_some() {
                social_links += 1;
            }
            if member.steam_developer.is_some() {
                social_links += 1;
            }
            if member.website.is_some() {
                social_links += 1;
            }
            if social_links > 4 {
                Err(format!(
                    "{:?}: Too many social links, keep at most 4",
                    member.original_path.as_ref().unwrap()
                ))?;
            }
        }
    }
    Ok(())
}
