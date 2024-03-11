use anyhow::anyhow;
use regex::Regex;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

/// Gets the unordered content of the error pages
/// supplied by user via
/// a path to a local Bevy git repo.
pub fn get_error_pages(bevy_repo_path: &Path) -> anyhow::Result<HashMap<String, String>> {
    if !bevy_repo_path.try_exists()? {
        return Err(anyhow!("The path ({bevy_repo_path:?}) is invalid"));
    }

    let mut error_page_paths: Vec<PathBuf> = vec![];

    let entries = bevy_repo_path.read_dir()?;
    let regex = Regex::new(r"B[0-9]{4}")?;

    for entry in entries {
        // You can't propagate a value multiple times
        // due to values moving, so it has to be put
        // in a variable to be used multiple times.
        let entry = entry?;

        if entry.metadata()?.is_dir() {
            continue;
        }

        if entry
            .file_name()
            .to_str()
            .is_some_and(|value| regex.is_match(value))
        {
            error_page_paths.push(entry.path());
        }
    }

    let mut results_map: HashMap<String, String> = HashMap::new();

    for path in error_page_paths {
        let file_name: String = path
            .file_name()
            .ok_or(anyhow!("An error page path has an invalid file stem"))?
            .to_string_lossy()
            .into_owned();
        let content = fs::read(path)?;
        results_map.insert(file_name, String::from_utf8(content)?);
    }

    Ok(results_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_page_content() {
        let error_content: anyhow::Result<HashMap<String, String>> =
            get_error_pages(Path::new("./bevy/errors"));
        assert!(error_content.is_ok());
    }
}
