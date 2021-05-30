use regex::Regex;

use generate_assets::*;

fn main() -> Result<(), ()> {
    let asset_dir = std::env::args().nth(1).unwrap();
    if let Ok(asset_root_section) = parse_assets(&asset_dir) {
        if asset_root_section.validate() {
            Ok(())
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

trait AssetValidator {
    fn validate(&self) -> bool;
}

#[derive(Debug)]
enum AssetError {
    DescriptionMissing,
    DescriptionTooLong,
    DescriptionWithFormatting,
}

impl AssetValidator for Section {
    fn validate(&self) -> bool {
        let mut valid = true;
        for content in self.content.iter() {
            if !content.validate() {
                valid = false;
            }
        }
        valid
    }
}

impl AssetValidator for AssetNode {
    fn validate(&self) -> bool {
        match self {
            AssetNode::Section(content) => content.validate(),
            AssetNode::Asset(content) => content.validate(),
        }
    }
}

impl AssetValidator for Asset {
    fn validate(&self) -> bool {
        let mut valid = true;
        if let Some(description) = self.description.as_ref() {
            if description.len() > 100 {
                valid = false;
                println!("{:50} - {:?}", self.name, AssetError::DescriptionTooLong);
            }
            if has_forbidden_formatting(description) {
                valid = false;
                println!(
                    "{:50} - {:?}",
                    self.name,
                    AssetError::DescriptionWithFormatting
                );
            }
        } else {
            valid = false;
            println!("{:50} - {:?}", self.name, AssetError::DescriptionMissing);
        }

        return valid;
    }
}

fn has_forbidden_formatting(string: &str) -> bool {
    if string.contains('\n') {
        return true;
    }
    if string.starts_with('#') {
        return true;
    }
    let re = Regex::new(r"\[(.+)\]\(((?:/|https?://)[\w\d./?=#]+)\)").unwrap();
    if re.is_match(string) {
        return true;
    }

    return false;
}
