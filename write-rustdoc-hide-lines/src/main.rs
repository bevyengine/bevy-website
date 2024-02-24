use std::{env, path::PathBuf, process::ExitCode};
use write_rustdoc_hide_lines::formatter;

/// Generates `hide_lines` annotations to Rust code blocks.
///
/// In most cases you can just call `write_rustdoc_hide_lines.sh`, which will automatically handle
/// formatting all files.
///
/// ```shell
/// $ cd write-rustdoc-hide-lines
/// $ ./write_rustdoc_hide_lines.sh
/// ```
///
/// You can also run the executable manually.
///
/// ```shell
/// $ cd write-rustdoc-hide-lines
///
/// # Format one folder.
/// $ cargo run -- format ../content/learn/book
///
/// # Format multiple folders.
/// $ cargo run -- format ../content/learn/book ../content/learn/quick-start
///
/// # Check one folder, but don't overwrite it.
/// $ cargo run -- check ../content/learn/book
/// ```
fn main() -> ExitCode {
    // The first argument is usually the executable path, so we skip that to just get arguments.
    let mut args = env::args().skip(1);

    match args.next() {
        Some(cmd) if cmd == "check" => check(args.map(PathBuf::from)),
        Some(cmd) if cmd == "format" => format(args.map(PathBuf::from)),
        Some(cmd) => {
            eprintln!(
                "Invalid subcommand '{cmd}' specified. Please use either 'format' or 'check'."
            );
            ExitCode::FAILURE
        }
        None => {
            eprintln!("No subcommand specified. Please use either 'format' or 'check'.");
            ExitCode::FAILURE
        }
    }
}

fn check(folders: impl Iterator<Item = PathBuf> + ExactSizeIterator) -> ExitCode {
    if folders.len() == 0 {
        eprintln!("Did not check any files because no folder arguments were passed.");

        return ExitCode::FAILURE;
    }

    // An aggregate list of all unformatted files, empty by default.
    let mut unformatted_files = Vec::new();

    for folder in folders {
        println!("\nChecking folder {:?}", folder);

        // Checks folders, exiting early if an error occurred.
        match formatter::check(&folder) {
            // Merge new unformatted files into existing unformatted files.
            Ok(mut unformatted) => unformatted_files.append(&mut unformatted),
            Err(error) => {
                eprintln!("Error: {}", error);

                return ExitCode::FAILURE;
            }
        }
    }

    if !unformatted_files.is_empty() {
        eprintln!("\nThe following files are not formatted:");

        for path in unformatted_files {
            eprintln!("- {:?}", path);
        }

        ExitCode::FAILURE
    } else {
        println!("All files are properly formatted. :)");

        ExitCode::SUCCESS
    }
}

fn format(folders: impl Iterator<Item = PathBuf> + ExactSizeIterator) -> ExitCode {
    if folders.len() == 0 {
        eprintln!("Did not format any files because no folder arguments were passed.");

        return ExitCode::FAILURE;
    }

    for folder in folders {
        println!("\nFormatting folder {:?}", folder);

        // Format folders, exiting early if an error occurred.
        if let Err(error) = formatter::format(&folder) {
            eprintln!("Error: {}", error);

            return ExitCode::FAILURE;
        }
    }

    println!("\nAll files have been formatted successfully!");

    ExitCode::SUCCESS
}
