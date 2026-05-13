use anyhow::{bail, Result};
use regex::Regex;
use std::{
    ffi::OsStr,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
};

use crate::{code_block_definition::CodeBlockDefinition, hidden_ranges::get_hidden_ranges};

/// Checks the given directory, returning a list of unformatted files.
pub fn check(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut unformatted_files = Vec::new();

    visit_dir_md_files(dir, &mut |path| {
        println!("- {path:?}");

        let src = fs::read_to_string(path)?;

        let formatted = format_file(&src)?;

        // Check if the formatted version is different from the original.
        if src != formatted {
            // Changes were made! Write that down.
            unformatted_files.push(path.to_path_buf());
        }

        Ok(())
    })?;

    Ok(unformatted_files)
}

/// Formats the given directory, automatically adding `hide_lines` annotations to code blocks.
pub fn format(dir: &Path) -> Result<()> {
    visit_dir_md_files(dir, &mut |path| {
        println!("- {path:?}");

        let src = fs::read_to_string(path)?;

        let formatted = format_file(&src)?;

        // Overwrite file with formatted contents.
        fs::write(path, formatted)?;

        Ok(())
    })?;

    Ok(())
}

/// Calls function `cb` for every file recursively found within the folder `dir`.
fn visit_dir_md_files(dir: &Path, cb: &mut dyn FnMut(&Path) -> Result<()>) -> Result<()> {
    if !dir.is_dir() {
        bail!(
            "Tried visiting the path {:?} that was not a directory.",
            dir
        );
    }

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_dir() {
            visit_dir_md_files(&path, cb)?;
        } else if let Some(ext) = path.extension().and_then(OsStr::to_str) {
            if ext.to_lowercase() == "md" {
                cb(&path)?;
            }
        }
    }

    Ok(())
}

fn format_file(src: &str) -> Result<String> {
    let mut contents = String::with_capacity(src.len());
    let mut rust_block: Vec<String> = vec![];
    let mut is_rust = false;

    let mut inside_code_block = false;

    // Find a code block delimiter and optionally the first specified language
    let code_block_delim = Regex::new(r"\s*```(\w*)")?;

    for line in src.lines() {
        let code_block_delim_match = code_block_delim.captures(line).and_then(|cap| cap.get(1));
        let is_code_block_delim = code_block_delim_match.is_some();

        if !inside_code_block && is_code_block_delim {
            let lang = code_block_delim_match.unwrap().as_str();
            if lang == "rust" || lang == "rs" {
                is_rust = true;
            }

            inside_code_block = true;
        } else if inside_code_block && is_code_block_delim {
            inside_code_block = false;
        }

        // Pass through non-rust code block contents and contents outside of code blocks.
        if !is_rust {
            writeln!(&mut contents, "{line}")?;
            continue;
        }

        rust_block.push(String::from(line));

        if inside_code_block {
            continue;
        }

        // Process the `rust `code block
        let code = &rust_block[1..rust_block.len() - 1];
        let real_hidden_ranges = get_hidden_ranges(code);
        let mut definition = CodeBlockDefinition::new(&rust_block[0]).unwrap();

        match definition.get_hidden_ranges() {
            Some(annotation_hidden_ranges) => {
                if *annotation_hidden_ranges != real_hidden_ranges {
                    definition.set_hidden_ranges(real_hidden_ranges);
                }
            }
            None => {
                if !real_hidden_ranges.is_empty() {
                    definition.set_hidden_ranges(real_hidden_ranges);
                }
            }
        }

        // Rewrite code block Zola annotations
        rust_block[0] = definition.into_string();

        // Write code block
        writeln!(&mut contents, "{}", rust_block.join("\n"))?;

        // Reset state
        inside_code_block = false;
        rust_block = vec![];
        is_rust = false;
    }

    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn add_missing_annotation() {
        let markdown = indoc! {r#"
            ```rust
            # test
            # test 2
            fn not_hidden() {

            }
            # test 3
            #[derive(Component)]
            struct A;
            # #[derive(Component)]
            struct B;
            ```
        "#};

        let contents = format_file(markdown);

        assert_eq!(
            contents.unwrap(),
            indoc! {r#"
                ```rust,hide_lines=1-2 6 9
                # test
                # test 2
                fn not_hidden() {

                }
                # test 3
                #[derive(Component)]
                struct A;
                # #[derive(Component)]
                struct B;
                ```
            "#}
        );
    }

    #[test]
    fn update_wrong_annotation() {
        let markdown = indoc! {r#"
            ```rust,hide_lines=2-3 7
            # test
            # test 2
            fn not_hidden() {

            }
            # test 3
            ```
        "#};

        let contents = format_file(markdown);

        assert_eq!(
            contents.unwrap(),
            indoc! {r#"
                ```rust,hide_lines=1-2 6
                # test
                # test 2
                fn not_hidden() {

                }
                # test 3
                ```
            "#}
        );
    }

    #[test]
    fn remove_annotation() {
        let markdown = indoc! {r#"
            ```rust,hide_lines=2-3 7
            fn not_hidden() {

            }
            ```
        "#};

        let contents = format_file(markdown);

        assert_eq!(
            contents.unwrap(),
            indoc! {r#"
                ```rust
                fn not_hidden() {

                }
                ```
            "#}
        );
    }

    #[test]
    fn indented() {
        let markdown = r#"
    ```rust
    # test
    # test 2
    fn not_hidden() {

    }
    # test 3
    #[derive(Component)]
    struct A;
    # #[derive(Component)]
    struct B;
    ```
"#;

        let contents = format_file(markdown);

        assert_eq!(
            contents.unwrap(),
            r#"
    ```rust,hide_lines=1-2 6 9
    # test
    # test 2
    fn not_hidden() {

    }
    # test 3
    #[derive(Component)]
    struct A;
    # #[derive(Component)]
    struct B;
    ```
"#
        );
    }
}
