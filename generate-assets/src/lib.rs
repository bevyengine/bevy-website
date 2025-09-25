use anyhow::{bail, Context};
use cratesio_dbdump_csvtab::rusqlite;
use cratesio_dbdump_csvtab::CratesIODumpLoader;
use github_client::GithubClient;
use gitlab_client::GitlabClient;
use serde::Deserialize;
use std::cmp::Ordering;
use std::{fs, path::PathBuf, str::FromStr};
use url::Url;

pub mod github_client;
pub mod gitlab_client;

type CratesIoDb = rusqlite::Connection;

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
    #[serde(rename = "crate")]
    pub crate_name: Option<String>,
    pub licenses: Option<Vec<String>>,
    pub bevy_versions: Option<Vec<String>>,
    pub nsfw: Option<bool>,

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

#[allow(clippy::large_enum_variant)]
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
/// Where to find metadata (bevy version and license) for assets.
pub struct MetadataSource<'a> {
    /// Connection to the crates.io database sqlite dump.
    pub crates_io_db: Option<&'a CratesIoDb>,
    /// Connection to Github API.
    pub github_client: Option<&'a GithubClient>,
    /// Connection to Gitlab API.
    pub gitlab_client: Option<&'a GitlabClient>,
    /// Official bevy crates names from crates.io DB dump, in lexigographic order.
    pub bevy_crates_names: Option<Vec<String>>,
    /// Prepared statement to retrieve metadata from crates.io.
    ///
    /// Initialized with [`get_metadata_from_cratesio_statement`] at the beginning
    /// of the algorithm, used by [`get_metadata_from_cratesio`] for each asset.
    pub get_metadata_from_cratesio_statement: Option<rusqlite::Statement<'a>>,
}

/// Entry point the algorithm to find [`Asset`] files inside [`Section`] folders,
/// parse asset files, and gather metadata information about assets from various external sources.
///
/// This initialises the root [`Section`], and initialize [`MetadataSource`] with
/// crates.io's database dump connection and information about official bevy crates.
pub fn parse_assets(
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
        let bevy_crates_ids = if let Ok((bevy_crates_names, bevy_crates_ids)) =
            get_official_bevy_crates_from_crates_io_db(db)
        {
            metadata_source.bevy_crates_names = Some(bevy_crates_names);
            Some(bevy_crates_ids)
        } else {
            None
        };
        metadata_source.get_metadata_from_cratesio_statement =
            Some(get_metadata_from_cratesio_statement(db, bevy_crates_ids)?);
    }

    visit_dirs(
        PathBuf::from_str(asset_dir).unwrap(),
        &mut asset_root_section,
        &mut metadata_source,
    )?;
    Ok(asset_root_section)
}

