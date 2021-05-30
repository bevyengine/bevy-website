use rand::{prelude::SliceRandom, thread_rng};
use serde::Serialize;
use std::{
    fs::{self, File},
    io::{self, prelude::*},
    path::Path,
};

use generate_assets::*;

fn main() -> io::Result<()> {
    let asset_dir = std::env::args().nth(1).unwrap();
    let content_dir = std::env::args().nth(2).unwrap();
    let _ = fs::create_dir(content_dir.clone());
    let asset_root_section = parse_assets(&asset_dir)?;

    asset_root_section.write(&content_dir, "", 0)?;
    Ok(())
}

trait FrontMatterWriter {
    fn write(&self, root_path: &str, current_path: &str, weight: usize) -> io::Result<()>;
}

#[derive(Serialize)]
struct FrontMatterAsset {
    title: String,
    description: String,
    weight: usize,
    extra: FrontMatterAssetExtra,
}

#[derive(Serialize)]
struct FrontMatterAssetExtra {
    link: String,
    image: Option<String>,
}

impl From<&Asset> for FrontMatterAsset {
    fn from(asset: &Asset) -> Self {
        FrontMatterAsset {
            title: asset.name.clone(),
            description: asset.description.clone().unwrap_or_default(),
            weight: asset.order.unwrap_or(0),
            extra: FrontMatterAssetExtra {
                link: asset.link.clone(),
                image: asset.image.clone(),
            },
        }
    }
}

impl FrontMatterWriter for Asset {
    fn write(&self, root_path: &str, current_path: &str, weight: usize) -> io::Result<()> {
        let path = Path::new(root_path).join(&current_path);

        let mut frontmatter = FrontMatterAsset::from(self);
        if self.order.is_none() {
            frontmatter.weight = weight;
        }
        if let Some(file) = self.image.as_ref() {
            let image_file_path = path.join(file).to_str().map(|p| p.to_string());
            let image_file_link = Path::new(current_path)
                .join(file)
                .to_str()
                .map(|p| p.to_string());
            let original_image = self
                .original_path
                .as_ref()
                .unwrap()
                .clone()
                .with_file_name(file);

            frontmatter.extra.image = image_file_link.clone();
            let _ = fs::copy(original_image, image_file_path.unwrap());
        }

        let mut file = File::create(path.join(format!(
            "{}.md",
            self.name.to_ascii_lowercase().replace("/", "-")
        )))?;
        file.write_all(
            format!(
                r#"+++
{}
+++
"#,
                toml::to_string(&frontmatter).unwrap(),
            )
            .as_bytes(),
        )?;

        Ok(())
    }
}

impl FrontMatterWriter for AssetNode {
    fn write(&self, root_path: &str, current_path: &str, weight: usize) -> io::Result<()> {
        match self {
            AssetNode::Section(content) => content.write(root_path, current_path, weight),
            AssetNode::Asset(content) => content.write(root_path, current_path, weight),
        }
    }
}

#[derive(Serialize)]
struct FrontMatterSection {
    title: String,
    sort_by: String,
    template: Option<String>,
    weight: usize,
    extra: FrontMatterSectionExtra,
}

#[derive(Serialize)]
struct FrontMatterSectionExtra {
    header_message: Option<String>,
    sort_order_reversed: bool,
}

impl From<&Section> for FrontMatterSectionExtra {
    fn from(section: &Section) -> Self {
        FrontMatterSectionExtra {
            header_message: section.header.clone(),
            sort_order_reversed: section.sort_order_reversed,
        }
    }
}

impl From<&Section> for FrontMatterSection {
    fn from(section: &Section) -> Self {
        FrontMatterSection {
            title: section.name.clone(),
            sort_by: "weight".to_string(),
            template: section.template.clone(),
            weight: section.order.unwrap_or(0),
            extra: section.into(),
        }
    }
}

impl FrontMatterWriter for Section {
    fn write(&self, root_path: &str, current_path: &str, weight: usize) -> io::Result<()> {
        let section_path = Path::new(&current_path).join(self.name.to_ascii_lowercase());
        let path = Path::new(&root_path).join(&section_path);
        fs::create_dir(path.clone())?;

        let mut frontmatter = FrontMatterSection::from(self);
        if self.order.is_none() {
            frontmatter.weight = weight;
        }

        let mut file = File::create(path.join("_index.md"))?;
        file.write_all(
            format!(
                r#"+++
{}
+++
"#,
                toml::to_string(&frontmatter).unwrap(),
            )
            .as_bytes(),
        )?;

        let mut sorted_section = vec![];
        for content in self.content.iter() {
            if let AssetNode::Section(section) = content {
                sorted_section.push(AssetNode::Section(section.clone()));
            }
        }
        sorted_section.sort_by_key(|section| format!("{}-{}", section.order(), section.name()));

        let mut randomized_assets = vec![];
        let mut manually_sorted_assets = vec![];
        for content in self.content.iter() {
            if let AssetNode::Asset(asset) = content {
                if asset.order.is_some() {
                    manually_sorted_assets.push(content.clone());
                } else {
                    randomized_assets.push(content.clone());
                }
            }
        }
        manually_sorted_assets.sort_by_key(AssetNode::order);
        randomized_assets.shuffle(&mut thread_rng());

        for (i, content) in sorted_section
            .iter()
            .chain(manually_sorted_assets.iter())
            .chain(randomized_assets.iter())
            .enumerate()
        {
            content.write(root_path, section_path.to_str().unwrap(), i)?
        }
        Ok(())
    }
}
