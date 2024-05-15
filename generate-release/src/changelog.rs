use crate::{
    github_client::{GithubClient, GithubIssuesResponse},
    helpers::{get_merged_prs, get_pr_area},
    markdown::write_markdown_section,
};
use std::{collections::BTreeMap, fmt::Write, path::PathBuf};

pub fn generate_changelog(
    from: &str,
    to: &str,
    path: PathBuf,
    client: &mut GithubClient,
) -> anyhow::Result<()> {
    let mut out = String::new();

    let mut areas = BTreeMap::<String, Vec<(String, GithubIssuesResponse)>>::new();

    let merged_prs = get_merged_prs(client, from, to, None)?;
    for (pr, _, title) in &merged_prs {
        let area = get_pr_area(pr);
        areas
            .entry(area)
            .or_default()
            .push((title.clone(), pr.clone()));
    }

    writeln!(out, "# Changelog")?;

    let mut count = 0;
    for (area, prs) in areas {
        writeln!(out, "## {area}")?;

        let mut prs = prs;
        prs.sort_by_key(|k| k.1.closed_at);

        for (title, pr) in prs {
            println!("# {title}");

            if let Some(body) = pr.body.as_ref() {
                let heading = format!(
                    "\n### [{}](https://github.com/bevyengine/bevy/pull/{})",
                    title, pr.number
                );
                writeln!(&mut out, "{heading}")?;

                let (section, found) = write_markdown_section(body, "changelog", false)?;
                write!(out, "{section}")?;
                if found {
                    count += 1;
                } else {
                    // Changelog not found so remove heading
                    // We need to do this because we don't know if there's a changelog when writing the heading
                    out = out.replace(&heading, "");
                }
            }
        }
    }

    println!("\nFound {count} PRs with a changelog");

    std::fs::write(path, out)?;

    Ok(())
}
