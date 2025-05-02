use anyhow::Context;
use rayon::prelude::*;

use crate::github_client::GithubClient;
use crate::helpers::{get_contributors, get_merged_prs};
use std::{collections::HashSet, fmt::Write, path::PathBuf};

/// Generates the list of contributors and a list of all closed PRs sorted by area labels
pub fn generate_contributors(
    from: &str,
    to: &str,
    path: PathBuf,
    client: &GithubClient,
) -> anyhow::Result<()> {
    // TODO consider adding website contributors

    let merged_prs = get_merged_prs(client, from, to, None)?;

    // Getting the list of contributors is really slow because we need to make a separate call for each commit
    // So we use rayon and parallelize the queries.
    // This means we hit the rate limit faster, so there's some retry logic in the client.
    // It still results in getting the full list faster overall
    rayon::ThreadPoolBuilder::new()
        // if we go too fast github starts complaining, so don't use all threads
        .num_threads(3)
        .build_global()
        .unwrap();
    let mut contributors = merged_prs
        .par_iter()
        .map(|(pr, commit, _)| -> HashSet<String> {
            let mut contributors = HashSet::new();
            let pr_contributors = get_contributors(client, commit, pr).unwrap();
            for c in pr_contributors {
                contributors.insert(c);
            }
            contributors.insert(format!("@{}", pr.user.login.clone()));
            contributors
        })
        .flatten()
        .collect::<HashSet<String>>();

    contributors.remove("@github-actions[bot]");

    println!("Found {} unique contributors", contributors.len());

    let mut output = String::new();

    for name in &contributors {
        writeln!(
            output,
            r#"[[contributors]]
name = '{name}'
"#
        )?;
    }

    std::fs::write(path, output).context("Writing contributors file")?;

    Ok(())
}
