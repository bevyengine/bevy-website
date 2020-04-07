+++
title = "Faq"
weight = 4
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
[extra]
long_title = "Frequently Asked Questions"
+++

### Why build Bevy instead of using INSERT-GAME-ENGINE-HERE?

@cart (original creator of Bevy) speaking: I decided to build Bevy after years of contributing code to other engines (ex: Godot). I spent over four years building a game in Godot and I have experience with Unity, Unreal, Three.js, Armory, and Game Maker. I built multiple custom engines using Rust, Go, HTML5, and Java. I have also closely followed the other major players in the Rust gamedev ecosystem, namely <a href="https://github.com/amethyst/amethyst" target="_blank">Amethyst</a>, <a href="https://github.com/hecrj/coffee" target="_blank">coffee</a>, and <a href="https://github.com/PistonDevelopers/piston" target="_blank">Piston</a>. I used to be a Senior Software Engineer at Microsoft which has also affected my opinions of software and what it should be.

These experiences led me to want the following from a game engine:

* It needs to be free and open source with no strings attached. Games are a huge part of our culture and humanity is investing _millions_ of hours into the development of games. Why are we (as game developers / engine developers) continuing to build up the ecosystems of closed-source monopolies that take cuts of our sales and deny us visibility into the tech we use daily? As a community I believe we can do so much better.
* It needs to have fast build/run/test loops, which translates to either scripting languages or fast compile times in native languages. But scripting languages introduce runtime overhead, cognitive load, and a barrier between me and the actual engine, so my preference here is a native language with fast compile times. 
* Ideally the engine is written in the same language as games are. Being able to run an IDE "go to definition" command on a symbol in your game and hop directly into the engine source is an extremely powerful concept. And you don't need to worry about translation layers or lossy abstractions. Also, if an engine's community builds games in the same language as the engine, they are more likely (and able) to contribute back to the engine.
* It needs to be easy to use for common tasks, but it also can't hide the details from you. Many engines are either "easy to use but too high level" or "very low level but difficult to do common tasks in".
* It needs to have an editor. Scene creation is a large part of game development and in many cases visual editors beat code. As a bonus, the editor should be built _in the engine_. Godot uses this approach and it is _so smart_. Doing so <a href="https://en.wikipedia.org/wiki/Eating_your_own_dog_food" target="_blank">dogfoods</a> the engine's UI system and creates positive feedback loops. Improvements to the editor are also often improvements to the core engine. And likewise community UI development can feed back into a better editor. It also makes sure your engine is flexible enough to build tooling (and not just games).
* It needs to be data-driven/data-oriented/data-first. ECS is a common way of doing this, but it definitely isn't the only way. These paradigms can make your game faster (cache friendly, easier to parallelize), but they also make common tasks like game state serialization and synchronization delightfully straightforward.

None of the engines on the market _quite_ line up with what I'm looking for. And the changes required to make them meet my requirements are either massive in scope, impossible (closed source), or unwelcome (the things I want aren't what the developers or customers want). On top of that, making new game engines is fun!

Bevy is not trying to out-compete other open-source game engines. As much as possible we should be collaborating and building common foundations. If you are an open source game engine developer and you think a Bevy component would make your engine better, one of your engine's components could make Bevy better, or both, please reach out! Bevy is already benefitting massively from the efforts of the Rust gamedev ecosystem and we would love to pay it forward in whatever way we can.