use crate::helpers::{get_contributors, get_merged_prs};
use crate::{github_client::GithubClient, helpers::get_pr_area};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fmt::Write,
    path::PathBuf,
};

/// Generates the list of contributors and a list of all closed PRs sorted by area labels
pub fn generate_release_note(
    from: &str,
    to: &str,
    path: PathBuf,
    client: &mut GithubClient,
) -> anyhow::Result<()> {
    let mut pr_map = BTreeMap::new();
    let mut areas = HashMap::<String, Vec<i32>>::new();
    let mut authors = HashSet::new();

    let merged_prs = get_merged_prs(client, from, to, None)?;
    for (pr, commit, title) in &merged_prs {
        let contributors = get_contributors(client, commit, pr)?;
        for c in contributors {
            authors.insert(c);
        }

        pr_map.insert(pr.number, title.to_string());

        let area = get_pr_area(pr);
        areas.entry(area).or_default().push(pr.number);

        authors.insert(pr.user.login.clone());
        println!(
            "[{title}](https://github.com/bevyengine/bevy/pull/{})",
            pr.number
        );
    }

    println!(
        "Found {} merged prs from {} to {}",
        merged_prs.len(),
        from,
        to,
    );

    let mut output = String::new();

    writeln!(&mut output, "# Release Notes - From {from} to {to}\n")?;

    writeln!(&mut output, "## Contributors\n")?;
    writeln!(&mut output, "A huge thanks to the {} contributors that made this release (and associated docs) possible! In random order:\n", authors.len())?;
    for author in &authors {
        writeln!(&mut output, "- @{author}")?;
    }
    writeln!(&mut output)?;

    writeln!(&mut output, "## Full Changelog")?;

    for (area, prs) in &areas {
        writeln!(&mut output)?;
        writeln!(&mut output, "## {area}")?;
        writeln!(&mut output)?;

        for pr_number in prs {
            let Some(pr_title) = pr_map.get(pr_number) else {
                continue;
            };
            writeln!(&mut output, "- [{pr_title}][{pr_number}]")?;
        }
    }

    writeln!(&mut output)?;

    for pr in pr_map.keys() {
        writeln!(
            &mut output,
            "[{pr}]: https://github.com/bevyengine/bevy/pull/{pr}"
        )?;
    }

    std::fs::write(path, output)?;

    Ok(())
}
