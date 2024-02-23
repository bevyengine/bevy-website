# Write `rustdoc` `hide_lines` Annotations

<!-- markdownlint-disable-next-line MD038 -->
This utility recursively iterates over all Markdown files in a given folder. It will update the [`hide_lines` Zola annotation] on all `rust` and `rs` code blocks to match [rustdoc hidden lines]. It will match all lines that start with `# `. A space after the hashtag is required, or else it would accidentally hide attributes like `#[derive(...)]`. If `hide_lines` is out of date, this tool can automatically update it.

[`hide_lines` Zola annotation]: https://www.getzola.org/documentation/content/syntax-highlighting/#annotations
[rustdoc hidden lines]: https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#hiding-portions-of-the-example

## Usage

To format the entire website, you can run `write_rustdoc_hide_lines.sh` from any directory:

```shell
./write_rustdoc_hide_lines.sh
```

The script automatically handles formatting the all Markdown files in the `content` directory. It is not an alias and does not accept any arguments. In general, you will only ever need to run the above script in order to make Github Actions pass.

If you want to format a specific directory, you can run the tool using [Cargo]:

[Cargo]: https://doc.rust-lang.org/cargo/index.html

```shell
cargo run -- format ./path/to/directory
```

You can also tell the tool to format multiple directories:

```shell
cargo run -- format ./folder1 ./folder2
```

If you just want to check a directory and do not want to format it, you can use the check command:

```shell
cargo run -- check ./folder1 ./folder2
```
