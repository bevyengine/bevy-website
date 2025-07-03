use crate::{
    github_client::{GithubClient, GithubIssuesResponse},
    helpers::{get_merged_prs, get_pr_area},
};
use std::{collections::BTreeMap, fmt::Write, path::PathBuf};

pub fn generate_changelog(
    from: &str,
    to: &str,
    path: PathBuf,
    client: &GithubClient,
) -> anyhow::Result<()> {
    let mut output = String::new();

    let mut areas = BTreeMap::<Vec<String>, Vec<(String, GithubIssuesResponse)>>::new();

    let merged_prs = get_merged_prs(client, from, to, None)?;
    for (pr, _, title) in &merged_prs {
        let area = get_pr_area(pr);
        areas
            .entry(area)
            .or_default()
            .push((title.clone(), pr.clone()));
    }

    let mut count = 0;
    let mut areas_vec: Vec<_> = areas.into_iter().collect();

    // Move empty areas to the end
    areas_vec.sort_by(|(a, _), (b, _)| match (a.is_empty(), b.is_empty()) {
        (false, false) => a.join(" ").cmp(&b.join(" ")),
        (false, true) => std::cmp::Ordering::Less,
        (true, false) => std::cmp::Ordering::Greater,
        (true, true) => std::cmp::Ordering::Equal,
    });

    for (area, prs) in areas_vec {
        writeln!(output, "[[areas]]")?;
        writeln!(
            output,
            "name = [{}]",
            area.iter()
                .map(|a| format!("\"{a}\""))
                .collect::<Vec<_>>()
                .join(", ")
        )?;

        let mut prs = prs;
        prs.sort_by_key(|k| k.1.closed_at);

        for (title, pr) in prs {
            writeln!(output, "[[areas.prs]]")?;
            writeln!(output, "title = \"{}\"", title.trim().replace('"', "\\\""))?;
            writeln!(output, "number = {}", pr.number)?;

            count += 1;
        }

        writeln!(output)?;
    }

    println!("\nAdded {count} PRs to the changelog");

    std::fs::write(path, output)?;

    Ok(())
}
