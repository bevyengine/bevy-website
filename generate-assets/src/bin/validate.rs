use std::fmt::Display;

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

    let results = asset_root_section.validate();

    let errors: Vec<_> = results.iter().filter_map(|r| r.as_ref().err()).collect();

    if errors.is_empty() {
        return Ok(());
    }

    for error in &errors {
        eprintln!("{}", error);
    }

    Err(anyhow!("{} asset(s) are invalid.", errors.len()))
}

#[derive(Debug)]
struct AssetError {
    asset_name: String,
    errors: Vec<ValidationError>,
}
impl Display for AssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.asset_name)?;
        for error in &self.errors {
            write!(f, "  {:?}", error)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum ValidationError {
    DescriptionTooLong,
    DescriptionWithFormatting,
    ImageInvalidLink,
    ImageInvalidExtension,
}

trait AssetValidator {
    fn validate(&self) -> Vec<Result<(), AssetError>>;
}

impl AssetValidator for Section {
    fn validate(&self) -> Vec<Result<(), AssetError>> {
        self.content
            .iter()
            .map(|content| content.validate())
            .flatten()
            .collect()
    }
}

impl AssetValidator for AssetNode {
    fn validate(&self) -> Vec<Result<(), AssetError>> {
        match self {
            AssetNode::Section(content) => content.validate(),
            AssetNode::Asset(content) => content.validate(),
        }
    }
}

impl AssetValidator for Asset {
    fn validate(&self) -> Vec<Result<(), AssetError>> {
        let mut errors = vec![];

        if self.description.len() > MAX_DESCRIPTION_LENGTH {
            errors.push(ValidationError::DescriptionTooLong);
        }

        if has_forbidden_formatting(&self.description) {
            errors.push(ValidationError::DescriptionWithFormatting);
        }

        if let Some(image) = self.image.as_ref() {
            let mut image_path = self.original_path.clone().unwrap();
            image_path.pop();
            image_path.push(image);

            if !image_path.is_file() {
                errors.push(ValidationError::ImageInvalidLink);
            }

            if let Some(extension) = image_path.extension().and_then(|ext| ext.to_str()) {
                if !["gif", "jpg", "jpeg", "png", "webp"].contains(&extension) {
                    errors.push(ValidationError::ImageInvalidExtension);
                }
            } else {
                errors.push(ValidationError::ImageInvalidExtension)
            }
        }

        if errors.is_empty() {
            vec![Ok(())]
        } else {
            vec![Err(AssetError {
                asset_name: self.name.clone(),
                errors,
            })]
        }
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
