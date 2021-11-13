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
    if !dir.is_dir() {
        // Todo: after the 0.6 release, remove this if statement
        // For now we will allow this to be able to point to the `latest` branch (0.5)
        // which does not yet include error codes
        return Ok(());
    }
    assert!(dir.is_dir(), "The path to the errors is not a directory");
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.file_name().unwrap() == ".git" || path.file_name().unwrap() == ".github" {
            continue;
        }
        if !path.is_dir() {
            if path.extension().unwrap() != "md" {
                continue;
            }

            let error_code = read_to_string(path.clone())?;

            let code = path
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap()
                .trim_end_matches(".md")
                .to_owned();
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
