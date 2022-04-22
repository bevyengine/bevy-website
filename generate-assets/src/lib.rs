use serde::Deserialize;
use std::{fs, io, path::PathBuf, str::FromStr};

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Asset {
    pub name: String,
    pub link: String,
    pub description: String,
    pub order: Option<usize>,
    pub image: Option<String>,

    // this field is not read from the toml file
    #[serde(skip)]
    pub original_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub content: Vec<AssetNode>,
    pub template: Option<String>,
    pub header: Option<String>,
    pub order: Option<usize>,
    pub sort_order_reversed: bool,
}

#[derive(Debug, Clone)]
pub enum AssetNode {
    Section(Section),
    Asset(Asset),
}
impl AssetNode {
    pub fn name(&self) -> String {
        match self {
            AssetNode::Section(content) => content.name.clone(),
            AssetNode::Asset(content) => content.name.clone(),
        }
    }
    pub fn order(&self) -> usize {
        match self {
            AssetNode::Section(content) => content.order.unwrap_or(99999),
            AssetNode::Asset(content) => content.order.unwrap_or(99999),
        }
    }
}

fn visit_dirs(dir: PathBuf, section: &mut Section) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.file_name().unwrap() == ".git" || path.file_name().unwrap() == ".github" {
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
                    || path.extension().expect("file must have an extension") != "toml"
                {
                    continue;
                }
                let mut asset: Asset =
                    toml::de::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
                asset.original_path = Some(path);
                section.content.push(AssetNode::Asset(asset));
            }
        }
    }
    Ok(())
}

pub fn parse_assets(asset_dir: &str) -> io::Result<Section> {
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
    Ok(asset_root_section)
}
