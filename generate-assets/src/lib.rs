use anyhow::{bail, Context};
use cratesio_dbdump_csvtab::CratesIODumpLoader;
use github_client::GithubClient;
use gitlab_client::GitlabClient;
use serde::Deserialize;
use std::{fs, path::PathBuf, str::FromStr};

pub mod github_client;
pub mod gitlab_client;

type CratesIoDb = cratesio_dbdump_csvtab::rusqlite::Connection;

const OFFICIAL_BEVY_CRATE_PREFIX_RANGE_START: &str = "bevy";
const OFFICIAL_BEVY_CRATE_PREFIX_RANGE_END: &str = "bevz";

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

#[derive(Default)]
pub struct MetadataSource<'a> {
    pub crates_io_db: Option<&'a CratesIoDb>,
    pub github_client: Option<&'a GithubClient>,
    pub gitlab_client: Option<&'a GitlabClient>,
    pub bevy_crates: Option<Vec<String>>,
}

fn visit_dirs(
    dir: PathBuf,
    section: &mut Section,
    metadata_source: &MetadataSource,
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
            visit_dirs(path.clone(), &mut new_section, metadata_source)?;
            section.content.push(AssetNode::Section(new_section));
        } else {
            if path.file_name().unwrap() == "_category.toml"
                || path.extension().expect("file must have an extension") != "toml"
            {
                continue;
            }

            let mut asset: Asset = toml::from_str(&fs::read_to_string(&path).unwrap())?;
            asset.original_path = Some(path);

            if let Err(err) = get_extra_metadata(&mut asset, metadata_source) {
                // We don't want to stop execution here
                eprintln!("Failed to get metadata for {}", asset.name);
                eprintln!("ERROR: {err:?}");
            }

            section.content.push(AssetNode::Asset(asset));
        }
    }

    Ok(())
}

pub fn parse_assets<'a>(
    asset_dir: &str,
    mut metadata_source: MetadataSource,
) -> anyhow::Result<Section> {
    let mut asset_root_section = Section {
        name: "Assets".to_string(),
        content: vec![],
        template: Some("assets.html".to_string()),
        header: Some("Assets".to_string()),
        order: None,
        sort_order_reversed: false,
    };

    if let Some(db) = metadata_source.crates_io_db {
        if let Ok(bevy_crates) = get_official_bevy_crates_from_crates_io_db(db) {
            metadata_source.bevy_crates = Some(bevy_crates);
        }
    }

    visit_dirs(
        PathBuf::from_str(asset_dir).unwrap(),
        &mut asset_root_section,
        &metadata_source,
        //crates_io_db,
        //github_client,
        //gitlab_client,
    )?;
    Ok(asset_root_section)
}

/// Tries to get bevy supported version and license information from various external sources
fn get_extra_metadata(asset: &mut Asset, metadata_source: &MetadataSource) -> anyhow::Result<()> {
    println!("Getting extra metadata for {}", asset.name);

    let url = url::Url::parse(&asset.link)?;
    let segments = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();
    let metadata = match url.host_str() {
        Some("crates.io") => {
            if let Some(db) = metadata_source.crates_io_db {
                let crate_name = segments[1];
                Some(get_metadata_from_crates_io_db(db, crate_name)?)
            } else {
                None
            }
        }
        Some("github.com") => {
            if let Some(client) = metadata_source.github_client {
                let username = segments[0];
                let repository_name = segments[1];
                Some(get_metadata_from_github(
                    client,
                    username,
                    repository_name,
                    &metadata_source.bevy_crates,
                )?)
            } else {
                None
            }
        }
        Some("gitlab.com") => {
            if let Some(client) = metadata_source.gitlab_client {
                let repository_name = segments[1];
                Some(get_metadata_from_gitlab(
                    client,
                    repository_name,
                    &metadata_source.bevy_crates,
                )?)
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
    bevy_crates: &Option<Vec<String>>,
) -> anyhow::Result<(Option<String>, Option<String>)> {
    let content = client
        .get_content(username, repository_name, "Cargo.toml")
        .context("Failed to get Cargo.toml from github")?;

    let cargo_manifest = toml::from_str::<cargo_toml::Manifest>(&content)?;
    Ok((
        get_license(&cargo_manifest),
        get_bevy_version(&cargo_manifest, bevy_crates),
    ))
}

fn get_metadata_from_gitlab(
    client: &GitlabClient,
    repository_name: &str,
    bevy_crates: &Option<Vec<String>>,
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
        get_bevy_version(&cargo_manifest, bevy_crates),
    ))
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
        if let Some(cargo_toml::Inheritable::Set(license)) = license {
            Some(license.clone())
        } else {
            license_file.as_ref().map(|_| String::from("non-standard"))
        }
    } else {
        None
    }
}

