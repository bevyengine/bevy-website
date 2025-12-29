use rand::{prelude::SliceRandom, rng};
use serde::Serialize;
use std::{
    fs::{self, File},
    io::{self, prelude::*},
    path::Path,
};

use generate_assets::{github_client::GithubClient, gitlab_client::GitlabClient, *};

fn main() -> anyhow::Result<()> {
    // Don't fail if file is not present, like in CI, just ignore it
    let _ = dotenv::dotenv();

    let asset_dir = std::env::args().nth(1).unwrap();
    let content_dir = std::env::args().nth(2).unwrap();

    let db = prepare_crates_db()?;

    let github_client = {
        // This should be configured in CI, but it's not mandatory if running locally
        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            Some(GithubClient::new(token))
        } else {
            println!("GITHUB_TOKEN not found, github links will be skipped");
            None
        }
    };

    let gitlab_client = {
        // This should be configured in CI, but it's not mandatory if running locally
        if let Ok(token) = std::env::var("GITLAB_TOKEN") {
            Some(GitlabClient::new(token))
        } else {
            println!("GITLAB_TOKEN not found, gitlab links will be skipped");
            Some(GitlabClient::new(String::from("")))
        }
    };

    let _ = fs::create_dir(content_dir.clone());
    let mut asset_root_section = parse_assets(
        &asset_dir,
        MetadataSource {
            crates_io_db: Some(&db),
            github_client: github_client.as_ref(),
            gitlab_client: gitlab_client.as_ref(),
            ..Default::default()
        },
    )?;

    let latest_bevy_version = get_latest_bevy_version(&db)?;

    sort_section(&mut asset_root_section.content, &latest_bevy_version);

    asset_root_section
        .write(Path::new(&content_dir), Path::new(""), 0)
        .expect("Failed to write assets section");
    Ok(())
}

/// Sort the assets in the section so that:
/// - Assets that have been manually assigned an order in `bevy-assets` are first
/// - Assets that are semver compatible with Bevy are next
/// - If all else is equal, sort randomly
fn sort_section(nodes: &mut [AssetNode], latest_bevy_version: &semver::Version) {
    for node in nodes.iter_mut() {
        if let AssetNode::Section(section) = node {
            sort_section(&mut section.content, latest_bevy_version);
        }
    }

    let mut to_sort = vec![];
    for node in nodes {
        let is_semver_compat = node_semver_compat_with(node, latest_bevy_version);

        let existing_order = match node {
            AssetNode::Asset(asset) => asset.order.unwrap_or(usize::MAX),
            _ => continue,
        };

        let random: u32 = rand::random();
        to_sort.push((node, existing_order, !is_semver_compat, random));
    }

    to_sort.sort_by_key(|sorts| (sorts.1, sorts.2, sorts.3));

    for (i, (node, _, _, _)) in to_sort.into_iter().enumerate() {
        let AssetNode::Asset(asset) = node else {
            continue;
        };

        asset.order = Some(i);
    }
}

fn node_semver_compat_with(node: &AssetNode, version: &semver::Version) -> bool {
    let AssetNode::Asset(asset) = node else {
        return false;
    };

    let Some(ver) = asset.bevy_versions.as_ref().and_then(|v| v.first()) else {
        return false;
    };

    let Ok(semver) = semver::VersionReq::parse(ver) else {
        return false;
    };

    semver.matches(version)
}

trait FrontMatterWriter {
    fn write(&self, root_path: &Path, current_path: &Path, weight: usize) -> io::Result<()>;
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
    licenses: Option<Vec<String>>,
    bevy_versions: Option<Vec<String>>,
    nsfw: Option<bool>,
}

impl From<&Asset> for FrontMatterAsset {
    fn from(asset: &Asset) -> Self {
        FrontMatterAsset {
            title: asset.name.clone(),
            description: asset.description.clone(),
            weight: asset.order.unwrap_or(0),
            extra: FrontMatterAssetExtra {
                link: asset.link.clone(),
                image: asset.image.clone(),
                licenses: asset.licenses.clone(),
                bevy_versions: asset.bevy_versions.clone(),
                nsfw: asset.nsfw,
            },
        }
    }
}

impl FrontMatterWriter for Asset {
    fn write(&self, root_path: &Path, current_path: &Path, weight: usize) -> io::Result<()> {
        let path = root_path.join(current_path);

        let mut frontmatter = FrontMatterAsset::from(self);
        if self.order.is_none() {
            frontmatter.weight = weight;
        }
        if let Some(file) = self.image.as_ref() {
            let image_file_path = path.join(file);
            let image_file_link = current_path.join(file);
            let original_image = self
                .original_path
                .as_ref()
                .unwrap()
                .clone()
                .with_file_name(file);

            frontmatter.extra.image = image_file_link.to_str().map(|link| link.to_string());
            let _ = fs::copy(original_image, image_file_path);
        }

        let formatted_path = path.join(format!(
            "{}.md",
            self.name
                .to_ascii_lowercase()
                .replace('/', "-")
                .replace(' ', "_")
                .replace(
                    |c: char| !c.is_ascii_alphanumeric() && !matches!(c, '-' | '_'),
                    ""
                )
        ));

        let mut file = File::create(formatted_path.clone())
            .unwrap_or_else(|err| panic!("Failed to create file at {:?}\n{}", formatted_path, err));

        file.write_all(
            format!(
                r#"+++
{}
+++
"#,
                toml::to_string(&frontmatter).unwrap(),
            )
            .as_bytes(),
        )
        .unwrap_or_else(|err| panic!("Failed to write at {:?}\n{}", formatted_path, err));

        Ok(())
    }
}

impl FrontMatterWriter for AssetNode {
    fn write(&self, root_path: &Path, current_path: &Path, weight: usize) -> io::Result<()> {
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
    fn write(&self, root_path: &Path, current_path: &Path, weight: usize) -> io::Result<()> {
        let section_path = current_path.join(self.name.to_ascii_lowercase());
        let path = root_path.join(&section_path);
        if !path.exists() {
            fs::create_dir(path.clone())
                .unwrap_or_else(|_| panic!("Failed to create dir {:?}", path));
        }

        let mut frontmatter = FrontMatterSection::from(self);
        if self.order.is_none() {
            frontmatter.weight = weight;
        }

        let mut file = File::create(path.join("_index.md"))
            .unwrap_or_else(|_| panic!("Failed to create _index.md at {:?}", path));
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
        randomized_assets.shuffle(&mut rng());

        for (i, content) in sorted_section
            .iter()
            .chain(manually_sorted_assets.iter())
            .chain(randomized_assets.iter())
            .enumerate()
        {
            content.write(root_path, &section_path, i)?;
        }
        Ok(())
    }
}
