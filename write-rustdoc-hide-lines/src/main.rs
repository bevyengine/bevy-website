use std::{env, fs::File, io::Write, path::PathBuf, process::ExitCode};
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

fn check(folders: impl Iterator<Item = PathBuf> + ExactSizeIterator) -> ExitCode {
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

    if is_ci {
        let mut job_summary = String::from("### `write-rustdoc-hide-lines` Summary\n\n");

        if unformatted_files.is_empty() {
            job_summary.push_str("Content properly formatted :sparkles:\n");
        } else {
            job_summary.push_str(
                "\
A few files need to be formatted :warning:

You can fix them by running `write-rustdoc-hide-lines`:

```shell
./write_rustdoc_hide_lines.sh
```\n\n",
            );

            for path in unformatted_files.iter() {
                // Write file paths using Markdown list format.
                job_summary.push_str("- ");
                job_summary.push_str(&path.to_string_lossy());
                job_summary.push('\n');

                // Output error message.
                // https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#setting-an-error-message
                println!("::error file={:?},title=File is not formatted with correct hide-lines annotations::Please run write_rustdoc_hide_lines.sh locally to fix all errors.", path);
            }

            let summary_path = env::var("GITHUB_STEP_SUMMARY")
                .expect("Could not find job summary file from environmental variable.");

            // Write job_summary to environment file.
            File::options()
                .append(true)
                .create(true)
                .open(summary_path)
                .unwrap()
                .write_all(job_summary.as_bytes())
                .unwrap();
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
