# Generate Release

This CLI tool is used to generate all the skeleton files required to create a new release.

For a bit more background see this issue: <https://github.com/bevyengine/bevy-website/issues/1163>

All commands assume they are ran in the `/generate-release` folder.

Each command will generate files in the `/release-content/{release-version}` folder. The `release-version` is an argument to all commands.

Each command have a `--from` and `--to` argument. You can pass it a Git branch, tag, or commit.

Before running the command, you'll need to generate a GitHub API token at <https://github.com/settings/tokens>. It's easier to use classic tokens.
The token must have `repo` permissions to be able to open issues (and PRs) on your behalf.

Then add it to a file called `.env` like so:

```env
GITHUB_TOKEN=token_string_copied_from_github
```

Here's an example for the commands used to generate the `0.14` release:

```shell
cargo run -- --from v0.13.0 --to main --release-version 0.14 migration-guides
cargo run -- --from v0.13.0 --to main --release-version 0.14 release-notes
cargo run -- --from v0.13.0 --to main --release-version 0.14 changelog
cargo run -- --from v0.13.0 --to main --release-version 0.14 contributors
```

## Generating a release

To generate a release from scratch, run all these commands then add the new migration guide and blog post to their respective `/content` folder. When doing so, it's important to use the `public_draft` feature to hide those pages until the day of the release. For the `public_draft` feature, you'll need to provide it a GitHub issue number, it's recommended to point it to an issue tracker for the current release being worked on. The issue needs to be on the `bevy-website` repo.

The following sections go in more details on each parts of the process.

### Migration Guides

The `migration-guides` command will generate the `/release-content/{release-version}/migration-guides` folder.
Inside will be a single `_guides.toml` that contains metadata needed for each guides. Then each guide will be a separate markdown file inside that folder.

Once the files are generated, you can easily add a new migration guide by adding a new file in `/content/learn/migration-guides`.

Inside that file, you should have something that looks like this:

```markdown
+++
title = "0.13 to 0.14"
insert_anchor_links = "right"
[extra]
weight = 9
long_title = "Migration Guide: 0.13 to 0.14"
public_draft = _release tracking issue number_
+++

{{ migration_guides(version="0.14") }}
```

The most important part of this is the `migrations_guides` shortcode. It will get the list of guides from the `_guides.toml` file and combine all the separate file and generate appropriate markup for it.

Remember to update the weight to be higher than the previous guides.

### Release Notes

The release notes is a bit more complicated since it has multiple parts that need to be generated.

You'll need to use the `release-notes`, `changelog`, and `contributors` commands.

- `release-notes` will generate the `/release-content/{release-version}/release-notes` folder. Inside will be a single `_release-notes.toml` file that contains the list of file names that will be combined into the final blog post. Each PR that needs a release note will then have a file generated.
- `changelog` generates a `changelog.toml` file with a list of all PRs merged sorted by main area.
- `contributors` generates a `contributors.toml` file with a list of all the usernames that authored a PR for the specified release.

Once all those files are generated you'll need to create a new blog post in `/content/news`. The content of the `index.md` file should look something like this:

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

The most important part of this is the `release_notes`, `changelog`, and `contributors` shortcodes. `release_notes` will get the list of release notes from the `_release_notes.toml` file and combine all the separate file and add them to this file. `contributors()` will load the `contributors.toml` file and generate the necessary markup. `changelog()` will load the `changelog.toml` file and generate the necessary markup.
