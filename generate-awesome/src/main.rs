use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, prelude::*},
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Deserialize, Debug, Clone)]
struct Awesome {
    name: String,
    link: String,
    description: Option<String>,
}

#[derive(Serialize)]
struct FrontMatterAwesome {
    title: String,
    description: String,
    weight: usize,
    extra: FrontMatterAwesomeExtra,
}

#[derive(Serialize)]
struct FrontMatterAwesomeExtra {
    link: String,
}

impl From<&Awesome> for FrontMatterAwesome {
    fn from(awesome: &Awesome) -> Self {
        FrontMatterAwesome {
            title: awesome.name.clone(),
            description: awesome.description.clone().unwrap_or_default(),
            weight: 0,
            extra: FrontMatterAwesomeExtra {
                link: awesome.link.clone(),
            },
        }
    }
}

impl Awesome {
    fn write(&self, current_path: &str, weight: usize) -> io::Result<()> {
        let path = Path::new(&current_path);

        let mut frontmatter = FrontMatterAwesome::from(self);
        frontmatter.weight = weight;

        let mut file = File::create(path.join(format!(
            "{}.md",
            self.name.to_ascii_lowercase().replace("/", "-")
        )))?;
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

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Section {
    name: String,
    content: Vec<AwesomeDir>,
    template: Option<String>,
    header: Option<String>,
}

#[derive(Serialize)]
struct FrontMatterSection {
    title: String,
    sort_by: String,
    template: Option<String>,
    weight: usize,
    extra: Option<FrontMatterSectionExtra>,
}

#[derive(Serialize)]
struct FrontMatterSectionExtra {
    header_message: String,
}

impl From<&Section> for FrontMatterSection {
    fn from(section: &Section) -> Self {
        FrontMatterSection {
            title: section.name.clone(),
            sort_by: "weight".to_string(),
            template: section.template.clone(),
            weight: 0,
            extra: section
                .header
                .clone()
                .map(|header_message| FrontMatterSectionExtra { header_message }),
        }
    }
}

impl Section {
    fn write(&self, current_path: &str, weight: usize) -> io::Result<()> {
        let path = Path::new(&current_path).join(self.name.to_ascii_lowercase());
        fs::create_dir(path.clone())?;

        let mut frontmatter = FrontMatterSection::from(self);
        frontmatter.weight = weight;

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

        let mut content = self.content.clone();
        // content.sort_by_key(AwesomeDir::name);
        content.sort_by(|a, b| a.name().partial_cmp(b.name()).unwrap());

        for (i, content) in content.iter().enumerate() {
            content.write(path.to_str().unwrap(), i)?
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum AwesomeDir {
    Section(Section),
    Awesome(Awesome),
}
impl AwesomeDir {
    fn write(&self, current_path: &str, weight: usize) -> io::Result<()> {
        match self {
            AwesomeDir::Section(content) => content.write(current_path, weight),
            AwesomeDir::Awesome(content) => content.write(current_path, weight),
        }
    }
    fn name(&self) -> &str {
        match self {
            AwesomeDir::Section(content) => &content.name,
            AwesomeDir::Awesome(content) => &content.name,
        }
    }
}

fn main() -> io::Result<()> {
    let awesome_dir = std::env::args().nth(1).unwrap();
    let frontmatter_dir = std::env::args().nth(2).unwrap();
    let _ = fs::create_dir(frontmatter_dir.clone());
    let mut awesome_section = Section {
        name: "Awesome".to_string(),
        content: vec![],
        template: Some("awesome.html".to_string()),
        header: Some("Awesome".to_string()),
    };
    visit_dirs(
        PathBuf::from_str(&awesome_dir).unwrap(),
        &mut awesome_section,
    )?;

    awesome_section.write(&frontmatter_dir, 0)?;
    Ok(())
}

fn visit_dirs(dir: PathBuf, section: &mut Section) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let folder = path.file_name().unwrap();
                let mut new_section = Section {
                    name: folder.to_str().unwrap().to_string(),
                    content: vec![],
                    template: None,
                    header: None,
                };
                visit_dirs(path.clone(), &mut new_section)?;
                section.content.push(AwesomeDir::Section(new_section));
            } else {
                let awesome: Awesome =
                    toml::de::from_str(&fs::read_to_string(entry.path()).unwrap()).unwrap();
                section.content.push(AwesomeDir::Awesome(awesome));
            }
        }
    }
    Ok(())
}
