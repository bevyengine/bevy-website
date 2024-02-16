# Write `rustdoc` `hide_lines` annotations

This utility iterates over the Markdown files on the given folder.
Searches `rust` code blocks and ensures that the [`hide_lines` Zola annotation] match the code block "hidden" lines (the ones starting with `#`).
If the annotation doesn't match the code block this utility will update it.

[`hide_lines` Zola annotation]: https://www.getzola.org/documentation/content/syntax-highlighting/#annotations

## Usage

Just run:

```sh
./write_rustdoc_hide_lines.sh
```

This will run the utility over the Book Markdown files.
