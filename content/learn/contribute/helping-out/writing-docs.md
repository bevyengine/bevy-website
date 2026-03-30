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

There are two main places where we keep docs: inline docs alongside the codebase, and on Bevy's website.

### Inline Docs

The inline docs for each release are published to [docs.rs](https://docs.rs/bevy).

To view the current docs on `main` before you contribute, you can go to [dev-docs.bevy.org](https://dev-docs.bevy.org/),
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

We also consider [bevy.org](https://bevy.org) to be part of our core documentation. The website has [it's own repository](https://github.com/bevyengine/bevy-website) and is built using the Zola static site engine. In our experience, it is fast, flexible, and straightforward to use.

To check out any local changes you've made:

1. [Download Zola v0.19.2](https://www.getzola.org/).
2. Clone Bevy's website GitHub repository and enter that directory:
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

**The Quick Start Guide** is the intended first-stop for new Bevy users. We want to give an overview of core concepts and make new users feel excited and confident in using Bevy for their projects.  

**The Bevy Book** is intended to be an approachable, canonical, and comprehensive overview of concepts and patterns related to developing with Bevy. We want this to be on-par with other similar Books in the ecosystem such as [The Rust Book](https://doc.rust-lang.org/stable/book/), but this doesn't mean only Bevy veterans can contribute.  

When writing and reviewing content for The Bevy Book, The Quick Start Guide, and other related learning materials, please try to follow these guidelines:  

### Style and tone

1. Use clear, simple language.
   1. Use precise technical terms when needed, but avoid introducing jargon unnecessarily.
2. Speak directly to the reader in an approachable tone.
3. Prefer short sentences. Remove extra words.
4. Use "we" and "you" pronouns.
   1. Keep pronouns consistent within sections and chapters.
   2. When third-person pronouns are needed, prefer "they/them", unless named characters are introduced.
5. Avoid idioms and slang.
6. Keep humor light.
   1. Avoid off-color or offensive humor.
   2. Be mindful not to overuse in-jokes or cultural references.
   3. Don't drag your jokes out: that's not what the audience is here to read.

### Explanations

Documentation should explain things well.
We've prepared some tips to help you do that!

This advice applies broadly to Bevy's documentation, but is formulated with the Bevy Book in mind.
The Quickstart Guide in particular should assume a lower level of expertise.

1. Your target audience is intelligent, curious and new to Bevy.
   1. Basic programming knowledge can be assumed.
   2. Readers are expected to be familiar with basic Rust syntax and concepts.
   3. Advanced Rust concepts (e.g. `unsafe`, lifetimes, macros) cannot be assumed, and should be avoided where possible.
   4. Rust knowledge around best practices and development cannot be assumed, but detailed explanations should be given via links to other resources.
   5. Basic familiarity with the concepts and tropes of games can be assumed.
   6. Familiarity with the tools and concepts of game development cannot be assumed, but information that is not specific to Bevy should be kept brief in most cases.
2. Explanations of a concept should not be duplicated within different sections of the book.
   1. Link out, providing a one-sentence summary if needed.
   2. Our learning materials assume a roughly linear reading order: knowledge of previous chapters can be assumed, but should come with a link.
3. **Bold** new vocabulary words where they are first used.
   1. Define them as soon as is reasonable after they are introduced.
   2. Concepts from previous chapters should be linked, not bolded.
4. When introducing a new concept:
   1. Explain the concept and why it's useful, linking to the API docs.
   2. Demonstrate basic usage.
   3. Briefly discuss tradeoffs, advanced usage and limitations.
5. Examples should be as simple as possible without sacrificing realism.
   1. Every piece of code showcased should be something someone might reasonably write.
6. The polish level of your examples should correspond to the point you're trying to make.
   1. If you're demonstrating a new feature, show only the most basic syntax as locally as possible.
   2. When trying to explain how a game can be made, organize and polish your code to showcase best practices.
   3. Lack of polish should serve an end: don't show bad or sloppy practices without a good reason.
   4. Showing how (and why!) to refactor your code is a very powerful teaching tool.
7. When teaching about related concepts, create a running example or theme and build upon the previous example.
   1. Named characters are occasionally useful as a pedagogical device when demonstrating complex concepts.
8. When multiple related or competing tools exist, clearly compare and contrast them.
   1. You should be lightly opinionated about limitations and strengths.
   2. Focus on factual tradeoffs (performance, flexibility, ease of reasoning, ...) rather than style.
   3. Recommend a default approach where possible.
   4. Describe concrete cases where each approach would work well.
9. Interesting, tangentially related asides should use `info` callouts to inform readers.
   1. These callouts should always be skippable.
10. Brief warnings should use bolding for emphasis. Long, important warnings should use a `warning` callout.

### Copyediting

1. Make sure your grammar and spelling are correct.
   1. Bevy uses American English spelling.
2. Chapter and section titles use title case.
3. Bevy does not use footnotes or extended parentheticals.
   1. These should be cut, or converted into callouts.
4. The names used in the text should match the Rust types and function arguments discussed.
   1. The same applies to types, variables and arguments introduced in examples.
5. Minimize the use of explanatory comments inside of examples.
   1. Prefer to simplify when possible.
   2. Omit doc comments.
   3. Move important discussion outside of the example body.
6. Mathematical expressions should be accompanied by a plain English explanation of the significance.

### Organizational

1. Carefully organize your work into separate pages, headings, paragraphs, and code blocks.
2. Clearly signal when you are explaining a concept in technical depth so it can be skipped.
3. Use lists, numbered lists, and sub-lists to present information in bite-sized ways.
   1. Refer back to these items by number!
4. Provide plenty of links, but be sure that what you are linking to is obvious by context.
   1. Link to other sections of the book / example / web page when you mention them.
   2. Always link to the most specific location you can, whether that's a section on a page or a method on a struct.
   3. When referencing a Rust API by name, mark it as a link every time, not just the first time it appears on the page.
   4. For other links, link an url at most once per major section of the page, at first occurrence. Do not link it again in later sections unless contextually important.
   5. Use the `latest` tag when linking to Bevy docs and source code so it won't go stale every time the version is updated.
   6. Make sure the text reads well even when removing the link. For example, write "Start exploring [our examples](https://github.com/bevyengine/bevy/tree/latest/examples#examples)" rather than "Start exploring our examples [here](https://github.com/bevyengine/bevy/tree/latest/examples#examples)"
   7. When linking to detailed explanations or discussions, summarize the most important points in addition to providing a link.
   8. Do not place multiple links directly after another if they will look similar to a single link.
   9. To reference Rust API docs use markdown's reference-style links like so: [`HashMap`]. This keeps the raw text more readable and lets you reference the same url multiple times.
    
    ```md
    [`HashMap`]
    
    [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
    ```
    
    [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html

### Technical

1. The Bevy book always targets the latest released version of Bevy.
   1. Do not mention previous versions or the history of a feature.
   2. Do not mention upcoming features or changes, even if the corresponding work is merged.
   3. Mentioning current limitations is acceptable, but must come with an issue link.
2. The code in each example should be correct and up-to-date, but does not need to be able to be compiled and run.
   1. Setting up and maintaining self-isolated examples is time-consuming, and pollutes the example for editors. Only do this for code that you expect users to copy-paste directly. See `learning-code-examples/README.md` for how to do this.
   2. If distracting setup is needed in a checked example, add it to the start of the example and use `hide-lines` to hide the setup.
   3. To validate that the checked examples work, run:
   
       ```sh
       ./learning-code-examples/validate_examples.sh
       ```
3. Prefer game-relevant, descriptive examples and variable names over generic ones like `MyEvent`. Avoid meaningless names like `foo` at all times.
4. It's good practice to break your code into blocks with comments or explanatory text, but you need to link to a cohesive, copy-able whole at the end.
5. Examples must pass Bevy's standard `clippy` lints.
6. Stick to a consistent style (e.g. for loops vs map) within each example.
7. If you need to give advice that will only matter to some of your audience (e.g. how to handle an edge case, or support a specific platform), do so in a clearly marked aside (like a callout) or list.
8. Examples should not use or rely on third-party plugins. These may be appropriate to link in "next steps" however at the end of the examples.
   1. Third-party crates should be limited to the most essential, such as `rand`.
9.     To make sure your web-based files (html, markdown) are formatted correctly run the commands:
   
    ```sh
    markdownlint -f -c .github/linters/.markdown-lint.yml .
    djlint
    typos
    ```
   
    in the root directory of your local Bevy's website's repository.
   
    This will format markdown files and tell you the issues in HTML files. In order to run the command you should install `markdownlint-cli`, `djlint`, and `typos-cli`. See for installation: <https://github.com/igorshubovych/markdownlint-cli>, <https://www.djlint.com/docs/getting-started/>, and <https://github.com/crate-ci/typos?tab=readme-ov-file#install>.
    
    {% callout() %}
    The CI also includes `editorconfigchecker`, but there isn't an easy way to run this manually, so you should instead rely on CI to validate files with this tool.
    {% end %}
