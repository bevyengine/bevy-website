use regex::Regex;
use serde::Serialize;
use std::fs::read_to_string;
use std::{fs, io, path::PathBuf, str::FromStr};

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub content: Vec<ErrorCode>,
    pub template: Option<String>,
    pub header: Option<String>,
    pub order: Option<usize>,
    pub sort_order_reversed: bool,
}

#[derive(Debug, Clone)]
pub struct ErrorCode {
    pub code: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct FrontMatterErrorCode {
    pub title: String,
    pub weight: usize,
}

impl From<&ErrorCode> for FrontMatterErrorCode {
    fn from(asset: &ErrorCode) -> Self {
        FrontMatterErrorCode {
            title: asset.code.clone(),
            weight: 0,
        }
    }
}

fn visit_dirs(dir: PathBuf, section: &mut Section) -> io::Result<()> {
    assert!(dir.is_dir(), "The path to the errors is not a directory");
    let error_code_pattern = Regex::new(r"B[0-9]{4}\.md").unwrap();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            let file_name = path
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap();
            if !error_code_pattern.is_match(&file_name) {
                continue;
            }

            let error_code = read_to_string(path.clone())?;

            let code = file_name.trim_end_matches(".md").to_owned();
            section.content.push(ErrorCode {
                content: error_code
                    .trim_start_matches(&format!("# {}", code.clone()))
                    .replace("```rust,*", "```rust")
                    .lines()
                    .map(|line| {
                        // throw away `should_panic` and `no_run` to fix code highlighting
                        if line.starts_with("```rust,") {
                            "```rust"
                        } else {
                            line
                        }
                    })
                    .collect::<Vec<&str>>()
                    .join("\n"),
                code,
            });
        }
    }

    Ok(())
}

pub fn parse_errors(errors_dir: &str) -> io::Result<Section> {
    let mut errors_root_section = Section {
        name: "Errors".to_string(),
        content: vec![],
        template: Some("errors.html".to_string()),
        header: Some("Errors".to_string()),
        order: None,
        sort_order_reversed: false,
    };
    visit_dirs(
        PathBuf::from_str(&errors_dir).unwrap(),
        &mut errors_root_section,
    )?;
    Ok(errors_root_section)
}

#[cfg(test)]
mod test_regex {
    use regex::Regex;

    #[test]
    fn error_code_pattern() {
        let error_code_pattern = Regex::new(r"B[0-9]{4}\.md").unwrap();
        assert!(error_code_pattern.is_match("B0000.md"));
        assert!(error_code_pattern.is_match("B9999.md"));

        assert!(!error_code_pattern.is_match("E0000.md"));
        assert!(!error_code_pattern.is_match("00000.md"));
        assert!(!error_code_pattern.is_match("B0000.txt"));
        assert!(!error_code_pattern.is_match("Cargo.toml"));
        assert!(!error_code_pattern.is_match("README.md"));
    }
}
