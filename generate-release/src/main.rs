use changelog::generate_changelog;
use clap::{Parser as ClapParser, Subcommand};
use contributors::generate_contributors;
use migration_guides::generate_migration_guides;
use std::path::PathBuf;

mod changelog;
mod contributors;
mod github_client;
mod helpers;
mod markdown;
mod migration_guides;

/// Generates markdown files used for a bevy releases.
///
/// Requires a valid `GITHUB_TOKEN` environment variable, you can use a .env file or use your preferred method of passing env arguments.
///
/// Example used to generate the 0.14 release:
/// cargo run -- --from v0.13.0 --to main --release-version 0.14 migration-guides
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
    /// Gets all merged PRs with the `C-Breaking-Change` label or with a `Migration Guide` section in the body
    /// * For each PR generate a file with the migration guide and a frontmatter with metadata about the PR.
    ///   This parses the markdown and generates valid makrdown that should pass markdownlint rules.
    #[command(verbatim_doc_comment)]
    MigrationGuides {
        /// Use this if you want to overwrite existing files
        #[arg(short, long)]
        overwrite_existing: bool,
    },
    /// Generates a list of all the merged PRs for the given release
    Changelog,
    /// Generates the list of contributors
    Contributors,
}

fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    let args = Args::parse();
    let client = github_client::GithubClient::new(
        std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found"),
        String::from("bevy"),
    );

    // WARN this assumes it gets ran from ./generate-release
    let release_path = PathBuf::from("..")
        .join("release-content")
        .join(args.release_version);

    match args.command {
        Commands::MigrationGuides { overwrite_existing } => generate_migration_guides(
            &args.from,
            &args.to,
            release_path.join("migration-guides"),
            &client,
            overwrite_existing,
        )?,
        Commands::Changelog => generate_changelog(
            &args.from,
            &args.to,
            release_path.join("changelog.md"),
            &client,
        )?,
        Commands::Contributors => generate_contributors(
            &args.from,
            &args.to,
            release_path.join("contributors.md"),
            &client,
        )?,
    };

    Ok(())
}
