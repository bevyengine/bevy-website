use anyhow::Result;
use regex::Regex;
use std::{
    ffi::OsStr,
    fmt::Write,
    fs::{self, DirEntry, File},
    io::{self, BufRead},
    path::Path,
};

use crate::{code_block_definition::CodeBlockDefinition, hidden_ranges::get_hidden_ranges};

pub fn run(dir: &Path) -> Result<()> {
    visit_dir_md_files(dir, &|entry| {
        println!("{:?}", entry.path());

        // Load and format file annotations
        let file = File::open(entry.path())?;
        let file_size = file.metadata().unwrap().len().try_into().unwrap();
        let contents = format_file(
            io::BufReader::new(file)
                .lines()
                .map(|line| line.map_err(anyhow::Error::from)),
            file_size,
        )?;

        // Rewrite file
        fs::write(entry.path(), contents)?;

        Ok(())
    })
}

fn visit_dir_md_files(dir: &Path, cb: &dyn Fn(&DirEntry) -> Result<()>) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            visit_dir_md_files(&path, cb)?;
        } else if let Some(ext) = path.extension().and_then(OsStr::to_str) {
            if ext.to_lowercase() == "md" {
                cb(&entry)?;
            }
        }
    }

    Ok(())
}

fn format_file(reader: impl Iterator<Item = Result<String>>, file_size: usize) -> Result<String> {
    let mut contents = String::with_capacity(file_size);
    let mut rust_block: Vec<String> = vec![];
    let mut is_rust = false;

    let mut inside_code_block = false;

    // Find a code block delimiter and optionally the first specified language
    let code_block_delim = Regex::new(r"\s*```(\w*)")?;

    for line in reader {
        let line = line?;

        let code_block_delim_match = code_block_delim.captures(&line).and_then(|cap| cap.get(1));
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
            writeln!(&mut contents, "{}", &line)?;
            continue;
        }

        rust_block.push(line);

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
        writeln!(&mut contents, "{}", &rust_block.join("\n"))?;

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

    fn lines_iter(code: &str) -> impl Iterator<Item = Result<String>> + '_ {
        code.split('\n').map(|line| Ok(String::from(line)))
    }

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

        let contents = format_file(lines_iter(markdown), markdown.len());

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

        let contents = format_file(lines_iter(markdown), markdown.len());

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

        let contents = format_file(lines_iter(markdown), markdown.len());

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

        let contents = format_file(lines_iter(markdown), markdown.len());

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
