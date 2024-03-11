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

/// Writes a valid docs section to contain
/// the error pages in.
///
/// The content folder passed should be
/// the Zola content folder.
pub fn write_section(content_folder_path: &Path) -> anyhow::Result<()> {
    let errors_folder_path = content_folder_path.join("learn/errors");
    // make sure the output folder exists
    fs::create_dir_all(&errors_folder_path)?;

    let section_content = r#"+++
title = "Errors"
template = "docs-section.html"
page_template = "docs-page.html"
redirect_to = "/learn/errors/introduction"
+++
"#;

    let introduction_content = r#"+++
title = "Introduction"
[extra]
weight = 0
+++

These pages document Bevy's error codes for the _current release_.

In case you are looking for the latest error codes from Bevy's main branch, you can find them in the [repository](<https://github.com/bevyengine/bevy/tree/main/errors>). 
"#;

    fs::write(errors_folder_path.join("_index.md"), section_content)?;
    fs::write(
        errors_folder_path.join("introduction.md"),
        introduction_content,
    )?;

    Ok(())
}

pub fn write_pages(
    content_folder_path: &Path,
    pages: HashMap<String, String>,
) -> anyhow::Result<()> {
    let errors_folder_path = content_folder_path.join("learn/errors");
    // make sure the output folder exists
    fs::create_dir_all(&errors_folder_path)?;

    // make the keys ordered so that
    // we know the weights for the pages
    let mut keys: Vec<&String> = pages.keys().collect();
    keys.sort_unstable();

    for (index, key) in keys.iter().enumerate() {
        let page_content = format!(
            r#"+++
title = "{}"
[extra]
weight = {}
+++

{}"#,
            key.strip_suffix(".md").unwrap_or(key),
            // since the introduction page takes the
            // zeroth position we need to treat the keys
            // like they're one indexed to not have
            // conflicting weights which have
            // undefined behavior.
            index + 1,
            pages
                .get(*key)
                .ok_or(anyhow!("The page content for {key} isn't valid!"))?
        );

        fs::write(errors_folder_path.join(key.to_lowercase()), page_content)?;
    }

    Ok(())
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

    #[test]
    fn test_write_section() {
        // fake content folder
        let content_path = Path::new("content");

        let result: anyhow::Result<()> = write_section(content_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_write_pages() {
        //fake content folder
        let content_path = Path::new("content");
        let pages_content =
            get_error_pages(Path::new("./bevy/errors")).expect("Page content should be valid");

        let result: anyhow::Result<()> = write_pages(content_path, pages_content);
        assert!(result.is_ok());
    }
}
