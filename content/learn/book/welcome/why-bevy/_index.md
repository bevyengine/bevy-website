+++
title = "Why Bevy?"
weight = 1
template = "book-section.html"
page_template = "book-section.html"
+++

There are plently of great reasons to use Bevy, but here are just a few:

### ECS 
**TL;DR: Better workflow and abstraction in your code.**

**LR:**

Bevy has been designed to use a flexible data system in order to provide a smart, usable workflow. The engine uses an **Entity Component System** (ECS) to acheive this goal. In traditional game engines, a singular object (entity) contains its logic and components in one system. This is known as an object-oriented entity system.

This has its benifits, however, in general, it is being phased out by ECSs. Instead of containing all the systems and components in a singular entity, in ECS, all of this is abstracted into structs (in Rust) and functions. Entities are defined as unique integers. 

Components are structs that contain values for said components. Systems are Rust functions that are parsed into the main function and can specify components that an entity must have to be affected by the system.

**This can allow for a great amount of abstraction that help with S.O.L.I.D principles.**

### Modularity

**TL;DR: You can add and/or remove components you don't use.**

**LR:**

Bevy hinges on its plugin system, which allows for the most minimal of programs to be actually minimal in size. For example, if you are developing a console-based game, you may not need to include the ```WindowPlugin```, or if your game does not include audio, you can exclude the ```AudioPlugin```.

In addition, there are more plugins made by our excellent community, and you could even make your own with ease, if the one you are looking for doesn't exist.

This can potentially reduce the size of your application or game and give the engine even more functionality than built-in.

### Hot Reloading

**TL;DR: Easy file changes without recompilation**

**LR:** 

The Bevy Game Engine uses a **scene system** that allows for **hot reloading**. In very short, not technological terms, you have the ability to instantly change certain files (meshes, textures, scenes) and view the changes without recompiling.

This allows for quick iteration for artists and 3D modelers as well as less compile times.

### Much more!

There is much more Bevy is unique for, including,
- Fast Compile Times
- Modular, parallel render graph
- Flex Box Inspired UI system
- Flexible 2D and 3D renderers

## Why not Bevy?

All of that seems amazing, however you should bear in mind the following when attempting to develop in Bevy.

### MASSIVE stability problems

**TL;DR: API changes for days!**

**LR:**

Bevy is a relatively new engine, going public as early as August 2020, meaning that there is a **VERY** high chance vital parts of the engine can change almost every month (if not every week), especially if you are using the master branch of the source. This is why we recommend holding onto a specific hash when working on a project (we'll get into that later).

Using the engine for major projects as of right now is **HIGHLY DISCOURAGED** (at least until version 1.0 comes out). Trust us when we say the engine changes every update.

### Not feature-complete

**TL;DR: Half the features ain't in yet.**

**LR:**

This links back to the first point. The engine is so early in development that there is no editor and no mobile support, along with other things. So if that is a necessity in your project, we are currently a no-go.

A lot of cool features are coming, though. So don't lose intrest just yet.
