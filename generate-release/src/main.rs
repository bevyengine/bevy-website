use clap::{Parser as ClapParser, Subcommand};
use migration_guide::generate_migration_guide;
use release_notes::generate_release_note;
use std::path::PathBuf;

mod github_client;
mod helpers;
mod migration_guide;
mod release_notes;

/// Generates markdown files used for a bevy releases.
///
/// Migration Guide:
/// * Gets all PRs with the `C-Breaking-Change` label and that were merged by bors.
/// * For each PR:
///     * Generate the title with a link to the relevant PR and
///     * Generate the migration guide section. This parses the markdown and generates valid makrdown that should pass markdownlint rules.
///
/// Release notes:
/// * Gets all PRs merged by bors
/// * Collect each author of closed PRs (Should this just list all contributors?)
/// * Sort each PR per area label
/// * Generate the list of merge PR
///
/// Requires a valid GITHUB_TOKEN environment variable, you can use a .env file or use your prefered method of passing env arguments.
///
/// Example used to generate for 0.9:
/// cargo run -- migration-guide --date 2022-11-12 --title "0.9 to 0.10" --weight 6
/// cargo run -- release-note --date 2022-11-12
#[derive(ClapParser)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    MigrationGuide {
        /// Date of the release of the previous version. Format: YYYY-MM-DD
        #[arg(short, long)]
        date: String,

        /// Title of the frontmatter
        #[arg(short, long)]
        title: String,

        /// Weight used for sorting
        #[arg(short, long)]
        weight: i32,

        /// Path used to output the generated file. Defaults to ./migration-guide.md
        #[arg(short, long)]
        path: Option<std::path::PathBuf>,
    },
    ReleaseNote {
        /// Date of the release of the previous version. Format: YYYY-MM-DD
        #[arg(short, long)]
        date: String,

        /// Path used to output the generated file. Defaults to ./release-note.md
        #[arg(short, long)]
        path: Option<std::path::PathBuf>,
    },
}

fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    let mut client = github_client::GithubClient::new(
        std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found"),
    );

    let args = Args::parse();
    match args.command {
        Commands::MigrationGuide {
            date,
            title,
            weight,
            path,
        } => generate_migration_guide(
            &title,
            weight,
            &date,
            path.unwrap_or_else(|| PathBuf::from("./migration-guide.md")),
            &mut client,
        )?,
        Commands::ReleaseNote { date, path } => generate_release_note(
            &date,
            path.unwrap_or_else(|| PathBuf::from("./release-note.md")),
            &mut client,
        )?,
    };

    Ok(())
}
