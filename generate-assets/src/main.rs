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
            weight: 0,
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
        frontmatter.weight = weight;

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
}

#[derive(Serialize)]
struct FrontMatterSection {
    title: String,
    sort_by: String,
    template: Option<String>,
    weight: usize,
    extra: Option<FrontMatterSectionExtra>,
}

#[derive(Serialize)]
struct FrontMatterSectionExtra {
    header_message: String,
}

impl From<&Section> for FrontMatterSection {
    fn from(section: &Section) -> Self {
        FrontMatterSection {
            title: section.name.clone(),
            sort_by: "weight".to_string(),
            template: section.template.clone(),
            weight: 0,
            extra: section
                .header
                .clone()
                .map(|header_message| FrontMatterSectionExtra { header_message }),
        }
    }
}

impl Section {
    fn write(
        &self,
        current_path: &str,
        weight: usize,
        manual_priorities: &[&str],
    ) -> io::Result<()> {
        let path = Path::new(&current_path).join(self.name.to_ascii_lowercase());
        fs::create_dir(path.clone())?;

        let mut frontmatter = FrontMatterSection::from(self);
        frontmatter.weight = weight;

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
        sorted_section.sort_by(|a, b| a.name().partial_cmp(b.name()).unwrap());
        for manual_priority in manual_priorities.iter().rev() {
            if let Some(index) = sorted_section
                .iter()
                .position(|a| a.name() == *manual_priority)
            {
                let asset = sorted_section.remove(index);
                sorted_section.insert(0, asset);
            }
        }

        let mut randomized_assets = vec![];
        for content in self.content.iter() {
            if let AssetNode::Asset(asset) = content {
                randomized_assets.push(AssetNode::Asset(asset.clone()));
            }
        }
        randomized_assets.shuffle(&mut thread_rng());

        for (i, content) in sorted_section
            .iter()
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
            AssetNode::Section(content) => content.write(current_path, weight, &[]),
            AssetNode::Asset(content) => content.write(current_path, weight),
        }
    }
    fn name(&self) -> &str {
        match self {
            AssetNode::Section(content) => &content.name,
            AssetNode::Asset(content) => &content.name,
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
    };
    visit_dirs(
        PathBuf::from_str(&asset_dir).unwrap(),
        &mut asset_root_section,
    )?;

    asset_root_section.write(&content_dir, 0, &["Learning", "Plugins and Crates"])?;
    Ok(())
}

fn visit_dirs(dir: PathBuf, section: &mut Section) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let folder = path.file_name().unwrap();
                let mut new_section = Section {
                    name: folder.to_str().unwrap().to_string(),
                    content: vec![],
                    template: None,
                    header: None,
                };
                visit_dirs(path.clone(), &mut new_section)?;
                section.content.push(AssetNode::Section(new_section));
            } else {
                let asset: Asset =
                    toml::de::from_str(&fs::read_to_string(entry.path()).unwrap()).unwrap();
                section.content.push(AssetNode::Asset(asset));
            }
        }
    }
    Ok(())
}
