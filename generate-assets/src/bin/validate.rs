use std::{fmt::Display, path::Path};

use anyhow::{anyhow, Context, Result};
use image::{io::Reader as ImageReader, DynamicImage};
use regex::Regex;

use generate_assets::*;

const MAX_DESCRIPTION_LENGTH: usize = 100;
const MAX_IMAGE_WIDTH: u32 = 1000;
const MAX_IMAGE_HEIGHT: u32 = 1000;
const MAX_IMAGE_BYTES: u64 = 1_000_000;
const ALLOWED_IMAGE_EXTENSIONS: &[&str] = &["gif", "jpg", "jpeg", "png", "webp"];

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

    eprintln!();
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
            writeln!(f, "  {}", error)?;
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
    ImageInvalid,
    ImageFileSizeTooLarge,
    ImageDimensionsTooLarge,
}
impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::DescriptionTooLong => write!(
                f,
                "Description must be at most {} chars in length.",
                MAX_DESCRIPTION_LENGTH
            ),
            ValidationError::DescriptionWithFormatting => {
                write!(f, "Description must not contain formatting.")
            }
            ValidationError::ImageInvalidLink => write!(f, "Image file not found."),
            ValidationError::ImageInvalidExtension => write!(f, "Image extension not allowed"),
            ValidationError::ImageInvalid => write!(f, "Image file is invalid or corrupt."),
            ValidationError::ImageFileSizeTooLarge => {
                write!(f, "Image file must be at most {} bytes.", MAX_IMAGE_BYTES)
            }
            ValidationError::ImageDimensionsTooLarge => write!(
                f,
                "Image dimensions must not exceed {}x{} px.",
                MAX_IMAGE_WIDTH, MAX_IMAGE_HEIGHT
            ),
        }
    }
}

trait AssetValidator {
    fn validate(&self) -> Vec<Result<(), AssetError>>;
}

impl AssetValidator for Section {
    fn validate(&self) -> Vec<Result<(), AssetError>> {
        self.content
            .iter()
            .flat_map(|content| content.validate())
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

            if let Some(extension) = image_path.extension().and_then(|ext| ext.to_str()) {
                if !ALLOWED_IMAGE_EXTENSIONS.contains(&extension) {
                    errors.push(ValidationError::ImageInvalidExtension);
                }
            } else {
                errors.push(ValidationError::ImageInvalidExtension)
            }

            errors.append(&mut validate_image(&image_path));
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

fn open_image(path: &Path) -> Result<DynamicImage, ValidationError> {
    let size = path
        .metadata()
        .map_err(|_| ValidationError::ImageInvalidLink)?
        .len();

    if size > MAX_IMAGE_BYTES {
        return Err(ValidationError::ImageFileSizeTooLarge);
    }

    let img = ImageReader::open(path)
        .map_err(|_| ValidationError::ImageInvalidLink)?
        .decode()
        .map_err(|_| ValidationError::ImageInvalid)?;

    Ok(img)
}

fn validate_image(path: &Path) -> Vec<ValidationError> {
    let mut errors = vec![];

    let img = match open_image(path) {
        Ok(img) => img,
        Err(err) => {
            errors.push(err);
            return errors;
        }
    };

    if img.width() > MAX_IMAGE_WIDTH || img.height() > MAX_IMAGE_HEIGHT {
        errors.push(ValidationError::ImageDimensionsTooLarge);
    }

    errors
}
