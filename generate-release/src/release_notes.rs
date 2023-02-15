use crate::helpers::get_merged_prs;
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
        // Find authors and co-authors
        // TODO this could probably be done with multiple threads to speed it up
        'retry: {
            match client.get_contributors(&commit.sha) {
                Ok(logins) => {
                    if logins.is_empty() {
                        println!(
                            "\x1b[93mNo contributors found for https://github.com/bevyengine/bevy/pull/{} sha: {}\x1b[0m",
                            pr.number,
                            commit.sha
                        );
                    }
                    for login in logins {
                        authors.insert(login);
                    }
                }
                Err(err) => {
                    println!("\x1b[93m{err:?}\x1b[0m");
                    // 15 is mostly arbitrary, but it seems to work as intended
                    println!("Sleeping 15s to avoid being rate limited");
                    std::thread::sleep(std::time::Duration::from_secs(15));
                    break 'retry;
                }
            }
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
        "Found {} prs merged by bors from {} to {}",
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
