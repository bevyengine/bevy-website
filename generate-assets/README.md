# Bevy-Assets Generation

This tool generates the [assets page] using the content in [`bevy-assets`].

[assets page]: https://bevy.org/assets/
[`bevy-assets`]: https://github.com/bevyengine/bevy-assets

To generate the assets page, run `generate_assets.sh`:

```shell
./generate_assets.sh
```

The shell script clones [`bevy-assets`] and runs the `generate` binary. `generate` reads the TOML file of each asset and downloads associated information from crates.io, Github, and Gitlab. When the program finishes, it will populate the `content/assets` folder. It can also be used to validate the TOML files, as used in [`bevy-assets`]'s CI, by running the `validate` binary.
