+++
title = "Creating Examples"
insert_anchor_links = "right"
[extra]
weight = 5
+++

Each example in Bevy must be:

1. **Goal-driven:** Each example must have a single clearly stated learning-focused goal.
2. **Working:** Every example must compile and run, and any intentionally introduced errors in them should be obvious (through tests, simple results, or clearly displayed behavior).
3. **Clear:** Examples should use descriptive variable names, be formatted, and be appropriately commented. Try your best to showcase best practices when it doesn't obscure the point of the example.
4. **Relevant:** Examples should explain, through comments or variable names, what they do and how this can be useful to a game developer.
5. **Minimal:** Examples should be no larger or complex than is needed to meet the goals of the example.

When you add a new example, be sure to update `examples/README.md` with the new example and add it to the root `Cargo.toml` file.
Run `cargo run -p build-templated-pages -- build-example-page` to do this automatically.
Use a generous sprinkling of keywords in your description: these are commonly used to search for a specific example.
See the [example style guide](#style-guide) to help make sure the style of your example matches what we're already using.

## Types of examples

[Examples in Bevy](https://github.com/bevyengine/bevy/tree/main/examples) in Bevy can be categorized into three distinct categories,
each of which have their own goals and subfolder:

1. API examples: clearly demonstrating a single feature or group of closely related small features.
2. Usage examples: teach users opinionated patterns for accomplishing common game development tasks.
3. Small games: focused on integrating these patterns into a cohesive whole while demonstrating how to structure larger projects.

In addition, we have two additional categories which are treated as examples by `cargo`, but are not intended to teach users:

1. Stress tests: uses extreme entity counts or otherwise unrealistic settings to allow us to readily detect performance changes in particular performance-sensitive hot paths.
2. Testbeds: designed to test features and scenes that rely on graphical rendering or interactivity, in both an automated and manual fashion.

Unlike examples, stress tests and test beds should be configurable, allowing Bevy contriubtors (and curious users) to see the impact of changes.

## When should I add an example?

Adding, maintaining and searching through examples has non-trivial costs: we need to think carefully about our selection.
Each of our example categories has its own goals which should be discussed separately.

A good API example is complementary to good API documentation, and should only be added to promote the discovery of important features, show API usage in context, or show off graphical, audible or interactive functionality that cannot be appropriately conveyed by written documentation alone.
When possible, prefer documentation tests and module documentation over API examples: these are easier to keep up to date, more clearly scoped and better connected to the APIs in question.

Usage examples tackle common tasks that might be encountered by veterans to game development (or CAD or...) that are trying to translate existing concepts or workflows into Bevy's APIs.
These should be focused on realism, building out the minimum required to demonstrate that they actually solve the problem at hand.
Usage examples can be more opinionated, and may demonstrate multiple possible approaches to the same problem, explaining the tradeoffs involved.

Game examples are much more expensive to maintain, and should be used sparingly. Each game example should demonstrate Bevy's capabilities in a new genre or style of game, while being simple enough for new Bevy users to follow and get excited about.
Polish is important here: these are often a new user's first impression of the engine, although it must be balanced against added complexity.

Each game example should be a few hundred to a few thousand lines at most.
Try to pick the simplest example that embodies the body or domain that you can, and strip it down to its essence.
The point is to prove to the reader that Bevy *can* make projects like this, not to actually make a project.

For example, suppose you wanted to demonstrate a turn-based RPG, analogous to Pokemon.
Implementing a single overworld level where you can walk around and a single battle with one character and no gameplay mechanics would be enough to get the idea across, and teach the reader how to architect their project.

As another example, consider making a first-person shooter game example.
A cross-hair, character controller, level, target dummy and the ability to fire a weapon would be enough.
Bullet decals, a weapon changing system, a scoreboard, respawning or so on are all valuable additions,
but generally won't have an architectural impact.
Instead, those features make more sense to demonstrate inside of dedicated usage examples in a lightweight way.

Full game templates or more complex demonstrations of functionality are also welcome, but to manage maintenance burden these should be submitted to [`bevy-assets`](https://github.com/bevyengine/bevy-assets), as part of our semi-curated collection of community-maintained learning resources.

## Style guide

Please adhere to the following guidelines when creating or updating an example.

### Organization

1. Examples should live in an appropriate subfolder of `/examples`.
   1. API examples live in `examples/api`, then subdivided by engine category.
   2. Usage examples live in `examples/usage`, then subdivided by domain.
   3. Game examples live in `examples/games`, with each game example getting its own dedicated subfolder.
   4. Stress tests live in the root level `stress-tests` folder.
   5. Testbeds live in the root level `tests` folder, subdivided by domain (e.g. `3d`).
2. Each example should consist of exactly one `.rs` file, except where assets or shaders are required.
   1. Game examples are an exception to this, and should be structured as self-contained small projects with a realistic file and module structure.
3. Assets live in `/assets`. Try to avoid adding new assets unless strictly necessary to keep the repository small. Don't add "large" asset files.
4. Each example should try to follow this order:
   1. Imports
   2. A `fn main()` block
   3. Example logic
5. Try to structure app / plugin construction in the same fashion as the actual code.
6. Examples should typically not have tests, as those are not directly reusable by the Bevy user.
7. Examples should not share common "utility" logic: if you feel this is warranted, improve the engine instead!
   1. Similarly, try to avoid incidental complexity and abstractions. Find a simpler way for now, and work to create these tools and resolve this tension in the engine itself.

### Dependencies

1. Examples should not take on new dependencies unless they are required for core functionality of that example.
2. Examples can never take on dependencies to Bevy ecosystem crates, to avoid circular dependencies and maintenance risk.
3. Any dependencies needed for examples should be listed as `dev-dependencies`.

### Stylistic preferences

1. Use simple, descriptive variable names.
   1. Avoid names like `MyComponent` in favor of more descriptive or evocative terms like `SparseComponent` or `Life`.
   2. Prefer single letter differentiators like `EventsA` and `EventsB` to nonsense words like `EventsFoo` and `EventsBar`.
   3. Avoid repeating the type of variables in their name where possible. For example, `Color` should be preferred to `ColorComponent`.
2. Prefer glob imports of `bevy::prelude::*` and `bevy::sub_crate::*` over granular imports (for terseness).
3. Use a consistent comment style:
   1. `///` doc comments belong above `#[derive(Trait)]` invocations.
   2. `//` comments should generally go above the line in question, rather than in-line.
   3. Avoid `/* */` block comments, even when writing long comments.
   4. Use \`variable_name\` code blocks in comments to signify that you're referring to specific types and variables.
   5. Start comments with capital letters; end them with a period if they are sentence-like.
4. Use comments to organize long and complex stretches of code that can't sensibly be refactored into separate functions.
5. Avoid making variables `pub` unless it is needed for your example.

### Code conventions

1. Refactor configurable values ("magic numbers") out into constants with clear names.
2. Prefer `for` loops over `.for_each`. The latter is faster (for now), but it is less clear for beginners, less idiomatic, and less flexible.
3. Use `.single` and `.single_mut` where appropriate.
4. In Queries, prefer `With<T>` filters over actually fetching unused data with `&T`.
5. Prefer disjoint queries using `With` and `Without` over param sets when you need more than one query in a single system.
6. Prefer structs with named fields over tuple structs except in the case of single-field wrapper types.
7. Use enum-labels over string-labels for app / schedule / etc. labels.

### Visual guidelines

Examples may be displayed in the [example showcase](https://bevyengine.org/examples/) and a consistent style helps keep things tidy there.

1. Use the default `ClearColor` and `WindowResolution` unless absolutely necessary.
2. "Instruction Text" should use the default font, color, and size. It should be inset 12 pixels from the edge of the window.

### "Feature" examples

These examples demonstrate the usage of specific engine features in clear, minimal ways.

1. Focus on demonstrating exactly one feature in an example.
2. Try to keep your names divorced from the context of a specific game, and focused on the feature you are demonstrating.
3. Where they exist, show good alternative approaches to accomplish the same task and explain why you may prefer one over the other.
4. Examples should have a visible effect when run, either in the command line or a graphical window.

### "Game" examples

These examples show how to build simple games in Bevy in a cohesive way.

1. Each of these examples lives in the `/examples/games` folder.
2. Aim for minimum but viable status: the game should be playable and not obviously buggy but does not need to be polished, featureful, or terribly fun.
3. Focus on code quality and demonstrating good, extensible patterns for users.
   1. Make good use of enums and states to organize your game logic.
   2. Keep components as small as possible but no smaller: all of the data on a component should generally be accessed at once.
   3. Keep systems small: they should have a clear single purpose.
   4. Avoid duplicating logic across similar entities whenever possible by sharing systems and components.
4. Use `///` doc comments to explain what each function / struct does as if the example were part of a polished production codebase.
5. Arrange your code into modules within the same file to allow for simple code folding / organization.
