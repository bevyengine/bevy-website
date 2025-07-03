use std::path::PathBuf;

use generate_community::*;
use unicode_segmentation::UnicodeSegmentation;

fn main() -> Result<(), String> {
    let community_dir: PathBuf = std::env::args().nth(1).unwrap().into();

    let people_root_section = parse_members(&community_dir).unwrap();

    validate_section(&people_root_section)?;

    Ok(())
}

const MAX_BIO_LENGTH: usize = 180;

fn validate_section(section: &Section) -> Result<(), String> {
    // Validate each community node in the given section.
    for node in section.content.iter() {
        validate_node(node)?;
    }

    // If this gets run, then there are no validation errors.
    Ok(())
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
                    Err(format!("Bio is longer than the maximum allowed length of {MAX_BIO_LENGTH}. It is currently {grapheme_count} characters long."))?;
                }
            }

            if member.roles.is_some() {
                Err("Roles must be defined in the roles.toml file")?;
            }
        }
    }
    Ok(())
}
