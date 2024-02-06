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
        Some(cmd) if cmd == "check" => check_or_format(args.map(PathBuf::from), false),
        Some(cmd) if cmd == "format" => check_or_format(args.map(PathBuf::from), true),
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

/// Checks each file in `folders`, optionally fixing them if `format` is true.
fn check_or_format(
    folders: impl Iterator<Item = PathBuf> + ExactSizeIterator,
    format: bool,
) -> ExitCode {
    if folders.len() == 0 {
        eprintln!("Did not format any files because no folder argument was passed.");

        return ExitCode::FAILURE;
    }

    for folder in folders {
        println!("Formatting folder: {:?}", folder);

        if format {
            // Format the given path, printing out errors as they occur.
            if let Err(error) = formatter::run(&folder) {
                eprintln!("Error: {}", error);

                // Exit early if an error occurred.
                return ExitCode::FAILURE;
            }
        } else {
            todo!("Checking folders is not yet implemented. Come back soon! :)");
        }
    }

    println!("Done!");

    ExitCode::SUCCESS
}
