use regex::Regex;

use generate_assets::*;

fn main() -> Result<(), ()> {
    let asset_dir = std::env::args().nth(1).unwrap();
    if let Ok(asset_root_section) = parse_assets(&asset_dir, None, None, None) {
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
    DescriptionTooLong,
    DescriptionWithFormatting,
    ImageInvalidLink,
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
        if self.description.len() > 100 {
            valid = false;
            println!("{:50} - {:?}", self.name, AssetError::DescriptionTooLong);
        }
        if has_forbidden_formatting(&self.description) {
            valid = false;
            println!(
                "{:50} - {:?}",
                self.name,
                AssetError::DescriptionWithFormatting
            );
        }
        if let Some(image) = self.image.as_ref() {
            if image.starts_with('.')
                || image.starts_with('/')
                || image.starts_with("http")
                || !(image.ends_with(".gif")
                    || image.ends_with(".jpeg")
                    || image.ends_with(".jpg")
                    || image.ends_with(".png"))
            {
                valid = false;
                println!("{:50} - {:?}", self.name, AssetError::ImageInvalidLink);
            }
        }

        valid
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

    false
}
