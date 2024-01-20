use generate_community::*;
use unicode_segmentation::UnicodeSegmentation;

fn main() -> Result<(), String> {
    let community_dir = std::env::args().nth(1).unwrap();

    let people_root_section = parse_members(&community_dir).map_err(|err| err.to_string())?;

    validate_section(&people_root_section)?;

    Ok(())
}

const MAX_BIO_LENGTH: usize = 180;

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

            if let Some(bio) = &member.bio {
                let grapheme_count = bio.graphemes(true).count();
                if grapheme_count > MAX_BIO_LENGTH {
                    Err(format!("Bio is longer than the maximum allowed length of {}. It is currently {} characters long.", MAX_BIO_LENGTH, grapheme_count))?;
                }
            }

            if member.roles.is_some() {
                Err(format!("Roles must be defined in the roles.toml file"))?;
            }
        }
    }
    Ok(())
}
