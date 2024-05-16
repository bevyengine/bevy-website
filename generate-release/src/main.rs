use changelog::generate_changelog;
use clap::{Parser as ClapParser, Subcommand};
use migration_guides::generate_migration_guides;
use release_notes::generate_release_notes;
use release_notes_website::generate_release_notes_website;
use std::path::PathBuf;

mod changelog;
mod github_client;
mod helpers;
mod markdown;
mod migration_guides;
mod release_notes;
mod release_notes_website;

/// Generates markdown files used for a bevy releases.
///
/// Requires a valid `GITHUB_TOKEN` environment variable, you can use a .env file or use your preferred method of passing env arguments.
///
/// Example used to generate the 0.14 release:
/// cargo run -- --from v0.13.0 --to main --release-version 0.14 migration-guide
/// cargo run -- --from v0.13.0 --to main release-note
/// cargo run -- --from bd4f611f7576c55739b466c6f0039e8421dab57e --to HEAD release-note-website
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
    /// Release notes:
    /// * Gets all merged PRs
    /// * Collect each author of merged PRs
    /// * Sort each PR per area label
    /// * Generate the list of merge PR
    #[command(verbatim_doc_comment)]
    ReleaseNotes,
    /// Generates the list of contributors and a list of all closed PRs sorted by area labels
    #[command(verbatim_doc_comment)]
    ReleaseNotesWebsite,
    Changelog,
}

fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    let args = Args::parse();
    let repo = if let Commands::ReleaseNotesWebsite { .. } = args.command {
        "bevy-website"
    } else {
        "bevy"
    };
    let mut client = github_client::GithubClient::new(
        std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found"),
        repo.to_string(),
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
            &mut client,
            overwrite_existing,
        )?,
        Commands::ReleaseNotes => generate_release_notes(
            &args.from,
            &args.to,
            release_path.join("release-notes.md"),
            &mut client,
        )?,
        Commands::ReleaseNotesWebsite => generate_release_notes_website(
            &args.from,
            &args.to,
            release_path.join("release-notes-website.md"),
            &mut client,
        )?,
        Commands::Changelog => generate_changelog(
            &args.from,
            &args.to,
            release_path.join("changelog.md"),
            &mut client,
        )?,
    };

    Ok(())
}
