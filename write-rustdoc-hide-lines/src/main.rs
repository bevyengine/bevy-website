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
/// $ cargo run -- ../content/learn/book
///
/// # Format multiple folders.
/// $ cargo run -- ../content/learn/book ../content/learn/quick-start
/// ```
fn main() -> ExitCode {
    // The first argument is usually the executable path, so we skip that to just get arguments.
    let folders = env::args().skip(1).map(PathBuf::from);

    if folders.len() == 0 {
        eprintln!("Did not format any files because no folder argument was passed.");

        return ExitCode::FAILURE;
    }

    for folder in folders {
        println!("Formatting folder: {:?}", folder);

        // Format the given path, printing out errors as they occur.
        if let Err(error) = formatter::run(&folder) {
            eprintln!("Error: {}", error);

            // Exit early if an error occurred.
            return ExitCode::FAILURE;
        }
    }

    println!("Done!");

    return ExitCode::SUCCESS;
}