/// Find any bevy dependency and get the corresponding bevy version from an asset's manifest
/// This makes sure to handle all the bevy_* crates
fn get_bevy_version(
    cargo_manifest: &cargo_toml::Manifest,
    bevy_crates: &Option<Vec<String>>,
) -> Option<String> {
    let search_range = OFFICIAL_BEVY_CRATE_PREFIX_RANGE_START.to_owned()
        ..OFFICIAL_BEVY_CRATE_PREFIX_RANGE_END.to_owned();

    // Tries to find an official bevy crate from the asset's dependencies
    let mut dependencies = cargo_manifest.dependencies.range(search_range.clone());
    let empty_vec = Vec::new();
    let bevy_crates = if let Some(bevy_crates) = bevy_crates {
        bevy_crates.iter()
    } else {
        empty_vec.iter()
    };

    let mut bevy_dependency =
        search_bevy_in_dependencies(dependencies.clone(), bevy_crates.clone());

    if bevy_dependency.is_none() {
        // Tries to find an official bevy crate from the asset's dev dependencies
        // An asset can indirectly depend on bevy through another crate, but would probably depend on bevy directly
        // for its examples, benchmarks or tests, in its dev dependencies
        let dev_dependencies = cargo_manifest.dev_dependencies.range(search_range.clone());
        bevy_dependency = search_bevy_in_dependencies(dev_dependencies, bevy_crates.clone());

        if bevy_dependency.is_none() {
            // Tries to find an official bevy crate from the asset's workspace dependencies
            if let Some(ref workspace) = cargo_manifest.workspace {
                let workspace_dependencies = workspace.dependencies.range(search_range);
                bevy_dependency = search_bevy_in_dependencies(workspace_dependencies, bevy_crates);
            }

            if bevy_dependency.is_none() {
                // If everything else fails, try to find any crate with a name starting with bevy
                // This can happen if the asset depends only on past or future bevy sub-crates,
                // or on third-party crates which name starts with bevy (might yield unaccurate results in that last case)
                bevy_dependency = dependencies.find_map(|(_, d)| get_bevy_dependency_version(d));
            }
        }
    }

    bevy_dependency
}

/// Seach the first official bevy crate found in a collection of dependencies and return its version.
/// If it was a bit more generic, this function could be called find_first_intersect_in_sorted_iterators.
/// Both dependencies and bevy_crates are assumed to be sorted (by key for dependencies, they are in this context),
/// and we find the first element that intersect both of them using that knowledge
fn search_bevy_in_dependencies<'a>(
    mut dependencies: std::collections::btree_map::Range<'a, String, cargo_toml::Dependency>,
    mut bevy_crates: std::slice::Iter<String>,
) -> Option<String> {
    let mut dependency = dependencies.next();
    let mut bevy_crate = bevy_crates.next();

    while !dependency.is_none() && !bevy_crate.is_none() {
        let dependency_name = dependency.unwrap().0;
        let bevy_crate_name = bevy_crate.unwrap();

        if dependency_name < bevy_crate_name {
            dependency = dependencies.next();
        } else {
            if dependency_name == bevy_crate_name {
                let dependency_version = get_bevy_dependency_version(dependency.unwrap().1);
                if dependency_version.is_some() {
                    return dependency_version;
                } else {
                    // In this case we found an official bevy crate but we couldn't get a version from it
                    dependency = dependencies.next();
                    bevy_crate = bevy_crates.next();
                }
            } else {
                bevy_crate = bevy_crates.next();
            }
        }
    }
    return None;
}

/// Gets the bevy version from the bevy dependency provided
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
        cargo_toml::Dependency::Inherited(_) => None,
    }
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

/// Gets at list of the official bevy crates from the crates.io database dump
fn get_official_bevy_crates_from_crates_io_db(db: &CratesIoDb) -> anyhow::Result<Vec<String>> {
    if let Ok(mut crates) = get_bevy_crates(db) {
        crates.sort();
        Ok(crates)
    } else {
        bail!("Problem fetching official bevy crates from crates.io")
    }
}

fn get_bevy_crates(
    db: &CratesIoDb,
) -> Result<Vec<String>, cratesio_dbdump_csvtab::rusqlite::Error> {
    let mut s =
        db.prepare_cached("SELECT name FROM crates WHERE homepage = ? AND repository = ?")?;
    let rows = s.query_and_then(
        [
            "https://bevyengine.org",
            "https://github.com/bevyengine/bevy",
        ],
        |r| -> Result<String, cratesio_dbdump_csvtab::rusqlite::Error> {
            Ok(r.get_unwrap::<_, String>(0))
        },
    )?;
    let mut bevy_crates = Vec::new();
    for bevy_crate in rows {
        bevy_crates.push(bevy_crate?);
    }
    Ok(bevy_crates)
}

