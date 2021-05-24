# Writing the Bevy book

So, you want to help write some learning material for the [Bevy website](https://bevyengine.org/)?

As you probably noticed, our introductory learning material is split into two main sections:

1. **Bevy Quick Start:** "Get started making your first game now!"
2. **Bevy Book:** "Understand how Bevy works, and how you can use it"

This is intended to cater to two different types of learners, without compromising the experience for either:

- **Example-first:** These users want to dive right in, see everything in action and get a working game as quickly as possible.
These users often have an idea in their mind that they want to start prototyping as quickly as possible.
- **Definition-first:** These users want to carefully build up a mental model of Bevy, thoroughly understanding each new concept before moving on.
These users tend to be driven by curiosity, or are aiming to carefully develop a new skill.

Crucially, these paths are independent of the experience levels of the learner!
Bevy intentionally aims to be inclusive of both complete beginners who have never programmed before, and professionals coming from other engines.

|                      | **Beginner**                                                       | **Professional**                                                     |
| -------------------- | ------------------------------------------------------------------ | -------------------------------------------------------------------- |
| **Example-first**    | Enthusiastic, wants to create a new version of the game they love. | Exploratory, wants to dive in and see how Bevy holds up in practice. |
| **Definition-first** | Curious, wants to understand how making games works.               | Critical, wants to understand Bevy's unique design choices.          |

Each of these requires their own complementary learning paths that branch as soon as they get to the [Learn page](https://bevyengine.org/learn/) to ensure that the first experience that they have with Bevy matches what they need.

Once users have completed the introductory learning materials in their path of choice, they can begin creating their own games or move on to our advanced examples to see how everything comes together in a realistic way.

## Bevy Quick Start: the example-first path

Users following the example-first path will tend to take the following route:

1. Encounter the Bevy homepage due to social media or word of mouth.
2. Navigate to the Learn page.
3. Choose one of the most relevant **quick start games**.
4. Complete that tutorial.
5. Dive into making the game they have in mind, accessing the following resources as needed when they encounter road-blocks:
   1. Official Examples.
   2. The Bevy book.
   3. Community tutorials and template games.
   4. Various community support forums.
   5. Streams, YouTube channels and blogs.
   6. Advanced examples.

Each quick start game should:

1. Assume zero existing knowledge of Bevy.
2. Begin with a initial high-level explanation of what we're trying to build.
3. Introduce commented code first, then explain what each of the critical terms means as they come up.
4. Be broken into compilable, playable sections: one per page of the guide.
5. Gradually refactor the code to add more functionality.
6. End with a list of suggestions (with appropriate links) on how you could extend the game in interesting ways.

This path should prioritize:

1. Rapid time-to-fun.
2. Functional, good-enough explanations that are tied to the code in front of them.
3. Relevance of quick-start game to the genre of game they want to make.
4. High asset quality.
5. Ease of extending the quick-start game with their own tweaks.
6. Explaining how to get unstuck, through documentation, community help and filing issues.

## The Bevy Book: the definition-first path

Users following the definition-first path will tend to take the following route:

1. Encounter the Bevy homepage due to social media or word of mouth.
2. Navigate to the Learn page.
3. Select the **Bevy book**.
4. Read through the book, largely in order.
5. Once they feel they have a good enough understanding of the engine, they will begin to make their own games, typically by jumping over onto the example-first path.
6. As they explore, they will also browse:
   1. The source code.
   2. [docs.rs](https://docs.rs/bevy/)
   3. CONTRIBUTING.md, GitHub issues and pull requests.
   4. Release notes.
   5. The engine development channels on Discord.
   6. Advanced examples to see how everything comes together.

Each chapter of the Bevy Book should:

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
   1. How to set up your dev environment.
   2. Code organization.
   3. Design patterns and best practices.
   4. Testing, benchmarking and debugging.
   5. Contributing to Bevy itself.
6. Linking to further reading: official examples, `docs.rs` and (very sparingly) source code links.

## Contributor's style guide

When writing and reviewing learning material for the Bevy Book and Quick Start Games, please try to follow these guidelines:

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

1. Carefully organize your work into separate pages, headings, paragraphs and code blocks.
2. Clearly signal when you are explaining a concept in technical depth so it can be skipped.
3. Use lists, numbered lists and sub-lists to present information in bite-sized ways.
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
7. If you need to give advice that will only matter to some of your audience (e.g. how to handle an edge case, or support a specific platform), do so in a clearly marked aside or list.
8. Examples should not use or rely on third-party plugins.
These may be appropriate to link in "next steps" however at the end of the examples.
   1. Third-party crates should be limited to the most essential, such as `rand`.
