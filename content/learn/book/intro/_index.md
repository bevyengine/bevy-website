+++
title = "Introduction"
template = "docs.html"
insert_anchor_links = "right"
[extra]
weight = 1
status = 'hidden'
+++

Bevy is a modern, ECS (Entity Component System)-first game engine written in Rust. We aim to be:

- **Capable:** Offer a complete 2D and 3D feature set.
- **Simple:** Easy for new users to pick up, but flexible enough for power users.
- **Modular:** Use only what you need. Replace what you don't like.
- **Fast:** App logic should run quickly, and when possible, in parallel.
- **Productive:** Short compile times and fast refreshes keep iteration tight.

Bevy is dual-licensed under [MIT](https://github.com/bevyengine/bevy/blob/main/LICENSE-MIT) and [Apache v2](https://github.com/bevyengine/bevy/blob/main/LICENSE-APACHE), and governed by the [Bevy Foundation](https://bevyengine.org/foundation/), a 501(c)(3) nonprofit. It is built in the open by hundreds of [community contributors](https://bevyengine.org/learn/contribute/introduction).

In practice, this means:

- You model game state as data in the ECS.
- You express behavior as systems that read and write that data.
- You compose functionality by combining small, modular pieces.
- You can inspect engine code, ecosystem code, and your game code using the same language and tooling.

This can feel unusual at first, especially if you are coming from object-oriented engines,
but don't be afraid: it'll click soon enough.

## Reading This Book

This book is [explanatory documentation](https://diataxis.fr/) covering Bevy's core ideas.
You can read it cover to cover, or skip ahead to the sections that interest you.

If you are new to Bevy, [read the first chapter] before proceeding.
It introduces the ECS vocabulary and core terms you'll need for the rest of this book.
After that, use the sidebar to jump to any chapter you need.
We've attempted to arrange them in a useful order, but they do not assume you have read previous chapters.

If you want a broader map of Bevy's documentation ecosystem (quick start guide, examples, API docs, and more), see [Further Reading](../further-reading).

If you're evaluating whether Bevy is the right fit for your project, read [Is Bevy Right for Your Project?](../is-bevy-right-for-your-project).

{% callout(type="info") %}
### Corrections and Extensions

Like any good piece of documentation, the Bevy Book is continuously updated.

If you spot a typo, [submit a quick PR](https://github.com/bevyengine/bevy-website/pulls).
The files used to create the book are just Markdown; you can find them in `content/learn/book`.
If you'd like to refactor something or add a new section, read our [Contributing Guide](https://bevyengine.org/learn/contribute/introduction/) and we'd be happy to welcome you to the flock!
{% end %}

[read the first chapter]: ./the-three-letters