#[cfg(test)]
mod tests {
    mod get_version {
        use super::super::*;

        use cargo_toml::{Dependency, Manifest};
        use std::collections::BTreeMap;

        fn get_manifest(
            dependencies: BTreeMap<String, Dependency>,
            dev_dependencies: BTreeMap<String, Dependency>,
            workspace_dependencies: BTreeMap<String, Dependency>,
        ) -> Manifest {
            #[allow(deprecated)]
            Manifest {
                package: Default::default(),
                workspace: Some(cargo_toml::Workspace {
                    members: Default::default(),
                    package: Default::default(),
                    default_members: Default::default(),
                    exclude: Default::default(),
                    metadata: Default::default(),
                    resolver: Default::default(),
                    dependencies: workspace_dependencies,
                }),
                dependencies,
                dev_dependencies,
                build_dependencies: Default::default(),
                target: Default::default(),
                features: Default::default(),
                replace: Default::default(),
                patch: Default::default(),
                lib: Default::default(),
                profile: Default::default(),
                badges: Default::default(),
                bin: Default::default(),
                bench: Default::default(),
                test: Default::default(),
                example: Default::default(),
            }
        }

        fn get_bevy_crates() -> Option<Vec<String>> {
            Some(vec!["bevy".to_string(), "bevy_transform".to_string()])
        }

        #[test]
        fn from_no_dependency() {
            let dependencies = BTreeMap::new();
            let dev_dependencies = BTreeMap::new();
            let workspace_dependencies = BTreeMap::new();

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version(&manifest, &get_bevy_crates());
            assert_eq!(version, None);
        }

        #[test]
        fn from_other_dependencies() {
            let mut dependencies = BTreeMap::new();
            let mut dev_dependencies = BTreeMap::new();
            let mut workspace_dependencies = BTreeMap::new();

            dependencies.insert(
                "other_first".to_string(),
                Dependency::Simple("0.10".to_string()),
            );
            dev_dependencies.insert(
                "other_second".to_string(),
                Dependency::Simple("0.10".to_string()),
            );
            workspace_dependencies.insert(
                "other_third".to_string(),
                Dependency::Simple("0.10".to_string()),
            );

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version(&manifest, &get_bevy_crates());
            assert_eq!(version, None);
        }

        #[test]
        fn from_main_crate() {
            let mut dependencies = BTreeMap::new();
            let dev_dependencies = BTreeMap::new();
            let workspace_dependencies = BTreeMap::new();

            dependencies.insert("bevy".to_string(), Dependency::Simple("0.10".to_string()));

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version(&manifest, &get_bevy_crates());
            assert_eq!(version, Some("0.10".to_string()));
        }

        #[test]
        fn from_sub_crate() {
            let mut dependencies = BTreeMap::new();
            let dev_dependencies = BTreeMap::new();
            let workspace_dependencies = BTreeMap::new();

            dependencies.insert(
                "bevy_transform".to_string(),
                Dependency::Simple("0.10".to_string()),
            );

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version(&manifest, &get_bevy_crates());
            assert_eq!(version, Some("0.10".to_string()));
        }

        #[test]
        fn from_dev_dependencies() {
            let dependencies = BTreeMap::new();
            let mut dev_dependencies = BTreeMap::new();
            let workspace_dependencies = BTreeMap::new();

            dev_dependencies.insert("bevy".to_string(), Dependency::Simple("0.10".to_string()));

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version(&manifest, &get_bevy_crates());
            assert_eq!(version, Some("0.10".to_string()));
        }

        #[test]
        fn from_workspace_dependencies() {
            let dependencies = BTreeMap::new();
            let dev_dependencies = BTreeMap::new();
            let mut workspace_dependencies = BTreeMap::new();

            workspace_dependencies
                .insert("bevy".to_string(), Dependency::Simple("0.10".to_string()));

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version(&manifest, &get_bevy_crates());
            assert_eq!(version, Some("0.10".to_string()));
        }

        #[test]
        fn from_third_party() {
            let mut dependencies = BTreeMap::new();
            let dev_dependencies = BTreeMap::new();
            let workspace_dependencies = BTreeMap::new();

            dependencies.insert(
                "bevy_third_party_crate_example".to_string(),
                Dependency::Simple("0.5".to_string()),
            );

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version(&manifest, &get_bevy_crates());
            // Note that this result is expected, but potentially wrong
            assert_eq!(version, Some("0.5".to_string()));
        }

