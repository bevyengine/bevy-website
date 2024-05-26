The CI tool no longer supports running multiple subcommands in a single call. Users who are currently doing so will need to split them across multiple commands:

```bash
# BEFORE
cargo run -p ci -- lints doc compile

# AFTER
cargo run -p ci -- lints && cargo run -p ci -- doc && cargo run -p ci -- compile
# or
cargo run -p ci -- lints; cargo run -p ci -- doc; cargo run -p ci -- compile
# or
cargo run -p ci -- lints
cargo run -p ci -- doc
cargo run -p ci -- compile
```
