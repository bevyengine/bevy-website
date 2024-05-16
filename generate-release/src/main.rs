use changelog::generate_changelog;
use clap::{Parser as ClapParser, Subcommand};
use migration_guide::generate_migration_guide;
use release_notes::generate_release_note;
use release_notes_website::generate_release_notes_website;
use std::path::PathBuf;

mod changelog;
mod github_client;
mod helpers;
mod markdown;
mod migration_guide;
mod release_notes;
mod release_notes_website;

/// Generates markdown files used for a bevy releases.
///
/// Requires a valid `GITHUB_TOKEN` environment variable, you can use a .env file or use your preferred method of passing env arguments.
///
/// Example used to generate the 0.14 release:
/// cargo run -- migration-guide --from v0.13.0 --to main --release-version 0.14
/// cargo run -- release-note --from v0.13.0 --to main
/// cargo run -- release-note-website --from bd4f611f7576c55739b466c6f0039e8421dab57e --to HEAD
#[derive(ClapParser)]
#[command(author, version, about, verbatim_doc_comment)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Gets all merged PRs with the `C-Breaking-Change` label or with a `Migration Guide` section in the body
    /// * For each PR generate a file with the migration guide and a frontmatter with metadata about the PR.
    ///   This parses the markdown and generates valid makrdown that should pass markdownlint rules.
    #[command(verbatim_doc_comment)]
    MigrationGuide {
        /// The name of the branch / tag to start from
        #[arg(long)]
        from: String,

        /// The name of the branch / tag to end on
        #[arg(long)]
        to: String,

        /// Release version i.e.: '0.13', '0.14', etc.
        ///
        /// This should be the version that you are preparing for release.
        #[arg(short, long)]
        release_version: String,

        /// Path used to output the generated file. Defaults to ./migration-guide.md
        #[arg(short, long)]
        path: Option<std::path::PathBuf>,

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
    ReleaseNote {
        /// The name of the branch / tag to start from
        #[arg(short, long)]
        from: String,

        /// The name of the branch / tag to end on
        #[arg(short, long)]
        to: String,

        /// Path used to output the generated file. Defaults to ./release-notes.md
        #[arg(short, long)]
        path: Option<std::path::PathBuf>,
    },
    /// Generates the list of contributors and a list of all closed PRs sorted by area labels
    #[command(verbatim_doc_comment)]
    ReleaseNoteWebsite {
        /// The name of the branch / tag to start from
        #[arg(short, long)]
        from: String,

        /// The name of the branch / tag to end on
        #[arg(short, long)]
        to: String,

        /// Path used to output the generated file. Defaults to ./release-notes-website.md
        #[arg(short, long)]
        path: Option<std::path::PathBuf>,
    },
    Changelog {
        /// The name of the branch / tag to start from
        #[arg(short, long)]
        from: String,

        /// The name of the branch / tag to end on
        #[arg(short, long)]
        to: String,

        /// Path used to output the generated file. Defaults to ./changelog.md
        #[arg(short, long)]
        path: Option<std::path::PathBuf>,
    },
}

fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    let args = Args::parse();
    let repo = if let Commands::ReleaseNoteWebsite { .. } = args.command {
        "bevy-website"
    } else {
        "bevy"
    };
    let mut client = github_client::GithubClient::new(
        std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found"),
        repo.to_string(),
    );

    match args.command {
        Commands::MigrationGuide {
            from,
            to,
            release_version,
            path,
            overwrite_existing,
        } => generate_migration_guide(
            &from,
            &to,
            path.unwrap_or_else(|| {
                PathBuf::from(format!(
                    "../release-content/{release_version}/migration-guides"
                ))
            }),
            &mut client,
            overwrite_existing,
        )?,
        Commands::ReleaseNote { from, to, path } => generate_release_note(
            &from,
            &to,
            path.unwrap_or_else(|| PathBuf::from("./release-notes.md")),
            &mut client,
        )?,
        Commands::ReleaseNoteWebsite { from, to, path } => generate_release_notes_website(
            &from,
            &to,
            path.unwrap_or_else(|| PathBuf::from("./release-notes-website.md")),
            &mut client,
        )?,
        Commands::Changelog { from, to, path } => generate_changelog(
            &from,
            &to,
            path.unwrap_or_else(|| PathBuf::from("./changelog.md")),
            &mut client,
        )?,
    };

    Ok(())
}
