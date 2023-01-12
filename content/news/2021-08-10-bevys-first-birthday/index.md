+++
title = "Bevy's First Birthday"
date = 2021-08-10
[extra]
author = "Carter Anderson"
twitter = "cart_cart"
github = "cart"
youtube = "cartdev"
image = "bevy_birthday.svg"
show_image = true
+++

[@cart](https://www.twitter.com/cart_cart) here (Bevy's creator, lead developer, and project manager) with some exciting news:

Today is Bevy's first birthday! And what a year it has been! Now seems like as good a time as any to look back on how far we've come, reflect a bit, and start thinking about what the next year of Bevy development will look like.

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. Bevy is also free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. We have a [Quick Start Guide](/learn/book/getting-started/) and a [Bevy Book](https://bevyengine.org/learn/book/introduction/). You can also check out [Bevy Assets](https://bevyengine.org/assets/) for a library of community-developed plugins, crates, games, and learning resources.

<!-- more -->

## A Year of Milestones

![milestones](milestones.svg)

* **August 10**: [Bevy 0.1](/news/introducing-bevy/)
  * Bevy's first public release! After months of working incognito, I released Bevy to the world. It was by no means complete, but it had most of the pillars in place to show the world what Bevy is (and could be): a modern and flexible renderer built on top of a modular Render Graph, a custom ECS with unrivaled ergonomics and competitive performance, 2D and 3D rendering features, asset handling, a modular app model that blurs the lines between engine developers and app developers, a custom UI system that integrates deeply with the engine, scenes, hot reloading, and blissfully productive iterative compile times.
* **August 19**: [Absolutely Wild Public Reception](/news/scaling-bevy/)
  * Just a week after release we became the 3rd most popular /r/rust post of all time, hit #2 on Hacker News, received 2,200 Github stars, merged pull requests from 26 new contributors, gained 644 Discord members, and received [sponsorships](https://github.com/sponsors/cart) that brought us 37% of the way to our first funding goal.
* **August 20**: [Reached our first funding goal ($1500 / month)](https://github.com/sponsors/cart)
  * [Embark became our first platinum sponsor](https://twitter.com/BevyEngine/status/1296525644004593664), which brought us past our first funding goal, allowing me to work on Bevy full time without eating into my savings. This honestly set the course for the rest of the year of Bevy development.
* **August 20**: [The Amethyst forum post: "Bevy Engine - Addressing the elephant in the room"](https://community.amethyst.rs/t/bevy-engine-addressing-the-elephant-in-the-room/)
  * This helped establish the strengths of each project and identified various collaboration areas, marking the beginning of a fruitful relationship between the two communities.
* **August 28**: [Reached our second funding goal ($2080 / month)](https://github.com/sponsors/cart)
  * As of this point, I'm making "state of Washington minimum wage" working on Bevy. This marks the point where I start thinking about building and managing Bevy as "my job".  
* **September 19**: [Bevy 0.2](/news/bevy-0-2/)
  * A month after the initial release we dropped another big one! This included a new async task system with significantly improved performance, initial Web platform support, parallel queries, a new transform system, joystick input, and some tasty Bevy ECS performance improvements.
* **November 3**: [Bevy 0.3](/news/bevy-0-3/)
  * Another month after Bevy 0.2 we had _another_ big release! This one added initial Android and iOS support, WASM asset loading, touch input, asset reference counting, dependencies, and sub assets, GLTF scene loading, Bevy ECS query ergonomics, 100% lockless parallel ECS and other performance improvements, flexible mesh attributes, another transform system rewrite, gamepad settings, plugin groups, and dynamic window settings.
* **December 19**: [Bevy 0.4](/news/bevy-0-4/)
  * We somehow still managed to keep up the "approximately once a month" release cadence. We added a WebGL2 render backend, cross platform main functions, live shader reloading, flexible ECS parameter orders, simplified query filters, system inputs, outputs, and chaining, a more featureful and flexible ECS schedule implementation, "fixed timesteps", states, gltf improvements, spawning scenes as children, dynamic linking for _drastically_ faster iterative compile times, a new text layout implementation, renderer optimizations, a new rust reflection crate (filling a major gap in the rust ecosystem), 3d texture assets, logging and profiling, hidpi rendering, timer improvements, task system improvements, and apple silicon support.
* **April 6**: [Bevy 0.5](/news/bevy-0-5/)
  * The almost-one-release-a-month streak finally ended. But thats not because we slowed down our pace! This release was a big one. It added Physically Based Rendering (PBR), GLTF asset improvements, Bevy ECS V2: a complete from-scratch rewrite with a novel archetype/sparse-set hybrid storage model, an "archetype graph" for faster archetype changes, query caching, uber fast for-each iterators, a new system executor with system labels, explicit system ordering/dependencies, system sets, and increased parallelism, "reliable" change detection, and a full rewrite of the State system). We also added a rich text api, hidpi and 2d-world space text, world-to-screen coordinate conversions, a 3d orthographic camera and new scaling modes, flexible camera bindings in shaders, render layers, sprite flipping, color spaces, wireframe rendering, and more smaller tweaks that I don't have room for here.
* **April 14**: [The Bevy RFC process is unveiled](https://github.com/bevyengine/rfcs)
  * Inspired by the Rust RFC process, we added a way to collaboratively design and review Bevy APIs prior to implementing them. This generally isn't required, but for bigger changes it ensures we think deeply about what we are building, mitigates risk, and encodes designs and intents for future Bevy developers.
* **June 1**: [First public release of Bevy Assets](https://twitter.com/BevyEngine/status/1399891316939448320)
  * [Bevy Assets](https://bevyengine.org/assets/) is a public library of community developed Bevy plugins, crates, assets, games, and learning materials. The website is fed by structured toml files in the [bevy-assets repo](https://github.com/bevyengine/bevy-assets). It has its roots in the awesome-bevy repo, our old unstructured markdown document with a list of community projects. It is still hot off the presses, but we have big plans for it!  
* **June 24**: [Reached our third funding goal ($4000 / month)](https://github.com/sponsors/cart)
  * Reaching this goal marked the point where I started thinking about Bevy as a career. I'm not making "market rate" for my skills and I'm still making less than 1/4th what I made as a Senior Software Engineer at Microsoft, but I'm no longer "just breaking even" and I'm starting to save some money.
* **August 2**: [Bevy hits 10,000 stars on Github](https://twitter.com/cart_cart/status/1422393321394085888)
  * I honestly can't believe we hit this so quickly.
* **August 10**: Bevy is now one year old!

## A Year By The Numbers

![numbers](numbers.svg)

* **255** unique Bevy contributors on [Github](https://github.com/bevyengine)
* **10,030** [Github](https://github.com/bevyengine) stars
* **837** forks on [Github](https://github.com/bevyengine)
* **1,501** pull requests (1060 merged) on [Github](https://github.com/bevyengine)
* **1,112** issues (609 closed) on [Github](https://github.com/bevyengine)
* **1,895** commits on [Github](https://github.com/bevyengine)
* **153**  [Github Discussions](https://github.com/bevyengine/bevy/discussions)
* **110** [Bevy Assets](https://bevyengine.org/assets/) (plugins, crates, games, apps, and learning materials)
* **57,349** downloads on [crates.io](https://crates.io/crates/bevy)
* **93** [@BevyEngine](https://twitter.com/BevyEngine) retweets of Bevy community content on Twitter
* **4,871** [Bevy Discord](https://discord.com/invite/bevy) members
* **771** community #showcase entries in the [Bevy Discord](https://discord.com/invite/bevy)
* **420,250** messages in the [Bevy Discord](https://discord.com/invite/bevy)

## Things I'm Proud Of

![proud](proud.svg)

### _Our_ Engine

I'm _so_ happy that Bevy quickly went from being "my engine" to being "our engine". It was beautiful to see so many people find passion projects in Bevy. Take a look at the feature author lists in the release blog posts (after Bevy 0.1). This community is _huge_, _wildly_ productive, and _intensely_ collaborative!

Our community is also extremely welcoming: we have an active [Github Q&A platform](https://github.com/bevyengine/bevy/discussions/categories/q-a) and [#help channel](https://discord.com/invite/bevy) on Discord (with over 82,000 messages). If you haven't already, [hop in to our Discord](https://discord.com/invite/bevy) and say hi!

### Development Pace

Just look at all of the releases above and the sheer size of their feature sets! The upcoming Bevy 0.6 is gearing up to be another big one and I doubt we'll slow down any time soon. The results speak for themselves so I'll just shut up now.

### Experimentation and Iteration

We didn't just build things once and assume that was the best we could do. When we started [I called out](/news/scaling-bevy/#what-made-bevy-good) that we needed the freedom to experiment and iterate. I put a big "stability warning" wherever I could to help manage expectations. And we sure did iterate! We [rewrote the transform system](https://bevyengine.org/news/bevy-0-2/#transform-system-rewrite) ... [twice](https://bevyengine.org/news/bevy-0-3/#transform-re-rewrite). We [completely rewrote Bevy ECS from scratch](https://bevyengine.org/news/bevy-0-5/#bevy-ecs-v2), leaving our hecs ECS roots behind (but not forgotten). We have a [brand new renderer](https://github.com/bevyengine/bevy/issues/2535) in the works. I've lost count of the number of "next generation Bevy UI" prototypes and designs people have put together.

I know the lack of stability has been tough for some people, but I think this is the only way to build The Best Game Engine™ in a collaborative way. Things will start to stabilize soon ... and the wait will be worth it I promise.

### The Bevy App Model

This year we invested heavily in what I call The Bevy App Model. Bevy {{rust_type(type="struct", crate="bevy_app", name="App", no_mod=true, plural=true)}} are easy to understand, ergonomic to write, and modular via {{rust_type(type="trait", crate="bevy_app", name="Plugin", no_mod=true, plural=true)}}. My goal was to blur the lines between engine developers and app developers. I think we absolutely nailed it:

1. There is no "scripting interface" separating "engine logic" from "app logic". We use a single language (Rust) for the whole stack. Rust feels modern with "high level" niceties while retaining low level performance and control. In my opinion, Bevy Apps are often simpler and more expressive than high level equivalents like Unity or Godot, thanks to the state-of-the-art [Bevy ECS](https://github.com/bevyengine/bevy/tree/main/crates/bevy_ecs). And under the hood Bevy Apps _are_ simpler because there are no internal translation layers between languages like C++ and scripting languages like C#:

    ```rust
    use bevy::prelude::*;

    // This is a complete, self-contained, cross platform Bevy App
    fn main() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_startup_system(setup)
            .add_system(greet_players)
            .run();
    }

    struct Player {
        name: String,
    }

    fn setup(mut commands: Commands) {
        commands.spawn().insert(Player {
            name: "Cart".to_string()
        });
    }

    fn greet_players(query: Query<&Player>) {
        for player in query.iter() {
            info!("hello {}!", player.name);
        }
    }
    ```

2. Bevy Engine "internals" are entirely implemented using the same App Model that "app developers" use. "App developers" _are_ "engine developers". "Engine developers" _are_ "app developers".

As a result of (1), (2), and Bevy being free and open source, we foster a feeling of "stack ownership" that the other major players can't. Curious app developers can dig into Bevy's internals and feel immediately at home. The [_thousands of pull requests_](https://github.com/bevyengine/bevy/pulls) are a testament to that. We've seen an [explosion of third party plugins](https://bevyengine.org/assets/#assets) being developed ranging from [realistic physics](https://rapier.rs/) to [specialized tilemap renderers](https://github.com/StarArawn/bevy_ecs_tilemap). Bevy's modular nature enables app developers to mix and match the pieces they like and "build their own engine". Bevy's core plugins like {{rust_type(type="struct", crate="bevy_asset", name="AssetPlugin", no_mod=true)}} and {{rust_type(type="struct", crate="bevy_render", name="RenderPlugin", no_mod=true)}} provide a common ground to ensure {{rust_type(type="trait", crate="bevy_app", name="Plugin", no_mod=true)}} interoperability. This composes nicely with the "modular Render Graph", which makes for an extremely pluggable engine.

### Bevy ECS

**Content Warning**: I'm going to boast a lot here and make some hard-to-verify claims, which might offend some peoples' sensibilities :)

Bevy ECS is the interface we use to build both engine features and apps, so it was natural to give it focus last year. I honestly don't think it is controversial to say that Bevy ECS has pushed the envelope of what an ECS can be. Bevy ECS is the "secret sauce" (well ... \*\*cough\*\* ... "open sauce") that I believe uniquely positions us in the engine market. This is a result of meticulous experimentation, benchmarking, collaboration with other experts in the field, and unification of a lot of good ideas in the wider ECS ecosystem.

* **The _most ergonomic ECS_ in existence**. This might sound like hyperbole, but I haven't found any other offering in any language that can do what we do, as tersely as we do it. We [have](https://bevyengine.org/news/introducing-bevy/#ergonomics) been [constantly](https://github.com/bevyengine/bevy/pull/741) pushing [the](https://github.com/bevyengine/bevy/pull/2398) envelope [here](https://github.com/bevyengine/bevy/pull/1525).
* **Macro-free ergonomics**. In Bevy we have an aversion to macros because they are DSLs (domain specific languages). We want to write _Rust_, not "custom language we invented that happens to live inside Rust". Most ECS-es that approach our level of ergonomics do it via macros. By building Bevy ECS directly on the Rust type system, we ensure Bevy code is "pure rust", the implementation is easier to debug, and users can extend it simply by implementing the relevant traits. This is one of our biggest innovations and I expect to see more projects learn from and adopt this model over time.
* **The first hybrid Archetypal / Sparse Set ECS**: We are the first ECS to support both Archetypal and Sparse Set storage, side-by-side. Prior to this, people selecting an ECS either needed to pick a Sparse Set ECS for faster component add/removes but slower query iteration, or an archetypal ECS for slower add/removes but faster query iteration. With Bevy ECS, developers can select what storage to use _per component_. Via some clever algorithms, we select the best iteration strategy for a given set of components. Credit goes to [@SanderMertens](https://github.com/SanderMertens) (the author of [flecs](https://github.com/SanderMertens/flecs)) for coming up with the initial idea and designs for this. After that, Sander and I spent a lot of time collaborating on implementation details. Bevy was "first to market" here, but flecs has plans to adopt a similar model (and plans to innovate on top of what we've done). This type of mutually beneficial cross project collaboration warms my heart. Why "hold cards close to your chest" and "compete" when you can collaboratively build things together? A rising tide lifts all boats.
* **Lock-free Parallel ECS**: We don't lock storages in Bevy ECS. When a System runs, it has full, unchecked, yet safe access to the data it pulls in. Everything is parallel by default. Everything Just Works™. You can easily define explicit before/after orderings between systems using System Labels, when that matters.
* **Automatic / parallel / global change detection**: All changes are cheaply and automatically tracked in Bevy. _And_ they can be ergonomically queried. This enables an entire class of optimizations that aren't possible in other ECS implementations. To my knowledge, we are the first to do this. All of the alternatives I'm aware of are either imperfect at capturing changes, require manual opt-in boilerplate, or (generally) both. Compare [Unity DOTs change detection](https://medium.com/@5argon/unity-ecs-creating-an-efficient-system-with-chunk-iteration-didaddorchange-didchange-221427f5361b) to [Bevy ECS change detection](https://github.com/bevyengine/bevy/tree/main/crates/bevy_ecs#change-detection) to get a feel for the massive gulf we have crossed here.

For those who think ECS is over-hyped ... I totally get it. ECS is _not_ the only way to build a good engine. Anyone who tells you otherwise either has an agenda or hasn't thought enough about the problem yet. ECS is, however, one of the best ways to standardize data representation and flow in a modular context. This is a problem that _every_ engine needs to solve and the results almost always end up looking _something_ like ECS, even if they don't use that label. ECS is also an extremely broad category, to the point of being almost meaningless. If thinking about Bevy ECS as "Bevy Data Layer" makes you happier, that is a completely valid mindset! Our APIs extend beyond traditional ECS definitions (and [we have plans to go further](https://github.com/bevyengine/bevy/pull/1627)). If you want to use an ECS-based engine because you've bought into the hype, I promise Bevy ECS can deliver :)

### Github Popularity

With over 10,000 stars, we are now the most popular Rust game engine on Github by a pretty wide margin. We are the [8th most popular game engine on Github](https://github.com/topics/game-engine). And Godot (currently holding the #1 spot with 40,900 stars) is starting to feel within our grasp, especially given their six year head start on us. If we make the same amount of progress next year, we'll be in the #2 spot! I'll be the first to say that popularity isn't everything. It isn't a measure of project maturity or feature set, but it _is_ a measure of how many people we reach and resonate with. Winning hearts and minds is an important part of scaling an open source project. I'm proud of this progress and I hope the rest of the community is too.

### Bevy Jobs

We're starting to see paid Bevy jobs pop up and some of them are resulting in open-source contributions back to Bevy. This is the start of the next phase of Bevy's maturity: adoption by professionals. The "stability warning" still stands and studios should take that into account, but these developments excite me.

## There is Always Room for Improvement

![improve](improve.svg)

### Delegating Work and Responsibility

For most of this year, I worked very hard to maintain absolute control over every aspect of the project. This is just how I prefer to do things: I know what I want Bevy to be and I have pretty strong opinions about how that should be accomplished. I reviewed every pull request and design, read (almost) every issue and discord message, wrote every blog post, obsessed over every twitter message. This year was a long and painful process of learning what my limits are, watching the needs of the project grow past them, and then stubbornly refusing to let go in favor of burnout, letting things slip, or both at the same time.

Part of this was me being a control freak to an irrational extent. Part of this was fueled by a more rational desire to hold off delegation until I could build trust in the ethics and technical chops of other Bevy contributors. In retrospect, I waited way too long.

Fortunately, after learning lessons the hard way, I finally started delegating a bit. I created a [Triage Team](https://github.com/orgs/bevyengine/teams/triage-team/members) and opened it up to interested Bevy contributors. I gave [@mockersf](https://github.com/mockersf) the ability to merge small and relatively "uncontroversial" pull requests. [@alice-i-cecile](https://github.com/alice-i-cecile) has been fantastic at wrangling issues and capturing and consolidating information. They're also heading up the new Bevy Book effort. [@Ratysz](https://github.com/Ratysz) built and generally "owns" the new Bevy ECS scheduler. There are plenty of other people shouldering responsibility now ... but I've gotta stop the list somewhere.

I'm still learning how to delegate properly and I'm still not doing it enough. Bevy is growing rapidly and I promise I'll do my best to ensure I'm not the bottleneck going forward.

I don't plan on giving up my "benevolent dictator" status any time soon. Rest assured that I still intend to review (almost) all pull requests and strictly dictate the direction of the engine. But don't be surprised when you start seeing more people drive increasingly large efforts. Scaling out is the only way to allow Bevy to grow at the pace it needs to.

### Project Planning and Communicating Project Direction

Immediately after the initial Bevy release, I [said we needed to focus](https://bevyengine.org/news/scaling-bevy/#focus-focus-focus). I outlined three [focus-areas](https://github.com/bevyengine/bevy/labels/focus-area) for us:

* [**Scenes**](https://github.com/bevyengine/bevy/issues/255): better scene format, inline assets, enabling / disabling systems
* [**PBR / Clustered Forward Rendering**](https://github.com/bevyengine/bevy/issues/179): PBR shaders, HDR, bloom, shadowing, all using clustered-forward rendering
* [**Editor-Ready UI**](https://github.com/bevyengine/bevy/issues/254): iterate on the current Bevy UI, add a canvas style drawing api, implement core widgets, theme-ability

If you have been paying any attention to what we've built over the past year, we clearly _haven't_ focused exclusively on these things, and in some cases chose to go in completely different directions. Here is what we _actually_ focused on (at various points during the year):

* **Bevy ECS and The Bevy App Model**: core rewrite, smarter and more featureful parallel scheduler, states, ergonomics, change detection, performance improvements
* **PBR and GLTF**: Success! We got one ... but only if you pretend "clustered" wasn't in the focus-area title and ignore HDR, bloom, and shadowing!
* **Renderer**: old renderer optimizations and features, new renderer (will be released in the upcoming Bevy 0.6)
* **Platform support** (iOS, Android, Web, as well as Desktop OS improvements)

We did make small iterative improvements to Scenes, but we barely scratched the surface of the topics in the focus-area issue. I no longer think enabling/disabling systems from within scenes is a good idea. There are [experiments covering the other topics](https://github.com/lassade/bevy_prefab), but we clearly didn't "focus". Editor-Ready UI has completely changed course. We plan on building a brand new API instead of building on what we have currently. This has seen a lot of design work and people are constantly experimenting, but again ... not our focus.

There was a clear mismatch in reported vs actual priorities, especially when it comes to how _I_ spent my time. Many of you probably recall "the time we focused on rewriting the ECS", "the time we focused on PBR", or "the time we focused on the new renderer" (aka right now). You won't remember "the time we focused on Editor-Ready UI", because it hasn't happened yet. This created problems for people who did try to break ground on these topics. They were free to experiment on their own, but without me investing time in building or "blessing" designs (because I _insist_ on having the final say on these things), interested and motivated developers ended up "directionless".

I believe we focused on the right things. We had more "foundational" work to do than I accounted for. Over time we all realized what the real focus areas _should_ be, but we never got around to "reporting that out".

Going forward I plan on changing my approach to focus areas in the following ways:

* We will have one or _maaaybe_ two focus areas at a time, according to the number of unknowns and our "bandwidth".
* I will acknowledge that focus areas _need_ immediate investment from me personally, for as long as I choose to occupy the "benevolent dictator" role. If I'm not actively working to unblock people from moving forward, it can't be a focus area (by definition).  
* If for whatever reason our focus needs to change when we discover new information, I will immediately report that out via a "focus area change".

### The Old Renderer's Mid Level APIs

The old renderer (pre Bevy 0.6) did a number of things right:

* User-friendly ECS-driven high level apis for custom shaders with very little boilerplate. For example, check out this ["custom shader material" app](https://github.com/bevyengine/bevy/blob/main/examples/shader/shader_custom_material.rs)
* A modular "low level" Render Graph that enabled developers to easily slot new features into the render pipeline without centralized top-level orchestration, or to create entirely new render pipelines.

However the mid-level apis between the render graph and the high level apis were complicated, full of abstractions and jargon, and in some cases, inefficient. As a result, this made the system hard to reason about and new developers struggled to understand it. I attribute this largely to focusing on making the high level and low level apis "good UX" and treating the mid level apis as "glue".

I consider the old design of the mid-level apis to be a pretty costly "mistake". The new renderer will solve these problems, largely by flattening out abstractions and clarifying dataflow. I'm really excited about it. Live and learn I guess. Things are rarely perfect on your first try.

### Issue and Pull Request Response Times

I'm especially unhappy with how long it takes to merge PRs, especially those that make "big" changes. Part of this is intentionally tabling PRs in subject areas we aren't ready to commit time and brainpower to, but a lot of it is just a bandwidth / attention problem. I've been working with other core contributors to build systems that streamline this process:

* Giving merge rights to a small group of trusted and capable people. Currently this is just [@mockersf](https://github.com/mockersf) and is scoped to "uncontroversial" prs.
* A "community review system" that forces a PR into my "high priority review queue" when there are enough approvals from the community.

I don't think we've seen these systems work at their full capacity yet, as some of them are new and we're currently prioritizing preparing for the Bevy 0.6 release. Thousands of merged PRs a year _is_ something to be proud of, but volume of pull requests is only going to get larger going forward. We need to be able to accommodate that.

### Filling Niches

At this stage of development, Bevy is capable of building most classes of apps (2d, 3d, ui-focused, etc). However we aren't yet "competitive" in any of these niches. For example, for 2D workflows our tilesets are pretty limited, we're missing important optimizations like sprite batching, and we don't have a visual editor for building levels. These things can be built as third party plugins (and in fact, [they have been](https://github.com/StarArawn/bevy_ecs_tilemap)), but Bevy needs to provide first class support out of the box. This past year we "went wide" and built up infrastructure to enable development in each niche, but going forward I want to see us focus on excelling in specific niches. This is largely just a matter of time as we build out features. But we need to be constantly aware of this: if we can't be the best at filling at least one niche, there is no point to any of this.

### I'm Not Drawing Enough Birds

Never thought I'd say those words but here we are. I have a pretty big backlog of "custom Bevy bird avatar" [sponsorship rewards](https://github.com/sponsors/cart) to make for people. Some people have been waiting a _long time_ for theirs. I honestly regret adding this as a reward tier because it draws time away from me actually building Bevy (which I have clearly prioritized over making these avatars). I will likely retire this reward tier in the near future, but I promise I will get through the backlog for those that have already sponsored me at that tier. I am sincerely sorry for the delay.

## The Next Year of Bevy

![next year](next_year.svg)

Here are some of my plans for the next year:

* **Bevy 0.6**: In the very near future we will release Bevy 0.6, which will include a brand new renderer that is drastically more efficient, more capable, easier to understand, and much more extensible. It draws inspiration from the [Bungie / Destiny](https://advances.realtimerendering.com/destiny/gdc_2015/Tatarchuk_GDC_2015__Destiny_Renderer_web.pdf) and [rafx](https://github.com/aclysma/rafx) architectures. This will include ports of all existing renderer features and initial implementations of features like shadows, viewports, material batching, and improved custom shaders.
* **Asset Pipeline Maturity**: The asset pipeline will get asset pre-processing, import configuration, and better dependency management. This will feed into renderer and scene improvements.
* **Next Generation Bevy UI**: We will build a new UI framework that takes advantage of Bevy ECS features, adds new features where necessary (such as "reactivity"), and makes UI development workflows more pleasant.
* **The Bevy Editor**: We will break ground on the Bevy Editor, which will be a Bevy App building on top of the "next generation Bevy UI". We will start with scene editing functionality and then build out from there.
* **Scene Improvements**: Scenes will get support for nested scenes, a nicer scene file format, and property overloading
* **The New Bevy Book**: The current Bevy Book hasn't changed much since its initial release and doesn't cover much more than basic Bevy ECS usage. The new Book is already in progress and will be a much more comprehensive guide to all aspects of Bevy.
* **New Bevy ECS Features**: We will likely get some form of "reactive ECS", entity-relationship indexing, and more granular and featureful parallel system scheduling  
* **Animation**: We will build a unified animation system that makes 2D and 3D animation easier and integrates naturally with the Bevy Editor
* **2D Features**: Sprite batching, more tileset features, layers, visual / interactive tooling in the Bevy Editor
* **3D Features**: Skeletal animation (that integrates with the Animation system), configurable / flexible / good looking shadows, at least one form of global illumination, more PBR properties, and visual / interactive tooling in the Bevy Editor
* **Bevy Game Jam**: We will have at least one official Bevy Game Jam to promote Bevy, battle test apis, and give users more examples to build off of.

I am relatively confident that we can make these things happen. We already have working prototypes for many of the features above and have started reaching consensus in a number of areas.

Here are some predictions about Bevy's trajectory over the next year:

* I expect the Bevy 0.6 release to be immediately followed by an uptick in advanced built-in and third party renderer features (as a result of the new renderer).
* In relatively short order, I expect to see more people reaching for Bevy as their preferred choice for 2D apps, thanks to improved tooling and performance.
* By the end of the year, I expect people to start taking us seriously for 3D applications, thanks to a solid set of built in "advanced rendering features" and our extremely competitive renderer modularity.
* I expect the number of people getting paid to develop Bevy Engine, build Bevy apps, and make Bevy content to go way up.
* If the "Next Generation Bevy UI" effort is successful, people wanting to build "Rust GUI apps" will start reaching for Bevy.
* We will break out of the "Rust gamedev enthusiast" circles. By the end of the year, Bevy will be brought up more regularly in the wider gamedev community alongside conversations about Unity, Unreal, and Godot. Not necessarily as a _direct_ competitor yet, but as a viable alternative for people that (1) want something new / innovative / different and (2) are willing to work around a smaller feature set and slightly less stable apis.

If any of this excites you, we would love your help! Check out our code on [Github](https://github.com/bevyengine/bevy), start participating in the [Bevy Community](https://bevyengine.org/community/), and consider [sponsoring my work](https://github.com/sponsors/cart) to ensure I can continue building and leading this wildly ambitious project.

I'm looking forward to spending the next year with you all!

\- [@cart](https://github.com/cart/)

<img src="/assets/bevy_logo_dark.svg" style="height: 4.0rem; margin-top: 1.5rem" />
