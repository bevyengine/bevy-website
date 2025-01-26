use anyhow::{anyhow, bail};
use regex::Regex;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

/// Gets the unordered content of the error pages
/// supplied by user via
/// a path to the directory containing
/// the original Bevy error files.
pub fn get_error_pages(bevy_errors_path: &Path) -> anyhow::Result<HashMap<String, String>> {
    // Guard clause that determines
    // if a path exists and is valid
    // and not a broken symbolic link.
    // Otherwise it errors and returns.
    if !bevy_errors_path.try_exists()? {
        bail!("The path ({bevy_errors_path:?}) is invalid");
    }

    let mut error_page_paths: Vec<PathBuf> = vec![];

    let entries = bevy_errors_path.read_dir()?;
    // Matches Bevy error codes, such as B0001, B0002, etc.
    let regex = Regex::new(r"B[0-9]{4}")?;

    for entry in entries {
        // Propagates any errors or issues
        // regarding reading the directory entry
        // (either file or directory).
        //
        // This is propagated, or unwrapped, here
        // because it can't be unwrapped or propagated
        // more than once
        // otherwise causing move semantics issues.
        // (So, don't try to minimize lines of code by
        // propagating this multiple times rather than once here).
        let entry = entry?;

        if entry.metadata()?.is_dir() {
            continue;
        }

        // Only adds files that follow the
        // Bevy error code format:
        // E.g. B0000, B0001, B0002, ..., B0030, ..., B9999.
        if entry
            .file_name()
            .to_str()
            .is_some_and(|value| regex.is_match(value))
        {
            error_page_paths.push(entry.path());
        }
    }

    let mut results_map: HashMap<String, String> = HashMap::new();

    let regex = Regex::new(r"# B[0-9]{4}")?;
    for path in error_page_paths {
        let file_name: String = path
            .file_name()
            .ok_or(anyhow!("An error page path has an invalid file stem"))?
            .to_string_lossy()
            .into_owned();
        let content = fs::read_to_string(path)?;
        // The error pages already have a header built-in
        // but Zola provides its own title header
        // so we need to remove this for proper formatting
        let regex_content = regex.replace(content.as_str(), "");

        // Code blocks will be invalid unless
        // the annotations are before the language
        // like `should_panic,rust` or `no_run,rust`.
        let mut content = String::new();
        for line in regex_content.lines() {
            // Ensure we only operate on
            // Rust code blocks.
            if !line.starts_with("```rust") {
                content.push_str(line);
                content.push('\n');
                continue;
            }

            let annotations = line
                .strip_prefix("```")
                .ok_or(anyhow!(
                    "Failed to find start of code block to strip from string"
                ))?
                .split(',');

            let mut line: String = String::from("```");

            // Add all annotations other than rust
            // to the content first to avoid the issue.
            for annotation in annotations {
                if annotation == "rust" {
                    continue;
                }

                line.push_str(annotation);
                line.push(',');
            }
            line.push_str("rust");

            content.push_str(&line);
            content.push('\n');
        }

        results_map.insert(file_name, content.to_string());
    }

    Ok(results_map)
}

/// Writes a valid docs section to contain
/// the error pages in.
///
/// The output path passed should be the folder
/// you want the Zola pages to be written / output.
pub fn write_section(output_path: &Path) -> anyhow::Result<()> {
    let errors_folder_path = output_path.join("errors");
    // Make sure the output folder exists
    fs::create_dir_all(&errors_folder_path)?;

    const SECTION_CONTENT: &str = r#"+++
title = "Errors"
template = "docs.html"
page_template = "docs.html"
redirect_to = "/learn/errors/introduction"
+++
"#;

    const INTRODUCTION_CONTENT: &str = r#"+++
title = "Introduction"
[extra]
weight = 0
+++

These pages document Bevy's error codes for the _current release_.

In case you are looking for the latest error codes from Bevy's main branch, you can find them in the [Bevy engine repository](<https://github.com/bevyengine/bevy/tree/main/errors>). 
"#;

    fs::write(errors_folder_path.join("_index.md"), SECTION_CONTENT)?;
    fs::write(
        errors_folder_path.join("introduction.md"),
        INTRODUCTION_CONTENT,
    )?;

    Ok(())
}

pub fn write_pages(output_path: &Path, pages: HashMap<String, String>) -> anyhow::Result<()> {
    let errors_folder_path = output_path.join("errors");
    // Make sure the output folder exists
    fs::create_dir_all(&errors_folder_path)?;

    // Make the keys ordered so that
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
            // Extracts error code from the file name
            // and uses it as the title.
            // Otherwise just uses file name
            // for the title.
            key.strip_suffix(".md").unwrap_or(key),
            // Since the introduction page takes the
            // zeroth position we need to treat the keys
            // like they're one indexed to not have
            // conflicting weights which have
            // undefined behavior.
            index + 1,
            pages.get(*key).ok_or(anyhow!(
                "The page content for {key} doesn't exist in the HashMap!"
            ))?
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
        let error_content = get_error_pages(Path::new("./bevy/errors"));
        assert!(error_content.is_ok());
    }

    #[test]
    fn test_write_section() {
        // Fake content folder
        // named uniquely to avoid
        // parallel test failures.
        let output_path = Path::new("section_content/learn");

        let result: anyhow::Result<()> = write_section(output_path);
        assert!(result.is_ok());

        // Clean up after tests
        fs::remove_dir_all("section_content").unwrap();
    }

    #[test]
    fn test_write_pages() {
        // Fake content folder
        // named uniquely to avoid
        // parallel test failures.
        let output_path = Path::new("pages_content/learn");
        let pages_content =
            get_error_pages(Path::new("./bevy/errors")).expect("Page content should be valid");

        let result: anyhow::Result<()> = write_pages(output_path, pages_content);
        assert!(result.is_ok());

        // Clean up after tests
        fs::remove_dir_all("pages_content").unwrap();
    }
}
