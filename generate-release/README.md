# Generate Release

This CLI tool is used to generate all the skeleton files required to create a new release.

For a bit more background see this issue: <https://github.com/bevyengine/bevy-website/issues/1163>

All commands assume they are ran in the `./generate-release` folder.

Each command will generate files in the `/release-content/{release-version}` folder. The `release-version` is an argument to all commands.

Each command have a `--from` and `--to` argument. You can pass it a git tag or a git commit.

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

{% callout(type="warning") %}
    Bevy relies heavily on improvements in the Rust language and compiler. As a result, the Minimum Supported Rust Version (MSRV) is "the latest stable release" of Rust.
{% end %}

<div class="migration-guide">
    {{ combine_migration_guides(release_content_path = "./release-content/0.14/") }}
</div>
```

The most important part of this is the `combine_migrations_guides` shortcode. It will get the list of guides from the `_guides.toml` file and combine all the separate file and add them to this file.

When adding a new migration guide, remember to update the weight of the `index.md` file. It always need to be one more than the new guide.

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
show_image = false

+++

<!-- Intro -->

<!-- TODO -->

<!-- more -->

{{ combine_release_notes(release_content_path = "./release-content/0.14/") }}

## <a name="what-s-next"></a>What's Next?

<!-- TODO -->

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

<!-- Contributors -->
{{ contributors(path="./release-content/0.14/contributors.toml") }}

<!-- Changelog -->
{{ changelog(path="./release-content/0.14/changelog.toml")}}
```

The most important part of this is the `combine_release_notes` shortcode and the `load_markdown()` shortcode for the contributors and changelog. `combine_release_notes` will get the list of release notes from the `_release_notes.toml` file and combine all the separate file and add them to this file. `load_markdown()` will load a markdown file and add it to the blog post.
