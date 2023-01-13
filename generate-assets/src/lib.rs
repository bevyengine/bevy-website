use anyhow::{bail, Context};
use cratesio_dbdump_csvtab::CratesIODumpLoader;
use github_client::GithubClient;
use gitlab_client::GitlabClient;
use serde::Deserialize;
use std::{fs, path::PathBuf, str::FromStr};

pub mod github_client;
pub mod gitlab_client;

type CratesIoDb = cratesio_dbdump_csvtab::rusqlite::Connection;

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Asset {
    pub name: String,
    pub link: String,
    pub description: String,
    pub order: Option<usize>,
    pub image: Option<String>,
    pub licenses: Option<Vec<String>>,
    pub bevy_versions: Option<Vec<String>>,

    // this field is not read from the toml file
    #[serde(skip)]
    pub original_path: Option<PathBuf>,
}

impl Asset {
    /// Parses a license string separated with OR into a Vec<String>
    fn set_license(&mut self, license: Option<String>) {
        if self.licenses.is_some() {
            return;
        }
        if let Some(license) = license {
            let licenses = license
                .split(" OR ")
                .map(|x| x.trim().to_string())
                .collect();
            self.licenses = Some(licenses);
        }
    }

    fn set_bevy_version(&mut self, version: Option<String>) {
        if self.bevy_versions.is_some() {
            return;
        }
        if let Some(version) = version {
            self.bevy_versions = Some(vec![version]);
        }
    }
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

fn visit_dirs(
    dir: PathBuf,
    section: &mut Section,
    crates_io_db: Option<&CratesIoDb>,
    github_client: Option<&GithubClient>,
    gitlab_client: Option<&GitlabClient>,
) -> anyhow::Result<()> {
    if dir.is_file() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.file_name().unwrap() == ".git" || path.file_name().unwrap() == ".github" {
            continue;
        }
        if path.is_dir() {
            let folder = path.file_name().unwrap();
            let (order, sort_order_reversed) = if path.join("_category.toml").exists() {
                let from_file: toml::Value =
                    toml::de::from_str(&fs::read_to_string(path.join("_category.toml")).unwrap())
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
            visit_dirs(
                path.clone(),
                &mut new_section,
                crates_io_db,
                github_client,
                gitlab_client,
            )?;
            section.content.push(AssetNode::Section(new_section));
        } else {
            if path.file_name().unwrap() == "_category.toml"
                || path.extension().expect("file must have an extension") != "toml"
            {
                continue;
            }

            let mut asset: Asset = toml::from_str(&fs::read_to_string(&path).unwrap())?;
            asset.original_path = Some(path);

            if let Err(err) =
                get_extra_metadata(&mut asset, crates_io_db, github_client, gitlab_client)
            {
                // We don't want to stop execution here
                eprintln!("Failed to get metadata for {}", asset.name);
                eprintln!("ERROR: {err:?}");
            }

            section.content.push(AssetNode::Asset(asset));
        }
    }

    Ok(())
}

pub fn parse_assets(
    asset_dir: &str,
    crates_io_db: Option<&CratesIoDb>,
    github_client: Option<&GithubClient>,
    gitlab_client: Option<&GitlabClient>,
) -> anyhow::Result<Section> {
    let mut asset_root_section = Section {
        name: "Assets".to_string(),
        content: vec![],
        template: Some("assets.html".to_string()),
        header: Some("Assets".to_string()),
        order: None,
        sort_order_reversed: false,
    };
    visit_dirs(
        PathBuf::from_str(asset_dir).unwrap(),
        &mut asset_root_section,
        crates_io_db,
        github_client,
        gitlab_client,
    )?;
    Ok(asset_root_section)
}

/// Tries to get bevy supported version and license information from various external sources
fn get_extra_metadata(
    asset: &mut Asset,
    crates_io_db: Option<&CratesIoDb>,
    github_client: Option<&GithubClient>,
    gitlab_client: Option<&GitlabClient>,
) -> anyhow::Result<()> {
    println!("Getting extra metadata for {}", asset.name);

    let url = url::Url::parse(&asset.link)?;
    let segments = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();
    let metadata = match url.host_str() {
        Some("crates.io") if crates_io_db.is_some() => {
            if let Some(db) = crates_io_db {
                let crate_name = segments[1];
                Some(get_metadata_from_crates_io_db(db, crate_name)?)
            } else {
                None
            }
        }
        Some("github.com") => {
            if let Some(client) = github_client {
                let username = segments[0];
                let repository_name = segments[1];
                Some(get_metadata_from_github(client, username, repository_name)?)
            } else {
                None
            }
        }
        Some("gitlab.com") => {
            if let Some(client) = gitlab_client {
                let repository_name = segments[1];
                Some(get_metadata_from_gitlab(client, repository_name)?)
            } else {
                None
            }
        }
        None => None,
        _ => bail!("Unknown host: {}", asset.link),
    };

    if let Some((license, version)) = metadata {
        asset.set_license(license);
        asset.set_bevy_version(version);
    }

    Ok(())
}

fn get_metadata_from_github(
    client: &GithubClient,
    username: &str,
    repository_name: &str,
) -> anyhow::Result<(Option<String>, Option<String>)> {
    let content = client
        .get_content(username, repository_name, "Cargo.toml")
        .context("Failed to get Cargo.toml from github")?;

    let cargo_manifest = toml::from_str::<cargo_toml::Manifest>(&content)?;
    Ok((
        get_license(&cargo_manifest),
        get_bevy_version(&cargo_manifest),
    ))
}

fn get_metadata_from_gitlab(
    client: &GitlabClient,
    repository_name: &str,
) -> anyhow::Result<(Option<String>, Option<String>)> {
    let search_result = client.search_project_by_name(repository_name)?;

    let repo = search_result
        .first()
        .context("Failed to find gitlab repo")?;

    let content = client
        .get_content(repo.id, &repo.default_branch, "Cargo.toml")
        .context("Failed to get Cargo.toml from gitlab")?;

    let cargo_manifest = toml::from_str::<cargo_toml::Manifest>(&content)?;
    Ok((
        get_license(&cargo_manifest),
        get_bevy_version(&cargo_manifest),
    ))
}

/// Gets the bevy version from the dependency list
/// Returns the version number if available.
/// If is is a git dependency, return either "main" or "git" for anything that isn't "main".
fn get_bevy_dependency_version(dep: &cargo_toml::Dependency) -> Option<String> {
    match dep {
        cargo_toml::Dependency::Simple(version) => Some(version.to_string()),
        cargo_toml::Dependency::Detailed(detail) => {
            if let Some(version) = &detail.version {
                Some(version.to_string())
            } else if detail.git.is_some() {
                if detail.branch == Some(String::from("main")) {
                    Some(String::from("main"))
                } else {
                    Some(String::from("git"))
                }
            } else {
                None
            }
        }
    }
}

/// Gets the license from a Cargo.toml file
/// Tries to emulate crates.io behaviour
fn get_license(cargo_manifest: &cargo_toml::Manifest) -> Option<String> {
    // Get the license from the package information
    if let Some(cargo_toml::Package {
        license,
        license_file,
        ..
    }) = &cargo_manifest.package
    {
        if let Some(license) = license {
            Some(license.clone())
        } else {
            license_file.as_ref().map(|_| String::from("non-standard"))
        }
    } else {
        None
    }
}

/// Find any dep that starts with bevy and get the version
/// This makes sure to handle all the bevy_* crates
fn get_bevy_version(cargo_manifest: &cargo_toml::Manifest) -> Option<String> {
    cargo_manifest
        .dependencies
        .keys()
        .find(|k| k.starts_with("bevy"))
        .and_then(|key| {
            cargo_manifest
                .dependencies
                .get(key)
                .and_then(get_bevy_dependency_version)
        })
}

/// Downloads the crates.io database dump and open a connection to the db
pub fn prepare_crates_db() -> anyhow::Result<CratesIoDb> {
    let cache_dir = {
        let mut current_dir = std::env::current_dir()?;
        current_dir.push("data");
        current_dir
    };

    if cache_dir.exists() {
        println!("Using crates.io data dump cache from: {:?}", cache_dir);
    } else {
        println!("Downloading crates.io data dump");
    }

    Ok(CratesIODumpLoader::default()
        .tables(&["crates", "dependencies", "versions"])
        .preload(true)
        .update()?
        .open_db()?)
}

/// Gets the required metadata from the crates.io database dump
fn get_metadata_from_crates_io_db(
    db: &CratesIoDb,
    crate_name: &str,
) -> anyhow::Result<(Option<String>, Option<String>)> {
    if let Ok(metadata) = get_metadata_from_db_by_crate_name(db, crate_name) {
        Ok(metadata)
    } else if let Ok(metadata) =
        get_metadata_from_db_by_crate_name(db, &crate_name.replace('_', "-"))
    {
        Ok(metadata)
    } else {
        bail!("Failed to get data from crates.io db for {crate_name}")
    }
}

fn get_metadata_from_db_by_crate_name(
    db: &CratesIoDb,
    crate_name: &str,
) -> anyhow::Result<(Option<String>, Option<String>)> {
    if let Some(Ok((_, _, license, _, deps))) =
        &cratesio_dbdump_lookup::get_rev_dependency(db, crate_name, "bevy")?.first()
    {
        let version = deps
            .as_ref()
            .ok()
            .and_then(|deps| deps.first())
            .map(|(version, _)| version.clone());
        Ok((Some(license.clone()), version))
    } else {
        bail!("Not found in crates.io db: {crate_name}")
    }
}