        #[test]
        fn from_dependencies_ignore_third_party() {
            let mut dependencies = BTreeMap::new();
            let dev_dependencies = BTreeMap::new();
            let workspace_dependencies = BTreeMap::new();

            // Alphabetical order could matter in this example, "third" < "transform"
            dependencies.insert(
                "bevy_third_party_crate_example".to_string(),
                Dependency::Simple("0.5".to_string()),
            );
            dependencies.insert(
                "bevy_transform".to_string(),
                Dependency::Simple("0.10".to_string()),
            );

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version(&manifest, &get_bevy_crates());
            assert_eq!(version, Some("0.10".to_string()));
        }

        #[test]
        fn from_dev_dependencies_ignore_third_party() {
            let mut dependencies = BTreeMap::new();
            let mut dev_dependencies = BTreeMap::new();
            let workspace_dependencies = BTreeMap::new();

            dependencies.insert(
                "bevy_third_party_crate_example".to_string(),
                Dependency::Simple("0.5".to_string()),
            );
            dev_dependencies.insert("bevy".to_string(), Dependency::Simple("0.10".to_string()));

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version(&manifest, &get_bevy_crates());
            assert_eq!(version, Some("0.10".to_string()));
        }

        #[test]
        fn from_workspace_dependencies_ignore_third_party() {
            let mut dependencies = BTreeMap::new();
            let dev_dependencies = BTreeMap::new();
            let mut workspace_dependencies = BTreeMap::new();

            dependencies.insert(
                "bevy_third_party_crate_example".to_string(),
                Dependency::Simple("0.5".to_string()),
            );
            workspace_dependencies
                .insert("bevy".to_string(), Dependency::Simple("0.10".to_string()));

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version(&manifest, &get_bevy_crates());
            assert_eq!(version, Some("0.10".to_string()));
        }

        #[test]
        fn from_dev_dependencies_with_path_dependency() {
            let mut dependencies = BTreeMap::new();
            let mut dev_dependencies = BTreeMap::new();
            let workspace_dependencies = BTreeMap::new();

            dependencies.insert(
                "bevy".to_string(),
                Dependency::Detailed(cargo_toml::DependencyDetail {
                    path: Some("fake/path/to/crate".to_string()),
                    ..Default::default()
                }),
            );
            dev_dependencies.insert(
                "bevy_transform".to_string(),
                Dependency::Simple("0.10".to_string()),
            );

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version(&manifest, &get_bevy_crates());
            assert_eq!(version, Some("0.10".to_string()));
        }

        #[test]
        fn from_third_party_crate_with_path_dependency() {
            let mut dependencies = BTreeMap::new();
            let dev_dependencies = BTreeMap::new();
            let workspace_dependencies = BTreeMap::new();

            // Alphabetical order could matter in this example, "first" < "second"
            dependencies.insert(
                "bevy_first_third_party_crate".to_string(),
                Dependency::Detailed(cargo_toml::DependencyDetail {
                    path: Some("fake/path/to/crate".to_string()),
                    ..Default::default()
                }),
            );
            dependencies.insert(
                "bevy_second_third_party_crate".to_string(),
                Dependency::Simple("0.10".to_string()),
            );

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version(&manifest, &get_bevy_crates());
            assert_eq!(version, Some("0.10".to_string()));
        }

        #[test]
        fn from_third_party_with_no_official_bevy_crates() {
            let mut dependencies = BTreeMap::new();
            let mut dev_dependencies = BTreeMap::new();
            let mut workspace_dependencies = BTreeMap::new();

            dependencies.insert(
                "bevy_third_party_crate_example".to_string(),
                Dependency::Simple("0.5".to_string()),
            );
            dev_dependencies.insert("bevy".to_string(), Dependency::Simple("0.10".to_string()));
            workspace_dependencies
                .insert("bevy".to_string(), Dependency::Simple("0.10".to_string()));

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version(&manifest, &Some(vec![]));
            assert_eq!(version, Some("0.5".to_string()));
        }

        #[test]
        fn from_no_dependency_with_no_official_bevy_crates() {
            let mut dependencies = BTreeMap::new();
            let mut dev_dependencies = BTreeMap::new();
            let mut workspace_dependencies = BTreeMap::new();

            dependencies.insert("other".to_string(), Dependency::Simple("0.5".to_string()));
            dev_dependencies.insert("bevy".to_string(), Dependency::Simple("0.10".to_string()));
            workspace_dependencies
                .insert("bevy".to_string(), Dependency::Simple("0.10".to_string()));

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version(&manifest, &Some(vec![]));
            assert_eq!(version, None);
        }
    }
}
