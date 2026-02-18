use anyhow::Context;
use changelog::generate_changelog;
use clap::{Parser as ClapParser, Subcommand};
use contributors::generate_contributors;
use github_client::BevyRepo;
use migration_guides::generate_migration_guides;
use release_notes::generate_release_notes;
use std::env::current_exe;

mod changelog;
mod contributors;
mod github_client;
mod helpers;
mod markdown;
mod migration_guides;
mod release_notes;

/// Generates markdown files used for a bevy releases.
///
/// Requires a valid `GITHUB_TOKEN` environment variable, you can use a .env file or use your preferred method of passing env arguments.
///
/// Example used to generate the 0.14 release:
/// cargo run -- --from v0.13.0 --to main --release-version 0.14 migration-guides
/// cargo run -- --from v0.13.0 --to main --release-version 0.14 release-notes
/// cargo run -- --from v0.13.0 --to main --release-version 0.14 changelog
/// cargo run -- --from v0.13.0 --to main --release-version 0.14 contributors
#[derive(ClapParser)]
#[command(author, version, about, verbatim_doc_comment)]
struct Args {
    /// The name of the branch / tag to start from
    #[arg(short, long)]
    from: String,

    /// The name of the branch / tag to end on
    #[arg(short, long)]
    to: String,

    /// Release version i.e.: '0.13', '0.14', etc.
    ///
    /// This should be the version that you are preparing for release.
    #[arg(short, long)]
    release_version: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Gets all merged PRs with the `M-Migration-Guide` label or with a `Migration Guide` section in the body
    /// * For each PR generate a file with the migration guide and a frontmatter with metadata about the PR.
    ///   This parses the markdown and generates valid makrdown that should pass markdownlint rules.
    #[command(verbatim_doc_comment)]
    MigrationGuides {
        /// Use this if you want to overwrite existing files
        #[arg(short, long)]
        overwrite_existing: bool,
    },
    /// Generates release notes for all PRs merged with the `M-Release-Note` label.
    ///
    /// This will also open an issue for each PR that doesn't have a release note.
    /// While duplicate issues will not be opened, be sure to use the `--local` flag
    /// if you want to test the release notes generation without spamming the repo.
    ReleaseNotes {
        /// Use this if you want to overwrite existing files
        #[arg(short, long)]
        overwrite_existing: bool,
        /// Create issues for required release notes, and comment on the original PRs.
        #[arg(short, long)]
        create_issues: bool,
    },
    /// Generates a list of all the merged PRs for the given release
    Changelog,
    /// Generates the list of contributors
    ///
    /// This is very slow because it needs to make a network request for each commit
    #[command(verbatim_doc_comment)]
    Contributors,
}

fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    let args = Args::parse();
    let client = github_client::GithubClient::new(
        std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found"),
        BevyRepo::Bevy,
    );

    let mut release_path = current_exe()?;
    // We pop thrice because the executable is
    // in the workspaces (roots) `target/debug` directory,
    // so we're effectively finding the directory containing
    // `target/debug/executable`, which is what we want.
    for _ in 0..3 {
        release_path.pop();
    }
    let release_path = release_path
        .join("release-content")
        .join(args.release_version);

    std::fs::create_dir_all(&release_path).context("Creating the release-content path")?;

    match args.command {
        Commands::MigrationGuides { overwrite_existing } => generate_migration_guides(
            &args.from,
            &args.to,
            release_path.join("migration-guides"),
            &client,
            overwrite_existing,
        )?,
        Commands::ReleaseNotes {
            overwrite_existing,
            create_issues,
        } => generate_release_notes(
            &args.from,
            &args.to,
            release_path.join("release-notes"),
            &client,
            overwrite_existing,
            create_issues,
        )?,
        Commands::Changelog => generate_changelog(
            &args.from,
            &args.to,
            release_path.join("changelog.toml"),
            &client,
        )?,
        Commands::Contributors => generate_contributors(
            &args.from,
            &args.to,
            release_path.join("contributors.toml"),
            &client,
        )?,
    };

    Ok(())
}
