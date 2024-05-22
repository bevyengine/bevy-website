# Generate Release

This CLI tool is used to generate all the skeleton files required to create a new release.

For a bit more background see this issue: <https://github.com/bevyengine/bevy-website/issues/1163>

All commands assume they are ran in the `./generate-release` folder.

Each command will generate files in the `/release-content/{release-version}` folder. The `release-version` is an argument to all commands.

Each command have a `--from` and `--to` argument. You can pass it a git tag or a git commit.

Before running the command, you'll need to generate a github api token at <https://github.com/settings/tokens>. It's easeier to use classic tokens and you don't need any specific role, just the default is enough. Then add it to a file called `.env` like so:

```env
GITHUB_TOKEN=token_string_copied_from_github
```

Here's an example for the commands used to generate the 0.14 release:

```shell
cargo run -- --from v0.13.0 --to main --release-version 0.14 migration-guides
cargo run -- --from v0.13.0 --to main --release-version 0.14 release-notes
cargo run -- --from v0.13.0 --to main --release-version 0.14 changelog
cargo run -- --from v0.13.0 --to main --release-version 0.14 contributors
```

## Migration Guides

The `migration-guides` command will generate the `/release-content/{release-version}/migration-guides` folder.
Inside will be a single `_guides.toml` that contains metadata needed for each guides. Then each guide will be a separate markdown file inside that folder.

Once the files are generated, you can easily add a new migration guide by adding a new file in `content/learn/migration-guides`.

Inside that file, you should have something that looks like this:

```markdown
+++
title = "0.13 to 0.14"
insert_anchor_links = "right"
[extra]
weight = 9
long_title = "Migration Guide: 0.13 to 0.14"
+++

{{ migration_guides(version="0.14") }}
```

The most important part of this is the `migrations_guides` shortcode. It will get the list of guides from the `_guides.toml` file and combine all the separate file and generate appropriate markup for it.

Remember to update the weight to be higher than the previous guides.

## Release Notes

The release notes is a bit more complicated since it has multiple parts that need to be generated.

You'll need to use the `release-notes`, `changelog`, and `contributors` commands.

- `release-notes` will generate the `/release-content/{release-version}/release-notes` folder. Inside will be a single `_release-notes.toml` file that contains the list of file names that will be combined into the final blog post. Each PR that needs a release note will then have a file generated.
- `changelog` generates a `changelog.md` file with a list of all PRs merged sorted by main area.
- `contributors` generates a `contributors.md` file with a list of all the usernames that authored a PR for the specified release.

Once all those files are generated you'll need to create a new blog post in `content/news`. The content of the `index.md` file should look something like this:

```markdown
+++
title = "Bevy 0.14"
date = 2024-05-17
[extra]
public_draft = _release tracking issue number_
+++

<!-- TODO Intro -->

<!-- more -->

{{ release_notes(version="0.14") }}

## What's Next?

<!-- TODO What's next -->

{{ support_bevy() }}
{{ contributors(version="0.14") }}
{{ changelog(version="0.14")}}
```

The most important part of this is the `combine_release_notes`, `changelog`, and `contributors` shortcodes. `combine_release_notes` will get the list of release notes from the `_release_notes.toml` file and combine all the separate file and add them to this file. `contributors()` will load the `contributors.md` file and generate the necessary markup. `changelog()` will load the `changelog.md` file and generate the necessary markup.
