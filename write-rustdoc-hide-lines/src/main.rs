use std::{env, path::PathBuf, process::ExitCode};
use write_rustdoc_hide_lines::formatter;

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

fn check(folders: impl ExactSizeIterator<Item = PathBuf>) -> ExitCode {
    if folders.len() == 0 {
        eprintln!("Did not check any files because no folder arguments were passed.");

        return ExitCode::FAILURE;
    }

    // An aggregate list of all unformatted files, empty by default.
    let mut unformatted_files = Vec::new();

    // Detect if we're running through Github Actions or not.
    // https://docs.github.com/en/actions/learn-github-actions/variables#default-environment-variables
    let is_ci = env::var("GITHUB_ACTIONS").is_ok_and(|x| x == "true");

    for folder in folders {
        if is_ci {
            // Create an collapsible group for all files checked.
            // https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#grouping-log-lines
            println!("::group::Checking folder {folder:?}");
        } else {
            println!("\nChecking folder {folder:?}");
        }

        // Checks folders, exiting early if an error occurred.
        match formatter::check(&folder) {
            // Merge new unformatted files into existing unformatted files.
            Ok(mut unformatted) => unformatted_files.append(&mut unformatted),
            Err(error) => {
                if is_ci {
                    // End group if an error occurred, so it is not hidden.
                    println!("::endgroup::");
                }

                eprintln!("Error: {error}");

                return ExitCode::FAILURE;
            }
        }

        if is_ci {
            println!("::endgroup::");
        }
    }

    if !unformatted_files.is_empty() {
        println!("\nThe following files are not formatted:");

        for path in unformatted_files {
            if is_ci {
                // Print custom error message, formatted in Github Actions.
                // https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#setting-an-error-message
                println!("::error file={path:?},title=File is not formatted with correct hide-lines annotations::- {path:?}");
            } else {
                println!("- {path:?}");
            }
        }

        println!("\nRun write_rustdoc_hide_lines.sh to automatically fix these errors.");

        ExitCode::FAILURE
    } else {
        println!("All files are properly formatted. :)");

        ExitCode::SUCCESS
    }
}

fn format(folders: impl ExactSizeIterator<Item = PathBuf>) -> ExitCode {
    if folders.len() == 0 {
        eprintln!("Did not format any files because no folder arguments were passed.");

        return ExitCode::FAILURE;
    }

    for folder in folders {
        println!("\nFormatting folder {folder:?}");

        // Format folders, exiting early if an error occurred.
        if let Err(error) = formatter::format(&folder) {
            eprintln!("Error: {error}");

            return ExitCode::FAILURE;
        }
    }

    println!("\nAll files have been formatted successfully!");

    ExitCode::SUCCESS
}
