use std::{
    collections::HashMap,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Context;

pub fn combine_migration_guides(
    title: String,
    weight: i32,
    release_version: String,
    output_path: Option<PathBuf>,
) -> anyhow::Result<()> {
    let output_path = output_path.unwrap_or(
        Path::new("..")
            .join("content")
            .join("learn")
            .join("migration-guides")
            .join(format!("{}.md", &title))
            .to_path_buf(),
    );
    let mut output = std::fs::File::create(output_path).context("Creating output file")?;

    // Write the frontmatter based on given parameters
    write!(
        &mut output,
        r#"+++
title = "{title}"
insert_anchor_links = "right"
[extra]
weight = {weight}
long_title = "Migration Guide: {title}"
+++

Bevy relies heavily on improvements in the Rust language and compiler.
As a result, the Minimum Supported Rust Version (MSRV) is "the latest stable release" of Rust."#
    )?;
    writeln!(&mut output)?;
    writeln!(&mut output)?;
    writeln!(&mut output, "<div class=\"migration-guide\">")?;

    // WARN This assumes it gets executed inside the generate-release folder
    // Should probably just be an input
    let migration_guides = Path::new("..")
        .join("release-content")
        .join(release_version)
        .join("migration-guides");
    let dir_content =
        std::fs::read_dir(migration_guides).context("Reading migration guides dir")?;
    for entry in dir_content {
        let area_path = entry?.path();
        if !area_path.is_dir() {
            break;
        }
        let guides = std::fs::read_dir(&area_path)?;
        for guide in guides {
            // println!("{:?}", guide?.path().components().last().unwrap());
            let content = std::fs::read_to_string(guide?.path())?;

            writeln!(&mut output)?;

            // CommonMark doesn't support metadata, so we kinda have to parse this manually
            // There's probably a nicer way to do this
            let mut frontmatter_count = 0;
            let mut frontmatter = HashMap::<&str, &str>::new();
            let mut lines_iter = content.lines();
            while let Some(line) = lines_iter.next() {
                if line.starts_with("+++") {
                    frontmatter_count += 1;
                }
                if frontmatter_count == 1 {
                    // in frontmatter
                    let Some((k, v)) = line.split_once('=') else {
                        continue;
                    };
                    frontmatter.insert(k.trim(), v.trim());
                }
                if frontmatter_count >= 2 {
                    break;
                }
            }

            // Write the title of the guide
            let mut title = frontmatter
                .get("title")
                .expect("frontmatter missing title key")
                .to_string();
            // remove the double quotes from the value
            title.remove(0);
            title.remove(title.len() - 1);
            let mut url = frontmatter
                .get("url")
                .expect("frontmatter missing url key")
                .to_string();
            // remove the double quotes from the value
            url.remove(0);
            url.remove(url.len() - 1);
            writeln!(&mut output, "### [{title}]({url})")?;

            let areas = frontmatter
                .get("areas")
                .expect("frontmatter missing areas key");
            // remove the brackets
            let areas = areas.replace('[', "").replace(']', "");
            let areas = areas.split(',').collect::<Vec<_>>();

            // Write custom HTML to show area tag on each section
            write!(&mut output, "\n<div class=\"migration-guide-area-tags\">")?;
            for area in &areas {
                write!(
                    &mut output,
                    "\n    <div class=\"migration-guide-area-tag\">{area}</div>"
                )?;
            }
            writeln!(&mut output, "\n</div>")?;

            let body = lines_iter.map(|l| format!("{l}\n")).collect::<String>();
            write!(&mut output, "{}", body)?;
        }
    }

    // closing tag for the file
    writeln!(&mut output, "</div>")?;

    Ok(())
}
