+++
title = "Writing Documentation"
insert_anchor_links = "right"
[extra]
weight = 5
+++

Like every other large, rapidly developing open source library you've ever used, Bevy's documentation can always use improvement.
This is incredibly valuable, and easily distributed, work that requires a bit of guidance:

* Inaccurate documentation is worse than no documentation; prioritize fixing broken docs.
* Bevy is remarkably unstable; before tackling a new major documentation project, check in with the community on Discord or GitHub (making an issue about specific missing docs is a great way to plan) about the stability of that feature and upcoming plans to save yourself heartache.
* Code documentation (doc examples and in the examples folder) is easier to maintain because the compiler will tell us when it breaks.
* Inline documentation should be technical and to the point. Link relevant examples or other explanations if broader context is useful.
* The Bevy Book is hosted on the `bevy-website` repository and targeted towards beginners who are just getting to know Bevy (and perhaps even Rust!).
* Accepted RFCs are not documentation; they serve only as a record of accepted decisions.

## Documentation Sources

There are two main places where we keep docs: inline docs alongside the codebase, and on the Bevy Website.

### Inline Docs

The inline docs for each release are published to [docs.rs](https://docs.rs/bevy).

To view the current docs on `main` before you contribute, you can go to [dev-docs.bevyengine.org](https://dev-docs.bevyengine.org/),
which has the latest API reference built from the repository on every commit made to the `main` branch.

To check out any local changes you've made:

1. Clone the Bevy Engine GitHub repository and enter that directory:
    1. `git clone https://github.com/bevyengine/bevy.git`
    2. `cd bevy`
2. Run `cargo doc --open`

Your web browser should open and you should be able to access a local version of the `docs.rs` page from there.

{% callout(type="warning") %}
The code in doc comments is compiled and tested to ensure that the examples work for our readers.
{% end %}

Doc-tests are run as part of the normal `cargo test` suite. To only run doc-tests, you can use `cargo test --doc`.

### The Website

We also consider [bevyengine.org](https://bevyengine.org) to be part of our core documentation. The website has [it's own repository](https://github.com/bevyengine/bevy-website) and is built using the Zola static site engine. In our experience, it is fast, flexible, and straightforward to use.

To check out any local changes you've made:

1. [Download Zola v0.18.0](https://www.getzola.org/).
2. Clone the Bevy Website GitHub repository and enter that directory:
    1. `git clone https://github.com/bevyengine/bevy-website.git`
    2. `cd bevy-website`
3. Start the Zola server with `zola serve`.

A local server should start and you should be able to access a local version of the website from there.

{% callout(type="warning") %}
The code in the learning materials (e.g., The Bevy Book, The Quick Start Guide, Advanced Examples, etc.) is compiled, formatted, and tested to make sure that the examples work for readers.

To validate your code snippets either run `validate_examples.sh` which resides in the `learning-code-examples` directory (and is the recommended way to use `learning-code-examples`), or from the root of the project run `cd learning-code-examples && cargo check --examples && cargo clippy --examples && cargo fmt --check`.
The code in the book is compiled and tested to make sure that the examples work for readers.
{% end %}

{% callout() %}
[See the `learning-code-examples` README.md for more info.](https://github.com/bevyengine/bevy-website/blob/main/learning-code-examples/README.md)
{% end %}

## Learning material structure

As you have probably noticed, our introductory learning material is split into two main sections:

1. **The Quick Start Guide:** "Get started making your first game now!"
2. **The Bevy Book:** "Understand how Bevy works, and how you can use it"

This is intended to cater to two different types of learners, without compromising the experience for either:

* **Example-first:** These users want to dive right in, see everything in action, and get a working game as quickly as possible.
These users often have an idea in their mind that they want to start prototyping as quickly as possible.
* **Definition-first:** These users want to carefully build up a mental model of Bevy, thoroughly understanding each new concept before moving on.
These users tend to be driven by curiosity, or are aiming to carefully develop a new skill.

Crucially, these paths are independent of the experience levels of the learner!
Bevy intentionally aims to be inclusive of both complete beginners who have never programmed before, and professionals coming from other engines.

|                      | **Beginner**                                                       | **Professional**                                                     |
| -------------------- | ------------------------------------------------------------------ | -------------------------------------------------------------------- |
| **Example-first**    | Enthusiastic, wants to create a new version of the game they love. | Exploratory, wants to dive in and see how Bevy holds up in practice. |
| **Definition-first** | Curious, wants to understand how making games works.               | Critical, wants to understand Bevy's unique design choices.          |

Each of these requires their own complementary learning paths that branch as soon as they get to the [Learn page](@/learn/_index.md) to ensure that the first experience that they have with Bevy matches what they need.

Once users have completed the introductory learning materials in their path of choice, they can begin creating their own games or move on to our advanced examples to see how everything comes together in a realistic way.

### Bevy Quick Start: the example-first path

Users following the example-first path will tend to take the following route:

1. Encounter the Bevy homepage due to social media or word of mouth.
2. Navigate to the Learn page.
3. Choose one of the most relevant **quick start games**.
4. Complete that tutorial.
5. Dive into making the game they have in mind, accessing the following resources as needed when they encounter road-blocks:
   1. Official examples.
   2. The Bevy Book.
   3. Community tutorials and template games.
   4. Various community support forums.
   5. Streams, videos, and blogs.
   6. Advanced examples.

Each quick start game should:

1. Assume zero existing knowledge of Bevy.
2. Begin with a initial high-level explanation of what we're trying to build.
3. Introduce commented code first, then explain what each of the critical terms means as they come up.
4. Be broken into compilable, playable sections; one per page of the guide.
5. Gradually refactor the code to add more functionality.
6. End with a list of suggestions (with appropriate links) on how you could extend the game in interesting ways.

This path should prioritize:

1. Rapid time-to-fun.
2. Functional, good-enough explanations that are tied to the code in front of them.
3. Relevance of quick start game to the genre of game they want to make.
4. High asset quality.
5. Ease of extending the quick-start game with their own tweaks.
6. Explaining how to get unstuck through documentation, community help, and filing issues.

### The Bevy Book: the definition-first path

Users following the definition-first path will tend to take the following route:

1. Encounter the Bevy homepage due to social media or word of mouth.
2. Navigate to the Learn page.
3. Select **The Bevy Book**.
4. Read through the book, largely in order.
5. Once they feel they have a good enough understanding of the engine, they will begin to make their own games, typically by jumping over onto the example-first path.
6. As they explore, they will also browse:
   1. The source code.
   2. [docs.rs](https://docs.rs/bevy/)
   3. The Contributing Guide, GitHub issues, and pull requests.
   4. Release notes.
   5. The engine development channels on Discord.
   6. Advanced examples to see how everything comes together.

Each chapter of The Bevy Book should:

1. Have a clear topic, and give a high-level overview of the subtopics it is going to cover and how they fit together.
2. Be broken down into several sections / pages to focus on detailed topics.
   1. These should have simple, minimal examples explaining how that functionality works.
3. Link to appropriate sections of quick start guides that demonstrate the ideas being taught in a more coherent way.

This path should prioritize:

1. Clear, thorough explanations.
2. Carefully introducing one concept at a time in an organized fashion.
3. Connecting concepts to each other in context.
4. Explaining the technical details of how things work, but only in clearly marked asides.
5. Communicating all of the supporting development practices that make Bevy productive:
   1. How to set up your development environment.
   2. Code organization.
   3. Design patterns and best practices.
   4. Testing, bench-marking, and debugging.
   5. Contributing to Bevy itself.
6. Linking to further reading such as, official examples, `docs.rs`, and (very sparingly) source code links.

## Contributor's style guide

When writing and reviewing content for The Bevy Book, The Quick Start Guide, and other related learning materials, please try to follow these guidelines:

### Writing

1. Use clear, simple language.
2. Prefer short sentences. Remove extra words.
3. **Bold** new vocabulary words where they are defined.
   1. Define them as soon as is reasonable after they are introduced.
4. Make sure your grammar and spelling are correct.
5. Avoid idioms and slang.
6. Speak directly to the reader in an approachable tone. Use "we" and "you" pronouns.
7. It can be useful to create specific, named characters to demonstrate a point.
   1. If you do, pick a pronoun set for them and stick to it.
   2. Otherwise, use  "they/them" third-person pronouns to refer to the reader or others.
8. Keep humor light.
   1. Avoid off-color or offensive humor.
   2. Be mindful not to overuse in-jokes or cultural references.
   3. Don't drag your jokes out: that's not what the audience is here to read.

### Organizational

1. Carefully organize your work into separate pages, headings, paragraphs, and code blocks.
2. Clearly signal when you are explaining a concept in technical depth so it can be skipped.
3. Use lists, numbered lists, and sub-lists to present information in bite-sized ways.
   1. Refer back to these items by number!
4. Provide plenty of links, but be sure that what you are linking to is obvious by context.
   1. Link to other sections of the book / example / web page when you mention them.
   2. Always link to the most specific location you can, whether that's a section on a page or a method on a struct.
   3. Use the `latest` tag when linking to Bevy docs and source code so it won't go stale every time the version is updated.
   4. When linking to detailed explanations or discussions, summarize the most important points in addition to providing a link.

### Technical

1. All examples must be able to be compiled and run.
2. Prefer game-relevant, descriptive examples and variable names over generic ones like `MyEvent`. Avoid meaningless names like `foo` at all times.
3. It's good practice to break your code into blocks with comments or explanatory text, but you need to link to a cohesive, copy-able whole at the end.
4. Examples must pass Bevy's standard `clippy` lints.
5. The polish level of your examples should correspond to the point you're trying to make.
   1. If you're demonstrating a new feature, show only the most basic syntax as locally as possible.
   2. When trying to explain how a game can be made, organize and polish your code to showcase best practices.
   3. Lack of polish should serve an end: don't show bad or sloppy practices without a good reason.
   4. Showing how (and why!) to refactor your code is a very powerful teaching tool.
6. Stick to a consistent style (e.g. for loops vs map) within each example.
7. If you need to give advice that will only matter to some of your audience (e.g. how to handle an edge case, or support a specific platform), do so in a clearly marked aside (like a callout), or list.
8. Examples should not use or rely on third-party plugins.
These may be appropriate to link in "next steps" however at the end of the examples.
   1. Third-party crates should be limited to the most essential, such as `rand`.
9. To validate local code changes you can either `./learning-code-examples/validate_examples.sh` from anywhere, or from the project's root `cd learning-code-examples && cargo check --examples && cargo clippy --examples && cargo fmt --check`.
10. To make sure your web-based files (html, markdown) are formatted correctly run the commands:

    ```sh
    markdownlint -f -c .github/linters/.markdown-lint.yml .
    djlint
    typos
    ```

    in the root directory of your local Bevy Website repository. This will format markdown files and tell you the issues in HTML files. In order to run the command you should install `markdownlint-cli`, `djlint`, and `typos-cli`. See: <https://github.com/igorshubovych/markdownlint-cli>, <https://www.djlint.com/docs/getting-started/>, and <https://github.com/crate-ci/typos?tab=readme-ov-file#install>.  Note that the CI also includes `editorconfigchecker` but there isn't an easy way to run this manually, so you should instead rely on CI to validate files with this tool.
11. To reference Rust API docs you can use markdown's reference-style links like so:
   [`HashMap`]

   ```md
   [`HashMap`]

   [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
   ```

   [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
