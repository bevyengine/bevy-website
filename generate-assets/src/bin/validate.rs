use anyhow::{anyhow, Context, Result};
use regex::Regex;

use generate_assets::*;

const MAX_DESCRIPTION_LENGTH: usize = 100;

fn main() -> Result<()> {
    let asset_dir = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("Please specify the path to bevy-assets"))?;

    let asset_root_section =
        parse_assets(&asset_dir, None, None, None).with_context(|| "Parsing assets")?;
    let errors = asset_root_section.validate();

    if errors.is_empty() {
        return Ok(());
    }

    eprintln!("{} error(s).", errors.len());

    for error in errors.iter() {
        eprintln!("{:?}", error);
    }

    Err(anyhow!("One or more assets are invalid."))
}

trait AssetValidator {
    fn validate(&self) -> Vec<AssetError>;
}

#[derive(Debug)]
enum AssetError {
    DescriptionTooLong(String),
    DescriptionWithFormatting(String),
    ImageInvalidLink(String),
    ImageInvalidExtension(String),
}

impl AssetValidator for Section {
    fn validate(&self) -> Vec<AssetError> {
        self.content
            .iter()
            .map(|content| content.validate())
            .flatten()
            .collect()
    }
}

impl AssetValidator for AssetNode {
    fn validate(&self) -> Vec<AssetError> {
        match self {
            AssetNode::Section(content) => content.validate(),
            AssetNode::Asset(content) => content.validate(),
        }
    }
}

impl AssetValidator for Asset {
    fn validate(&self) -> Vec<AssetError> {
        let mut errors = vec![];

        if self.description.len() > MAX_DESCRIPTION_LENGTH {
            errors.push(AssetError::DescriptionTooLong(self.name.clone()));
        }

        if has_forbidden_formatting(&self.description) {
            errors.push(AssetError::DescriptionWithFormatting(self.name.clone()));
        }

        if let Some(image) = self.image.as_ref() {
            let mut image_path = self.original_path.clone().unwrap();
            image_path.pop();
            image_path.push(image);

            if !image_path.is_file() {
                errors.push(AssetError::ImageInvalidLink(self.name.clone()));
            }

            if let Some(extension) = image_path.extension().and_then(|ext| ext.to_str()) {
                if !["gif", "jpg", "jpeg", "png", "webp"].contains(&extension) {
                    errors.push(AssetError::ImageInvalidExtension(self.name.clone()));
                }
            } else {
                errors.push(AssetError::ImageInvalidExtension(self.name.clone()))
            }
        }

        errors
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
