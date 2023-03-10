+++
title = "Faq"
weight = 6
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
[extra]
long_title = "Frequently Asked Questions"
+++

### Why create Bevy when INSERT-GAME-ENGINE-HERE exists?

There are plenty of fantastic engines out there ... why build another one? Especially when there are already so many in the Rust ecosystem?

First a bit about me: I decided to build Bevy after years of contributing code to other engines (ex: Godot). I spent over four years [building a game in Godot](https://www.youtube.com/c/cartdev) and I also have experience with Unity, Unreal, and a number of other frameworks like SDL and Three.js. I have built multiple custom engines in the past using Rust, Go, HTML5, and Java. I have also used and/or closely followed most of the current players in the Rust gamedev ecosystem. I recently quit my job as Senior Software Engineer at Microsoft and my experience there deeply affected my opinions of software and what it should be.

These experiences led me to want the following from a game engine:

* **Free and Open Source**: It needs to be free and open source with _no strings attached_. Games are a huge part of our culture and humanity is investing _millions_ of hours into the development of games. Why are we (as game developers / engine developers) continuing to build up the ecosystems of closed-source monopolies that take cuts of our sales and deny us visibility into the tech we use daily? As a community I believe we can do so much better. This criteria eliminates Unreal and Unity, despite their huge feature sets.
* **Productive**: It needs to have fast build/run/test loops, which translates to either scripting languages or fast compile times in native languages. But scripting languages introduce runtime overhead, cognitive load, and a barrier between me and the actual engine, so my preference here is a native language with fast compile times. Sadly compile times are a huge problem in the Rust ecosystem and many Rust engines have prohibitively long iterative compiles. Fortunately Rust game engines like Macroquad and coffee prove that productive iterative compile times are possible.
* **Turtles All The Way Down**: Ideally the engine is written in the same language that games are. Being able to run an IDE "go to definition" command on a symbol in your game and hop directly into the engine source is an extremely powerful concept. You also don't need to worry about heavy language translation layers or lossy abstractions. If an engine's community builds games in the same language as the engine, they are more likely (and able) to contribute back to the engine.
* **Simple**: It needs to be easy to use for common tasks, but it also can't hide the details from you. Many engines are either "easy to use but too high level" or "very low level but difficult to do common tasks in". Additionally, many engines in Rust are littered with lifetimes and generics. Both are powerful tools to be sure, but they also introduce cognitive load and reduce ergonomics. Generics can also have a huge impact on compile times if you aren't careful.
* **Editor**: It needs to have an (optional) graphical editor. Scene creation is a large part of game development and in many cases visual editors beat code. As a bonus, the editor should be built _in the engine_. Godot uses this approach and it is _so smart_. Doing so [dogfoods](https://en.wikipedia.org/wiki/Eating_your_own_dog_food) the engine's UI system and creates positive feedback loops. Improvements to the editor are also often improvements to the core engine. It also makes sure your engine is flexible enough to build tooling (and not just games). I personally consider building an engine's editor in another stack to be a missed opportunity (ex: the web, QT, native widgets).
* **Data Driven**: It needs to be data-driven/data-oriented/data-first. ECS is a common way of doing this, but it definitely isn't the only way. These paradigms can make your game faster (cache friendly, easier to parallelize), but they also make common tasks like game state serialization and synchronization delightfully straightforward.

None of the engines on the market _quite_ line up with what I'm looking for. And the changes required to make them meet my requirements are either massive in scope, impossible (closed source), or unwelcome (the things I want aren't what the developers or customers want). On top of that, making new game engines is fun!

Bevy is not trying to out-compete other open-source game engines. As much as possible we should be collaborating and building common foundations. If you are an open source game engine developer and you think a Bevy component would make your engine better, one of your engine's components could make Bevy better, or both, please reach out! Bevy is already benefitting massively from the efforts of the Rust gamedev ecosystem and we would love to pay it forward in whatever way we can.
