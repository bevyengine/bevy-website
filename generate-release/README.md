# Generate Release

This CLI tool is used to generate all the skeleton files required to create a new release.

For a bit more background see this issue: <https://github.com/bevyengine/bevy-website/issues/1163>

The commands can be run from anywhere inside the workspace folder. If you have a `.env` file, this will only work if it is located at the root of the workspace.

Each command will generate files in the `/release-content/{release-version}` folder. The `release-version` is an argument to all commands.

Each command have a `--from` and `--to` argument. You can pass it a Git branch, tag, or commit.

To create issues for the `release-notes` subcommand, you need to pass the `--create-issues` flag, otherwise it performs a dry-run that does not have lasting consequences. This should probably only be done for the initial run, after a regular dry-run has been done to confirm the tool is working as expected.

Before running the command, you'll need to generate a GitHub API token at <https://github.com/settings/tokens>. It's easier to use classic tokens.
The token must have `repo` permissions to be able to open issues (and PRs) on your behalf.

Then add it to a file called `.env` (stored in the root `bevy-website` folder) like so:

```env
GITHUB_TOKEN=token_string_copied_from_github
```

Here's an example for the commands used to generate the `0.14` release:

```shell
cargo run -p generate-release -- --from v0.13.0 --to main --release-version 0.14 migration-guides
cargo run -p generate-release -- --from v0.13.0 --to main --release-version 0.14 release-notes
cargo run -p generate-release -- --from v0.13.0 --to main --release-version 0.14 changelog
cargo run -p generate-release -- --from v0.13.0 --to main --release-version 0.14 contributors
```

## Generating a release

To generate a release from scratch, run all these commands then add the new migration guide and blog post to their respective `/content` folder. When doing so, it's important to use the `public_draft` feature to hide those pages until the day of the release. For the `public_draft` feature, you'll need to provide it a GitHub issue number, it's recommended to point it to an issue tracker for the current release being worked on. The issue needs to be on the `bevy-website` repo.

When you're merging or editing notes and guides, keep in mind that this tool will not regenerate notes or guides that still have a PR number in any note or guide's metadata, contained in the `_<release_notes|guides>.toml`. This means to merge multiple PRs into one note or guide you simply remove one `[[release_notes]]` or `[[guides]]` entry, and move its PR number to the merged entry that is the sum of all the merged PRs. For editing, this means the other metadata will also not be regenerated if the PR number still exists in the metadata.

The following sections go in more details on each parts of the process.

### Migration Guides

The `migration-guides` command will generate the `/release-content/{release-version}/migration-guides` folder.
Inside will be a single `_guides.toml` that contains metadata needed for each guides. Then each guide will be a separate markdown file inside that folder.

Once the files are generated, you can easily add a new migration guide by adding a new file in `/content/learn/migration-guides`.

Inside that file, you should have something that looks like this:

```markdown
+++
# Let Xa be the old major version, and ya the old minor version,
# and Xb be the new major version, and yb the new minor version.
# 
# Change the Bevy versions below to match these!
title = "Xa.ya to Xb.yb"
insert_anchor_links = "right"
[extra]
# Let N be the weight of the prior / last migration guide, plus one.
weight = N
long_title = "Migration Guide: Xa.ya to Xb.yb"
# GitHub issue number for tracking this release's
# migration guides or news post.
public_draft = _release tracking issue number_
+++

{{ migration_guides(version="Xb.yb") }}
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
# Let X be the major version, and y the minor version.
# Change the Bevy release versions below to match this one!
title = "Bevy X.y"
# Insert a date in the year, month, day format.
# This should be the date that the post will be posted.
date = YYYY-MM-DD
[extra]
# GitHub issue number for tracking this release's
# news post.
public_draft = _release tracking issue number_
+++

<!-- TODO Intro -->

<!-- more -->

{{ release_notes(version="X.y") }}

## What's Next?

<!-- TODO What's next -->

{{ support_bevy() }}
{{ contributors(version="X.y") }}
{{ changelog(version="X.y")}}
```

The most important part of this is the `release_notes`, `changelog`, and `contributors` shortcodes. `release_notes` will get the list of release notes from the `_release_notes.toml` file and combine all the separate file and add them to this file. `contributors()` will load the `contributors.toml` file and generate the necessary markup. `changelog()` will load the `changelog.toml` file and generate the necessary markup.

> [!NOTE]
> The `contributors` field in `_release_notes.toml` is for all non-PR-author contributors to the PR; they should be added to the `authors` field on a case-by-case basis depending on level of involvement.
