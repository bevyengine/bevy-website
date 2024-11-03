+++
title = "Creating Examples"
insert_anchor_links = "right"
[extra]
weight = 5
+++

Most [examples in Bevy](https://github.com/bevyengine/bevy/tree/main/examples) aim to clearly demonstrate a single feature, group of closely related small features, or show how to accomplish a particular task (such as asset loading, creating a custom shader, or testing your app).
In rare cases, creating new "game" examples is justified in order to demonstrate new features that open a complex class of functionality in a way that's hard to demonstrate in isolation or requires additional integration testing.

Examples in Bevy should be:

1. **Working:** They must compile and run, and any intentionally introduced errors in them should be obvious (through tests, simple results, or clearly displayed behavior).
2. **Clear:** They must use descriptive variable names, be formatted, and be appropriately commented. Try your best to showcase best practices when it doesn't obscure the point of the example.
3. **Relevant:** They should explain, through comments or variable names, what they do and how this can be useful to a game developer.
4. **Minimal:** They should be no larger or complex than is needed to meet the goals of the example.

When you add a new example, be sure to update `examples/README.md` with the new example and add it to the root `Cargo.toml` file.
Run `cargo run -p build-templated-pages -- build-example-page` to do this automatically.
Use a generous sprinkling of keywords in your description: these are commonly used to search for a specific example.
See the [example style guide](#style-guide) to help make sure the style of your example matches what we're already using.

More complex demonstrations of functionality are also welcome, but these should be submitted to [`bevy-assets`](https://github.com/bevyengine/bevy-assets).

## Style guide

Please adhere to the following guidelines when creating or updating an example.

### Organization

1. Examples should live in an appropriate subfolder of `/examples`.
2. Examples should be a single file if possible.
3. Assets live in `/assets`. Try to avoid adding new assets unless strictly necessary to keep the repository small. Don't add "large" asset files.
4. Each example should try to follow this order:
   1. Imports
   2. A `fn main()` block
   3. Example logic
5. Try to structure app / plugin construction in the same fashion as the actual code.
6. Examples should typically not have tests, as those are not directly reusable by the Bevy user.

### Stylistic preferences

1. Use simple, descriptive variable names.
   1. Avoid names like `MyComponent` in favor of more descriptive terms like `Events`.
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
