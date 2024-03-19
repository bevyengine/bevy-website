# Generate Errors

This website utility tool takes the documentation of Bevy-unique error codes from the Bevy engine repo and makes them valid Zola pages.

For most uses run the bash script from any directory:
`./generate-errors/generate-errors.sh`
or
`./generate-errors.sh`

## Niche Advice

Almost certainly all use cases will be covered by the bash script above, but if you, for whatever reason, need to use the tool straight from cargo then you can run it like:
`cargo run -- --errors-path <ERRORS_PATH> --output-path <OUTPUT_PATH>`
Where `errors-path` is the directory containing the original error files in the Bevy repo. `output-path` is the folder that the errors section folder is output.

You can also see the help page for the tool via:
`cargo run -- -h` or `cargo run -- --help`