/// Recursive traversal of directories inside the cloned "Bevy Assets" project,
/// each directory is a [`Section`], configured inside the `_category.toml` file,
/// each other file with a `.toml` extension is an [`Asset`].
fn visit_dirs(
    dir: PathBuf,
    section: &mut Section,
    metadata_source: &mut MetadataSource,
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

/// Tries to get bevy supported version and license information from various external sources.
fn get_extra_metadata(
    asset: &mut Asset,
    metadata_source: &mut MetadataSource,
) -> anyhow::Result<()> {
    println!("Getting extra metadata for {}", asset.name);

    let url = match &asset.crate_name {
        Some(crate_name) => Url::parse(&format!("https://crates.io/crates/{crate_name}")),
        None => Url::parse(&asset.link),
    }?;
    let segments = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();

    let metadata = match url.host_str() {
        Some("crates.io") => {
            if let Some(ref mut statement) = metadata_source.get_metadata_from_cratesio_statement {
                let crate_name = segments[1];
                Some(get_metadata_from_crates_db(crate_name, statement)?)
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
                    &metadata_source.bevy_crates_names,
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
                    &metadata_source.bevy_crates_names,
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

/// Merge two licenses, get the combination of both of them.
fn merge_license(license1: Option<String>, license2: Option<String>) -> Option<String> {
    if license1.is_none() {
        return license2;
    }
    if license2.is_none() {
        return license1;
    }

    let license1 = license1.unwrap();
    let license2 = license2.unwrap();
    if license1.contains(&license2) {
        return Some(license1);
    }
    if license2.contains(&license1) {
        return Some(license2);
    }

    Some(license1 + " " + &license2)
}

/// Merge two versions, get the "maximum" of the two
/// TODO: normalize versions to be able to compare them
/// In the mean time this just returns version1 if it's Some
fn merge_version(version1: Option<String>, version2: Option<String>) -> Option<String> {
    if version1.is_some() {
        return version1;
    }
    version2
}

/// Gets metadata from a Github project.
///
/// This algorithm, in order:
/// - tries to get metadata from the root `Cargo.toml` file,
/// - if the license is missing, search the license of the project on Github,
/// - if metadata is missing, search all `Cargo.toml` files, then tries to get metadata
///   from all of them, until we have the information we need.
///
/// Note:
/// - The search call of the API has a tendency to return 403 errors after a few number
///   of calls. Assets that are at the "end" might not have correct metadata because of that.
/// - This algorithm tries to retain the "best" version and merge all licenses found.
/// - If a licence and version is found, it will stop searching, but the information
///   about the version and license could have gotten "better" by searching deper.
/// - Likewise, the project license is never checked if a license is provided in the root
///   `Cargo.toml` file.
fn get_metadata_from_github(
    client: &GithubClient,
    username: &str,
    repository_name: &str,
    bevy_crates: &Option<Vec<String>>,
) -> anyhow::Result<(Option<String>, Option<String>)> {
    let result = get_metadata_from_github_manifest(
        client,
        username,
        repository_name,
        bevy_crates,
        "Cargo.toml",
    );

    let (mut license, mut version) = match result {
        Ok(lic_ver) => lic_ver,
        Err(err) => {
            println!("Error getting metadata from root cargo file from github: {err}");
            (None, None)
        }
    };

    if license.is_none() {
        license = client.get_license(username, repository_name).ok();
    }

    if license.is_none() || version.is_none() {
        let cargo_files = match client.search_file(username, repository_name, "Cargo.toml") {
            Ok(cargo_files) => cargo_files,
            Err(err) => {
                println!("Error fetching cargo files from github: {err:#}");
                return Ok((license, version));
            }
        };

        let mut cargo_files = cargo_files
            .iter()
            //Exclude the root Cargo.toml, we already searched in it
            .filter(|f| f != &"Cargo.toml");

        let mut cargo_file = cargo_files.next();
        while (license.is_none() || version.is_none()) && cargo_file.is_some() {
            let cargo_file_path = cargo_file.unwrap();

            let result = get_metadata_from_github_manifest(
                client,
                username,
                repository_name,
                bevy_crates,
                cargo_file_path,
            );
            match result {
                Ok((new_license, new_version)) => {
                    (license, version) = (
                        merge_license(license, new_license),
                        merge_version(version, new_version),
                    );
                }
                Err(err) => {
                    println!("Error getting metadata from other cargo file from github: {err}");
                    return Ok((license, version));
                }
            }

            cargo_file = cargo_files.next();
        }
    }

    Ok((license, version))
}

/// Gets metadata from a `Cargo.toml` file in a Github project.
fn get_metadata_from_github_manifest(
    client: &GithubClient,
    username: &str,
    repository_name: &str,
    bevy_crates: &Option<Vec<String>>,
    path: &str,
) -> anyhow::Result<(Option<String>, Option<String>)> {
    let content = client
        .get_content(username, repository_name, path)
        .context("Failed to get Cargo.toml from github")?;

    let cargo_manifest = toml::from_str::<cargo_toml::Manifest>(&content)?;

    Ok((
        get_license(&cargo_manifest),
        get_bevy_version_from_manifest(&cargo_manifest, bevy_crates),
    ))
}

/// Gets metadata from a Gitlab project.
///
/// This algorithm only looks into the root `Cargo.toml` file.
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
        get_bevy_version_from_manifest(&cargo_manifest, bevy_crates),
    ))
}

/// Gets the license from a `Cargo.toml` file
/// Tries to emulate crates.io behavior.
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

/// Find any bevy dependency and get the corresponding bevy version from a `Cargo.toml` file.
///
/// This algorithm checks if a dependency to an official bevy crate is found, in order:
/// - in the (regular) dependencies,
/// - in the dev dependencies (used for examples, tests and benchmarks),
/// - in the workspace dependencies.
///
/// It doesn't go deeper if a version is already found.
fn get_bevy_version_from_manifest(
    cargo_manifest: &cargo_toml::Manifest,
    bevy_crates: &Option<Vec<String>>,
) -> Option<String> {
    let search_range = OFFICIAL_BEVY_CRATE_PREFIX_RANGE_START.to_owned()
        ..OFFICIAL_BEVY_CRATE_PREFIX_RANGE_END.to_owned();

    let dependencies = cargo_manifest.dependencies.range(search_range.clone());
    if let Some(bevy_crates) = bevy_crates {
        let bevy_crates = bevy_crates.iter();

        // Tries to find an official bevy crate from the asset's dependencies.
        let mut bevy_dependency =
            search_bevy_in_manifest_dependencies(dependencies.clone(), bevy_crates.clone());

        if bevy_dependency.is_none() {
            // Tries to find an official bevy crate from the asset's dev dependencies.
            // An asset can indirectly depend on bevy through another crate,
            // but would probably depend on bevy directly for its examples,
            // benchmarks or tests, in its dev dependencies.
            let dev_dependencies = cargo_manifest.dev_dependencies.range(search_range.clone());
            bevy_dependency =
                search_bevy_in_manifest_dependencies(dev_dependencies, bevy_crates.clone());

            if bevy_dependency.is_none() {
                // Tries to find an official bevy crate from the asset's workspace dependencies.
                if let Some(ref workspace) = cargo_manifest.workspace {
                    let workspace_dependencies = workspace.dependencies.range(search_range);
                    bevy_dependency =
                        search_bevy_in_manifest_dependencies(workspace_dependencies, bevy_crates);
                }
            }
        }

        bevy_dependency
    } else {
        None
    }
}

/// Search the first official bevy crate found in a collection of `Cargo.toml`
/// dependencies and return its version.
///
/// If it was a bit more generic, this function could be called `find_first_intersect_in_sorted_iterators`.
/// Both `dependencies` and `bevy_crates` are assumed to be sorted (by key for `dependencies`, they are in this context),
/// and we find the first element that intersect both of them using that knowledge.
fn search_bevy_in_manifest_dependencies(
    mut dependencies: std::collections::btree_map::Range<'_, String, cargo_toml::Dependency>,
    mut bevy_crates: std::slice::Iter<String>,
) -> Option<String> {
    let mut dependency = dependencies.next();
    let mut bevy_crate = bevy_crates.next();

    while dependency.is_some() && bevy_crate.is_some() {
        let dependency_name = dependency.unwrap().0;
        let bevy_crate_name = bevy_crate.unwrap();

        match dependency_name.cmp(bevy_crate_name) {
            Ordering::Less => dependency = dependencies.next(),
            Ordering::Equal => {
                let dependency_version =
                    get_bevy_manifest_dependency_version(dependency.unwrap().1);

                if dependency_version.is_some() {
                    return dependency_version;
                }

                // In this case we found an official bevy crate but we couldn't get a version from it
                dependency = dependencies.next();
                bevy_crate = bevy_crates.next();
            }
            Ordering::Greater => bevy_crate = bevy_crates.next(),
        }
    }

    None
}

/// Gets the bevy version from the `Cargo.toml` bevy dependency provided.
///
/// Returns the version number if available.
/// If is a git dependency, return either "main" or "git" for anything that isn't "main".
fn get_bevy_manifest_dependency_version(dep: &cargo_toml::Dependency) -> Option<String> {
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

/// Downloads the crates.io database dump and open a connection to the db.
pub fn prepare_crates_db() -> anyhow::Result<CratesIoDb> {
    let cache_dir = {
        let mut current_dir = std::env::current_dir()?;
        current_dir.push("data");
        current_dir
    };

    if cache_dir.exists() {
        println!("Using crates.io data dump cache from: {cache_dir:?}");
    } else {
        println!("Downloading crates.io data dump");
    }

    let db = CratesIODumpLoader::default()
        .tables(&["crates", "dependencies", "versions"])
        .preload(true)
        .update()?
        .open_db()?;

    db.execute_batch(
        "\
        CREATE INDEX IF NOT EXISTS versions_crate_id_index ON versions(crate_id);
        CREATE INDEX IF NOT EXISTS dependencies_crate_id_index ON dependencies(crate_id);
        CREATE INDEX IF NOT EXISTS crates_id_index ON crates(id);
        CREATE INDEX IF NOT EXISTS crates_name_index ON crates(name);
     ",
    )
    .expect("could not create crates.io database indices");

    Ok(db)
}

/// Gets metadata of a crate from the crates.io database dump.
///
/// If the crate is not found, retries with `-` instead of `_`.
fn get_metadata_from_crates_db(
    crate_name: &str,
    get_metadata_from_cratesio_statement: &mut rusqlite::Statement,
) -> anyhow::Result<(Option<String>, Option<String>)> {
    if let Ok(metadata) =
        get_metadata_from_crates_db_by_name(crate_name, get_metadata_from_cratesio_statement)
    {
        Ok(metadata)
    } else if let Ok(metadata) = get_metadata_from_crates_db_by_name(
        &crate_name.replace('_', "-"),
        get_metadata_from_cratesio_statement,
    ) {
        Ok(metadata)
    } else {
        bail!("Failed to get data from crates.io db for {crate_name}")
    }
}

/// Gets metadata of a crate from the crates.io database dump using the exact crate
/// name provided.
fn get_metadata_from_crates_db_by_name(
    crate_name: &str,
    get_metadata_from_cratesio_statement: &mut rusqlite::Statement,
) -> anyhow::Result<(Option<String>, Option<String>)> {
    if let Ok((license, version)) =
        get_metadata_from_cratesio(crate_name, get_metadata_from_cratesio_statement)
    {
        let license = if !license.is_empty() {
            Some(license)
        } else {
            None
        };

        Ok((license, version))
    } else {
        bail!("Not found in crates.io db: {crate_name}")
    }
}

/// Gets at list of the official bevy crates from the crates.io database dump,
/// in lexicographic order.
fn get_official_bevy_crates_from_crates_io_db(
    db: &CratesIoDb,
) -> anyhow::Result<(Vec<String>, Vec<String>)> {
    if let Ok(mut bevy_crates) = get_bevy_crates(db) {
        bevy_crates.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));
        Ok(bevy_crates.into_iter().unzip())
    } else {
        bail!("Problem fetching official bevy crates from crates.io")
    }
}

// Get official bevy crates name and ids from the crates.io database dump.
#[allow(clippy::let_and_return)]
fn get_bevy_crates(db: &CratesIoDb) -> Result<Vec<(String, String)>, rusqlite::Error> {
    let mut bevy_crates_statement = db.prepare(
        "\
            SELECT name, id \
            FROM crates \
            WHERE (homepage = ? OR homepage = ?)\
                AND repository = ?\
        ",
    )?;

    // Required let and return due to bevy_crates_statement not living long enough.
    let bevy_crates = bevy_crates_statement
        .query_and_then(
            [
                "https://bevy.org",
                "https://bevyengine.org",
                "https://github.com/bevyengine/bevy",
            ],
            |r| -> Result<(String, String), rusqlite::Error> {
                Ok((r.get_unwrap::<_, String>(0), r.get_unwrap::<_, String>(1)))
            },
        )?
        .collect();

    bevy_crates
}

/// Get the highest (according to semver) version of Bevy listed in the crates.io database
pub fn get_latest_bevy_version(db: &CratesIoDb) -> anyhow::Result<semver::Version> {
    let mut bevy_id_statement = db.prepare(
        "\
            SELECT id \
            FROM crates \
            WHERE name = 'bevy'\
        ",
    )?;

    let bevy_id: String = bevy_id_statement.query_row([], |row| row.get(0))?;

    let mut bevy_versions_statement = db.prepare(
        "\
            SELECT num \
            FROM versions \
            WHERE crate_id = ?\
        ",
    )?;

    let bevy_versions: Vec<semver::Version> = bevy_versions_statement
        .query_map([bevy_id], |r| r.get::<_, String>(0))?
        .filter_map(|r| semver::Version::parse(&r.ok()?).ok())
        .collect();

    bevy_versions
        .into_iter()
        .max()
        .context("Failed to retrieve Bevy versions from crates.io db")
}

/// Get a prepared statement to get license and version for a crate from the
/// crates.io database dump.
///
/// To be used later by [`get_metadata_from_cratesio`].
pub fn get_metadata_from_cratesio_statement(
    db: &CratesIoDb,
    bevy_crates_ids: Option<Vec<String>>,
) -> Result<rusqlite::Statement<'_>, rusqlite::Error> {
    let bevy_crates_ids = bevy_crates_ids.unwrap_or_default();

    db.prepare(&format!(
        "\
        SELECT last_version.license, dep.req \
        FROM ( \
            SELECT version_id, license, major, \
                CAST(SUBSTR(minor_and_patch,0,second_point) AS INTEGER) minor, \
                CAST(SUBSTR(minor_and_patch,second_point+1) AS INTEGER) patch \
            FROM ( \
                SELECT version_id, license, major, minor_and_patch, \
                    INSTR(minor_and_patch, '.') second_point \
                FROM ( \
                    SELECT version_id, license, \
                        CAST(SUBSTR(num,0,first_point) AS INTEGER) major, \
                        SUBSTR(num,first_point+1) minor_and_patch \
                    FROM ( \
                        SELECT v.id version_id, v.license license, v.num num, \
                            INSTR(v.num, '.') first_point \
                        FROM crates c \
                            INNER JOIN versions v ON c.id = v.crate_id \
                        WHERE c.name = ? \
                    ) \
                ) \
            ) \
            ORDER BY major DESC, minor DESC, patch DESC \
            LIMIT 1 \
        ) last_version \
            LEFT JOIN dependencies dep ON \
            ( \
                last_version.version_id = dep.version_id AND \
                dep.crate_id IN ({}) \
            ) \
        ORDER BY dep.kind \
        LIMIT 1\
        ",
        bevy_crates_ids.join(",")
    ))
}

