use rand::{prelude::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, prelude::*},
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Deserialize, Debug, Clone)]
struct Asset {
    name: String,
    link: String,
    description: Option<String>,
    order: Option<usize>,
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
}

impl From<&Asset> for FrontMatterAsset {
    fn from(asset: &Asset) -> Self {
        FrontMatterAsset {
            title: asset.name.clone(),
            description: asset.description.clone().unwrap_or_default(),
            weight: asset.order.unwrap_or(0),
            extra: FrontMatterAssetExtra {
                link: asset.link.clone(),
            },
        }
    }
}

impl Asset {
    fn write(&self, current_path: &str, weight: usize) -> io::Result<()> {
        let path = Path::new(&current_path);

        let mut frontmatter = FrontMatterAsset::from(self);
        if self.order.is_none() {
            frontmatter.weight = weight;
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

#[derive(Debug, Clone)]
struct Section {
    name: String,
    content: Vec<AssetNode>,
    template: Option<String>,
    header: Option<String>,
    order: Option<usize>,
    sort_order_reversed: bool,
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

impl Section {
    fn write(&self, current_path: &str, weight: usize) -> io::Result<()> {
        let path = Path::new(&current_path).join(self.name.to_ascii_lowercase());
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
            content.write(path.to_str().unwrap(), i)?
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum AssetNode {
    Section(Section),
    Asset(Asset),
}
impl AssetNode {
    fn write(&self, current_path: &str, weight: usize) -> io::Result<()> {
        match self {
            AssetNode::Section(content) => content.write(current_path, weight),
            AssetNode::Asset(content) => content.write(current_path, weight),
        }
    }
    fn name(&self) -> String {
        match self {
            AssetNode::Section(content) => content.name.clone(),
            AssetNode::Asset(content) => content.name.clone(),
        }
    }
    fn order(&self) -> usize {
        match self {
            AssetNode::Section(content) => content.order.unwrap_or(99999),
            AssetNode::Asset(content) => content.order.unwrap_or(99999),
        }
    }
}

fn main() -> io::Result<()> {
    let asset_dir = std::env::args().nth(1).unwrap();
    let content_dir = std::env::args().nth(2).unwrap();
    let _ = fs::create_dir(content_dir.clone());
    let mut asset_root_section = Section {
        name: "Assets".to_string(),
        content: vec![],
        template: Some("assets.html".to_string()),
        header: Some("Assets".to_string()),
        order: None,
        sort_order_reversed: false,
    };
    visit_dirs(
        PathBuf::from_str(&asset_dir).unwrap(),
        &mut asset_root_section,
    )?;

    asset_root_section.write(&content_dir, 0)?;
    Ok(())
}

fn visit_dirs(dir: PathBuf, section: &mut Section) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.file_name().unwrap() == ".git" {
                continue;
            }
            if path.is_dir() {
                let folder = path.file_name().unwrap();
                let (order, sort_order_reversed) = if path.join("_category.toml").exists() {
                    let from_file: toml::Value = toml::de::from_str(
                        &fs::read_to_string(path.join("_category.toml")).unwrap(),
                    )
                    .unwrap();
                    (
                        from_file
                            .get("order")
                            .and_then(|v| v.as_integer())
                            .map(|v| v as usize),
                        from_file
                            .get("sort_order_reversed")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false),
                    )
                } else {
                    (None, false)
                };
                let mut new_section = Section {
                    name: folder.to_str().unwrap().to_string(),
                    content: vec![],
                    template: None,
                    header: None,
                    order,
                    sort_order_reversed,
                };
                visit_dirs(path.clone(), &mut new_section)?;
                section.content.push(AssetNode::Section(new_section));
            } else {
                if path.file_name().unwrap() == "_category.toml"
                    || path.extension().unwrap() != "toml"
                {
                    continue;
                }
                let asset: Asset = toml::de::from_str(&fs::read_to_string(path).unwrap()).unwrap();
                section.content.push(AssetNode::Asset(asset));
            }
        }
    }
    Ok(())
}
