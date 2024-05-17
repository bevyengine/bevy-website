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

    let mut areas = BTreeMap::<String, Vec<(String, GithubIssuesResponse)>>::new();

    let merged_prs = get_merged_prs(client, from, to, None)?;
    for (pr, _, title) in &merged_prs {
        let area = get_pr_area(pr);
        areas
            .entry(area)
            .or_default()
            .push((title.clone(), pr.clone()));
    }

    writeln!(output, "## Full Changelog")?;
    writeln!(output, "The changes mentioned above are only the most appealing, highest impact changes that we've made this cycle.
Innumerable bug fixes, documentation changes and API usability tweaks made it in too.
For a complete list of changes, check out the PRs listed below.")?;

    let mut count = 0;
    for (area, prs) in areas {
        writeln!(output)?;
        writeln!(output, "### {area}")?;
        writeln!(output)?;

        let mut prs = prs;
        prs.sort_by_key(|k| k.1.closed_at);

        for (title, pr) in prs {
            writeln!(
                output,
                "* [{}](https://github.com/bevyengine/bevy/pull/{})",
                title.trim(),
                pr.number
            )?;
            count += 1;
        }
    }

    println!("\nAdded {count} PRs to the changelog");

    std::fs::write(path, output)?;

    Ok(())
}