/// Get license and bevy version for a crate from crates.io,
/// using the prepared statement provided by [`get_metadata_from_cratesio_statement`].
pub fn get_metadata_from_cratesio(
    crate_name: &str,
    get_metadata_from_cratesio_statement: &mut rusqlite::Statement,
) -> Result<(String, Option<String>), rusqlite::Error> {
    get_metadata_from_cratesio_statement.query_row(
        [crate_name],
        |r| -> Result<(String, Option<String>), rusqlite::Error> {
            Ok((
                r.get_unwrap::<_, String>(0),
                r.get_unwrap::<_, Option<String>>(1),
            ))
        },
    )
}

#[cfg(test)]
mod tests {
    mod get_bevy_version_from_manifest {
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
                    lints: Default::default(),
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
                lints: Default::default(),
            }
        }

        fn get_bevy_crates_names() -> Option<Vec<String>> {
            Some(vec!["bevy".to_string(), "bevy_transform".to_string()])
        }

        #[test]
        fn from_no_dependency() {
            let dependencies = BTreeMap::new();
            let dev_dependencies = BTreeMap::new();
            let workspace_dependencies = BTreeMap::new();

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version_from_manifest(&manifest, &get_bevy_crates_names());
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
            let version = get_bevy_version_from_manifest(&manifest, &get_bevy_crates_names());
            assert_eq!(version, None);
        }

        #[test]
        fn from_main_crate() {
            let mut dependencies = BTreeMap::new();
            let dev_dependencies = BTreeMap::new();
            let workspace_dependencies = BTreeMap::new();

            dependencies.insert("bevy".to_string(), Dependency::Simple("0.10".to_string()));

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version_from_manifest(&manifest, &get_bevy_crates_names());
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
            let version = get_bevy_version_from_manifest(&manifest, &get_bevy_crates_names());
            assert_eq!(version, Some("0.10".to_string()));
        }

        #[test]
        fn from_dev_dependencies() {
            let dependencies = BTreeMap::new();
            let mut dev_dependencies = BTreeMap::new();
            let workspace_dependencies = BTreeMap::new();

            dev_dependencies.insert("bevy".to_string(), Dependency::Simple("0.10".to_string()));

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version_from_manifest(&manifest, &get_bevy_crates_names());
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
            let version = get_bevy_version_from_manifest(&manifest, &get_bevy_crates_names());
            assert_eq!(version, Some("0.10".to_string()));
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
            let version = get_bevy_version_from_manifest(&manifest, &get_bevy_crates_names());
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
            let version = get_bevy_version_from_manifest(&manifest, &get_bevy_crates_names());
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
            let version = get_bevy_version_from_manifest(&manifest, &get_bevy_crates_names());
            assert_eq!(version, Some("0.10".to_string()));
        }

        #[test]
        fn from_dev_dependencies_with_path_dependency() {
            let mut dependencies = BTreeMap::new();
            let mut dev_dependencies = BTreeMap::new();
            let workspace_dependencies = BTreeMap::new();

            dependencies.insert(
                "bevy".to_string(),
                Dependency::Detailed(Box::new(cargo_toml::DependencyDetail {
                    path: Some("fake/path/to/crate".to_string()),
                    ..Default::default()
                })),
            );
            dev_dependencies.insert(
                "bevy_transform".to_string(),
                Dependency::Simple("0.10".to_string()),
            );

            let manifest = get_manifest(dependencies, dev_dependencies, workspace_dependencies);
            let version = get_bevy_version_from_manifest(&manifest, &get_bevy_crates_names());
            assert_eq!(version, Some("0.10".to_string()));
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
            let version = get_bevy_version_from_manifest(&manifest, &Some(vec![]));
            assert_eq!(version, None);
        }
    }
}
