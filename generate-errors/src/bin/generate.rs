use generate_errors::{parse_errors, ErrorCode, FrontMatterErrorCode, Section};
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

fn main() -> io::Result<()> {
    let errors_dir = std::env::args()
        .nth(1)
        .expect("First argument should specify the errors directory");
    let content_dir = std::env::args()
        .nth(2)
        .expect("Second argument should specify the content directory");
    let _ = fs::create_dir(content_dir.clone());
    let errors_root_section = parse_errors(&errors_dir)?;

    errors_root_section.write(Path::new(&content_dir), Path::new(""), 0)
}

trait FrontMatterWriter {
    fn write(&self, root_path: &Path, current_path: &Path, weight: usize) -> io::Result<()>;
}

#[derive(Serialize)]
struct FrontMatterSection {
    title: String,
    sort_by: String,
    template: Option<String>,
    weight: usize,
    extra: FrontMatterSectionExtra,
}

#[derive(Serialize)]
struct FrontMatterSectionExtra {
    header_message: Option<String>,
    sort_order_reversed: bool,
}

impl From<&Section> for FrontMatterSection {
    fn from(section: &Section) -> Self {
        FrontMatterSection {
            title: section.name.clone(),
            sort_by: "weight".to_string(),
            template: section.template.clone(),
            weight: section.order.unwrap_or(0),
            extra: section.into(),
        }
    }
}

impl From<&Section> for FrontMatterSectionExtra {
    fn from(section: &Section) -> Self {
        FrontMatterSectionExtra {
            header_message: section.header.clone(),
            sort_order_reversed: section.sort_order_reversed,
        }
    }
}

impl FrontMatterWriter for Section {
    fn write(&self, root_path: &Path, current_path: &Path, weight: usize) -> io::Result<()> {
        let section_path = current_path.join(self.name.to_ascii_lowercase());
        let path = root_path.join(&section_path);
        fs::create_dir(path.clone())?;

        let mut frontmatter = FrontMatterSection::from(self);
        if self.order.is_none() {
            frontmatter.weight = weight;
        }

        let mut file = File::create(path.join("_index.md"))?;
        file.write_all(
            format!(
                r#"+++
{}
+++
"#,
                toml::to_string(&frontmatter).unwrap(),
            )
            .as_bytes(),
        )?;

        for (i, content) in self.content.iter().enumerate() {
            content.write(root_path, &section_path, i)?
        }
        Ok(())
    }
}

impl FrontMatterWriter for ErrorCode {
    fn write(&self, root_path: &Path, current_path: &Path, weight: usize) -> io::Result<()> {
        let path = root_path.join(&current_path);

        let mut frontmatter = FrontMatterErrorCode::from(self);
        frontmatter.weight = weight;

        let mut file = File::create(path.join(format!("{}.md", self.code)))?;
        file.write_all(
            format!(
                r#"+++
{}
+++
{}"#,
                toml::to_string(&frontmatter).unwrap(),
                self.content
            )
            .as_bytes(),
        )?;

        Ok(())
    }
}
