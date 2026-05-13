+++
title = "Explaining Examples"
insert_anchor_links = "right"
[extra]
weight = 6
+++

Writing up an explanation for an example can be helpful for providing context and **telling a story** of how problems are solved by combining different features of Bevy.

## Where to write explanations

Example explanations should be **in the .rs file of the example**. This keeps them directly tied to the example's code, which will keep eyes on the explanation when changes are made to the code itself.

## Style guide

Everything in [Writing Documentation](../writing-docs) also applies here, with some context-specific tweaks as the context for examples is different.

{% callout(type="warning") %}
There is no predictable order to how people visit the examples pages, people will look for what they need in the moment and then go to it. No page can assume that another page has been read first, or at least if it does it may need to link directly to that other page.
{% end %}

### Building a narrative and avoiding rote repetition

A subject should be identified for an example so that the narrative can be built around it. If someone is writing an explanation on an example that is only about "Setting up the camera" then there's plenty of space to give to the details of setting up the camera and all the possible things one could draw on and why. But we would not want to overemphasise extra things we put in the scene to show that the camera is in fact working. Similarly, for a UI example we will want to explain through a narrative of why we want to achieve making a UI this way within a common, abstracted context for building a UI in games.

A **non-goal** of examples is to give a blurb for every part of the engine and combine relevant blurbs together on each example page. This form of documentation is best left for what the API docs do, and an explanation built this way eliminates the chance to explain context. Examples are an opportunity to show Bevy-in-context, already proving itself by running in the browser with the code right next to it.

### What's in a paragraph

Paragraphs should ideally cover **1 concept/topic** before moving on and be fairly short (generally 50 to 150 words, this is not a hard rule and neither minimising or maximising is "better"). This is to benefit **skim-reading** as a way of finding information quickly and then being able to properly contextualise it once you've found something that resembles what you're looking for.

Subjects of paragraphs should usually be name dropped at the beginning of those paragraphs, again this serves the purpose of skim-reading by having there be a semi-reliable area where a concept's name is dropped.

### Who looks at examples

The demographic assumptions are a bit different than in [Writing Documentation](../writing-docs#learning-material-structure) as we are assuming that people are already navigating the example pages.

There are 3 kinds of people we're aiming to write this kind of documentation for, each with their own reasons to visit the Bevy example pages:

1. Beginners to games programming.
    1. Curious about niche engines.
    2. Missing a lot of context on why game engine design decisions are made.
    3. Already ingesting a lot of new information.
    4. May be making faulty assumptions about what game engines do.
    5. May have an ambitious project in mind they're contextualising information around.
2. Intermediates.
    1. Experienced with game development / games programming.
    2. May be looking for analogues to how things are done in other engines.
3. Professionals.
    1. Knows what kind of feature or tool they need.
    2. Needs confidence that Bevy is able to appeal to their advanced / specific needs.

For **beginners** we want to be explaining things in an accessible way that shows the broader Bevy toolset and its context, for **intermediates** we want to avoid being condescending, and for **professionals** we want to be showing breadcrumbs that appeal to their use cases, which has the benefit of showing less experienced users a path to expertise.

Some things should be explained so that someone new to games programming has the relationships between concepts briefly given context to, while more complicated Bevy features and more complex use-cases should be lightly name dropped to act as this "mountain" for the user to "climb."

### Rust experience

Experience with Rust should be assumed to be intermediate. If something complex is happening with Rust we shouldn't be afraid to explain it, but it should be brief and in the context of a specific Bevy feature. Rust doesn't need to be over-explained.

## Questions to ask when planning an explanation

1. What are the key concepts in the example?
2. Whose eye might be caught by the example?
    1. Imagine the assumptions one would make when looking at an example and what they would want to get out of it, across skill sets and amount of experience.
3. Is the topic an "advanced" feature or something plug-and-play?
    1. Does the technology need to be explained, or just how to use it?
    2. Something that requires an already existing deep knowledge of another domain may only need to nod towards that domain, not fully explain it.
4. Does the explanation you're writing also line up with the [general documentation style guide](../writing-docs)?

## Explanation rot

Bevy is young, it changes quickly. The intention for explanations that follow this specification (at the very least until Bevy reaches stability) is that they are disposable. Through ["wrong docs are worse than no docs"](../writing-docs) a catastrophically wrong example explanation should be assumed to be better deleted than rewritten (for instance, a massive change between versions with no capacity to address all changes in the docs) but this doesn't mean an outdated explanation is automatically unrecoverable.

By splitting up concepts across paragraphs there should be a clear path to re-writing an explanation. Catastrophically wrong example explanations between Bevy versions **should** be pretty infrequent because of this, but should be noted when they happen.
