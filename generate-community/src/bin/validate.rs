use generate_community::*;
use std::path::PathBuf;
use unicode_segmentation::UnicodeSegmentation;

fn main() -> Result<(), String> {
    let community_dir = PathBuf::from(&std::env::args().nth(1).unwrap());
    let org_members_path = community_dir.join("The Bevy Organization");
    let community_members_path = community_dir.join("Community Members");

    let org_members = parse_members(&org_members_path).unwrap();
    for member in org_members {
        validate_member(member)?;
    }
    let community_members = parse_members(&community_members_path).unwrap();
    for member in community_members {
        validate_member(member)?;
    }

    Ok(())
}

const MAX_BIO_LENGTH: usize = 180;

fn validate_member(member: Member) -> Result<(), String> {
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

    Ok(())
}
