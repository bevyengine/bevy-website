+++
title = "Bevy's Fourth Birthday"
date = 2024-08-10
authors = ["Carter Anderson"]
[extra]
twitter = "cart_cart"
github = "cart"
youtube = "cartdev"
image = "bevy_birthday.svg"
show_image = true
+++

Hey! [@cart](https://www.twitter.com/cart_cart) here (Bevy's creator and Project Lead). Another year of Bevy development has finally passed, meaning that Bevy is officially _four years old_ now!

As is tradition, I will take this as a chance to reflect on the past year and outline our hopes and dreams for the future. If you're curious, check out Bevy's [First Birthday](/news/bevys-first-birthday), [Second Birthday](/news/bevys-second-birthday), and [Third Birthday](/news/bevys-third-birthday/) posts.

I also highly encourage Bevy developers and community members to write their own "Bevy Birthday" reflection posts. Just publish your post somewhere (and to social media if you want) and [link to it here](https://github.com/bevyengine/bevy-website/issues/1592). One month from now, we will do a "Reflecting on Bevy's Fourth Year" rollup post that aggregates these in one place. This is our chance as a community to celebrate our wins, identify improvement areas, and calibrate our path for the next year.

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. Bevy is also free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. We have a [Quick Start Guide](/learn/quick-start/introduction). You can also check out [Bevy Assets](/assets/) for a library of community-developed plugins, crates, games, and learning resources.

<!-- more -->

## A Year of Milestones

![milestones](milestones.svg)

* **November 4**: [Bevy 0.12](/news/bevy-0-12/)
  * We added Deferred Rendering, Bevy Asset V2, PCF Shadow Filtering, StandardMaterial Light Transmission, Material Extensions, Rusty Shader Imports, Suspend and Resume on Android, Automatic Batching and Instancing of Draw Commands, Renderer Optimizations, One Shot Systems, UI Materials, and more!
* **December 2**: [Bevy Jam #4: That's a LOT of Entities!](https://itch.io/jam/bevy-jam-4)
  * The fourth official Bevy game jam! 368 people joined, 87 people submitted games, and people left 1,639 ratings. [That's a LOT of beeeeees](https://andrewb330.itch.io/thats-a-lot-of-beeeeees) won!
* **January 31**: Bevy hits 1,000,000 downloads on crates.io!
* **February 10**: Bevy hits 30,000 stars on GitHub!
* **February 17**: [Bevy 0.13](/news/bevy-0-13/)
  * We added Lightmaps, Irradiance Volumes / Voxel Global Illumination, Approximate Indirect Specular Occlusion, Reflection Probes, Primitive Shapes, System Stepping, Dynamic Queries, Automatically Inferred Command Flush Points, Slicing / Tiling / Nine-patch 2D images, Camera-Driven UI, Camera Exposure, Animation Interpolation Modes, and more!
* **March 11**: [The Bevy Foundation](/news/bevy-foundation/)
  * We announced The Bevy Foundation, the next step in our journey to build a world class free and open source game engine. Bevy Foundation is a non-profit organization dedicated to developing Bevy and teaching people how to use it!
* **May 1**: [Alice starts a Paid Full Time Role at The Bevy Foundation](https://mastodon.social/deck/@alice_i_cecile@mastodon.gamedev.place/112368168674098114)
  * Thanks to The Bevy Foundation's [generous donors](/donate/), Alice was able to start full time work at The Bevy Foundation as a Staff Engineer.
* **July 4**: [Bevy 0.14](/news/bevy-0-14/)
  * We added Virtual Geometry, Sharp Screen Space Reflections, Depth of Field, Per-Object Motion Blur, Volumetric Fog / Lighting, Filmic Color Grading, PBR Anisotropy, Auto Exposure, PCF for Point Lights, Animation Blending, ECS Hooks and Observers, Better Colors, Computed States and Sub-States, Rounded UI Corners, and more!
* **July 20th**: [Bevy Jam #5: Cycles](https://itch.io/jam/bevy-jam-5)
  * The fifth official Bevy game jam! 224 people joined, 79 people submitted games, and people left 1,677 ratings (so far). The voting period hasn't ended yet, so [go play and rate some games](https://itch.io/jam/bevy-jam-5/entries)!
* **August 10**: Bevy is now four years old!

## A Year By The Numbers

![numbers](numbers.svg)

* **1027** unique Bevy contributors on [GitHub](https://github.com/bevyengine) (up from 741)
* **34,537** [GitHub](https://github.com/bevyengine) stars (up from 25,222)
* **3,379** forks on [GitHub](https://github.com/bevyengine) (up from 2,541)
* **9,221** pull requests (6,788 merged) on [GitHub](https://github.com/bevyengine) (up from 5,732 prs and 3,993 merged)
* **5,624** issues (3,816 closed) on [GitHub](https://github.com/bevyengine) (up from 3,497 and 2,290 closed)
* **7,770** commits on [GitHub](https://github.com/bevyengine) (up from 5,214)
* **1,327**  [GitHub Discussions](https://github.com/bevyengine/bevy/discussions) (up from 874)
* **382** [Bevy Assets](/assets/) (plugins, crates, games, apps, and learning materials) (up from 281)
* **1,495,435** downloads on [crates.io](https://crates.io/crates/bevy) (up from 661,020)
* **19,087** [Bevy Discord](https://discord.com/invite/bevy) members (up from 14,244)
* **4,447** community #showcase entries in the [Bevy Discord](https://discord.com/invite/bevy) (up from 3,202)
* **2,083,652** messages in the [Bevy Discord](https://discord.com/invite/bevy) (up from 1,424,903)

Note that for consistency and clarity all of these numbers are given in "absolute totals", as that is how they are generally reported. For example, we now have 34,537 _total_ GitHub stars ... the number you will see in our repo. I've included the totals as reported last year as well, which can be used to calculate the change in the numbers since last year.

## Things I'm Proud Of

![proud](proud.svg)

I'll try not to repeat myself here, but note that I am still extremely proud of the things I outlined in Bevy's [First Birthday](/news/bevys-first-birthday), [Second Birthday](/news/bevys-second-birthday), and [Third Birthday](/news/bevys-third-birthday) posts.

### The Bevy Foundation

Getting this spun up occupied a solid chunk of my time this year. I had to learn the intricacies of setting up corporations, researching and selecting the best legal structure, achieving state-level non-profit status, achieving federal-level 501c3 non-profit status (the hard part here was putting together the application, which is _still_ pending five months later), hiring people in Canada as a US company, setting up corporate bank accounts, defining bylaws, conflict of interest policies, articles of incorporation, state and federal tax compliance, maximizing transparency by publicizing structural documents and meeting minutes, I [built a custom crowdfunding platform](/donate/) to cut out overhead ... the list goes on and on. This was exhausting work, but it was also _definitely_ a worthwhile investment.

With The Bevy Foundation in place, Bevy is positioned to live a long and stable life, independent of any one person. It exists to be a central, trusted place to fund and lead the Bevy project. It puts its mission (and specifically the needs of the community) above all else. It is lead by its Board of Directors. By convention there is a 1:1 mapping between Director and Bevy Maintainer ... we want the people actually in the trenches to be in charge of the foundation. The board structure also means that I no longer have absolute authority over Bevy. I _am_ currently the President of the foundation (which by convention is also Bevy's Project Lead). The combination of these two roles gives me a lot of the same authority I've had in the past. But it is now my job to enact the will of the board. And if they no longer think I should hold that position, they can remove me.

This also solves our biggest outstanding funding problem: "popularity contest" ad-hoc individual crowdfunding. By directing all funding to the centralized Bevy Foundation, money no longer goes directly to the people with the largest following or the most direct connections to the project. Instead, The Bevy Foundation can choose where it goes based on the needs of the project.

### Alice is Keeping the Train Rolling

This year, thanks to our [generous donors](/donate/), The Bevy Foundation was able to bring Alice on as a full-time Staff Engineer.

For years Alice has spent countless hours as a maintainer, performing a significant (dominating!) portion of Bevy's project manager duties, in addition to individual technical contributions, documentation work, and more. She has taken this to the next level in her new full-time role at The Bevy Foundation, and the benefits are clearly tangible (and quantifiable). Pull requests stagnate far less often. Our issue backlog is tightly organized. 41% of the _total pull requests merged over Bevy's lifetime_ happened _over the past year_. Our total number of unique contributors is up by _39%_ over the past year! Bevy's development pace grew rapidly this year, and Alice is the one that widened the bottleneck to make that happen. The Bevy community, the maintainers, and I have all felt the constraints of this bottleneck over the years. And while "scaling Bevy development" will be an ever-present constantly-evolving problem to solve, I think I speak for most of us when I say "thank you Alice for making this SOOOO much better than it was".

This has also allowed me to take a big step back from these duties. I still perform them, but far less often. This in turn allowed me to dedicate time to things like [spinning up The Bevy Foundation](/news/bevy-foundation/) and [designing and building Bevy's new Scene / UI system](https://github.com/bevyengine/bevy/discussions/14437) (a large improvement to the Bevy developer experience and a cornerstone for the upcoming Bevy Editor).

Although project management has been a focus, I'd like to stress that Alice's role is multi-faceted. She has big plans for technical work, foundation programs, documentation, and more. But I'll let her speak for herself about those!

### Renderer Maturity

This year I am _once again_ surprised by how rapidly Bevy's renderer is maturing. I thought last year was a landmark (potentially anomalous) year for the renderer, but Bevy's fourth year is arguably even bigger. This is _just_ the list of the "big" highlight features:

* [Deferred Rendering](/news/bevy-0-12/#deferred-rendering)
* [Virtual Geometry](/news/bevy-0-14/#virtual-geometry-experimental)
* [Irradiance Volumes / Voxel Global Illumination](/news/bevy-0-13/#irradiance-volumes-voxel-global-illumination)
* [Approximate Indirect Specular Occlusion](/news/bevy-0-13/#approximate-indirect-specular-occlusion)
* [Lightmaps](/news/bevy-0-13/#lightmaps)
* [PCF Shadow Filtering](/news/bevy-0-12/#pcf-shadow-filtering)
* [Automatic Batching and Instancing of Draw Commands](/news/bevy-0-12/#automatic-batching-and-instancing-of-draw-commands)
* [Sharp Screen Space Reflections](/news/bevy-0-14/#sharp-screen-space-reflections)
* [Reflection Probes](/news/bevy-0-13/#minimal-reflection-probes)
* [Volumetric Fog / Lighting](/news/bevy-0-14/#volumetric-fog-and-volumetric-lighting-light-shafts-god-rays)
* [Depth of Field](/news/bevy-0-14/#fast-depth-of-field)
* [Light Transmission](/news/bevy-0-12/#standardmaterial-light-transmission)
* [Per-Object Motion Blur](/news/bevy-0-14/#per-object-motion-blur)
* [Material Extensions](/news/bevy-0-12/#material-extensions)
* [Rusty Shader Imports](/news/bevy-0-12/#rusty-shader-imports)
* [Camera Exposure](/news/bevy-0-13/#camera-exposure)
* [Filmic Color Grading](/news/bevy-0-14/#filmic-color-grading)
* [PBR Anisotropy](/news/bevy-0-14/#pbr-anisotropy)
* [Auto Exposure](/news/bevy-0-14/#auto-exposure)

The Bevy renderer developer community has really heated up this year. In addition to many of the "usual suspects" from previous years (`@superdump`, `@JMS55`, `@robtfm`, `@DGriffin91`, `@aevyrie`, `@icesentry`, `@coreh`, and others), you'll probably notice that _many_ of these features were built by `@pcwalton`, who is new to the Bevy project this year (but is _definitely_ [not new to the Rust ecosystem](http://pcwalton.github.io/)). He has been absolutely prolific this year and the Bevy community is lucky to have him on board.

As a general assessment, Bevy now has many of the renderer features people expect from a modern game engine, in addition to exciting cutting edge features like [Virtual Geometry](https://jms55.github.io/posts/2024-06-09-virtual-geometry-bevy-0-14/) (built by `@JMS55`), which is something pretty much no one but Unreal has an offering for (you maybe have heard of Nanite before!).

We've made huge strides in renderer performance this year thanks to many targeted optimizations, moving more work to the GPU, and restructuring renderer dataflow. We're [midway through a transition to GPU driven rendering](/news/bevy-0-12/#the-road-to-gpu-driven-rendering) that will take this even further. We still have plenty to do in this space, but we have a lot of irons in the fire at this point and plenty of motivated and talented renderer developers.

Combine that with one of the most modular game engine renderers out there right now, and I think Bevy is well positioned to attract some serious attention in the coming year from both renderer nerds and those interested in making use of top-tier graphics tech.

If you are interested in diving in, check out our vibrant [`#render-dev` Discord channel](https://discord.com/invite/bevy).

### UI Ecosystem Development

Bevy UI is one of the most exciting spaces in Bevy engine development right now. We have a _ton_ of projects in this space:

* [My Next Generation Bevy Scene / UI work](https://github.com/bevyengine/bevy/discussions/14437): This is the kickoff document for the Next Generation Scene / UI working group, an outline of my current work, and a general vision for our path forward.
* [Alice's Vision For UI](https://hackmd.io/@bevy/HkjcMkJFC): This is in part a response to my document. It aims to define the space more broadly in the form of questions and answers. It also ascribes confidence to each answer, which makes it an excellent place for people to see what is solid and what still needs discussion. This document encodes Alice's thoughts, but I pretty much fully agree with them.
* [Quill](https://github.com/viridia/quill): `@viridia`'s coarse-grained (React-style) reactive UI framework for Bevy. They have built some [pretty impressive](https://discord.com/channels/691052431525675048/692648638823923732/1260026892199661671) demos with it. Quill V2 just dropped and it is better than ever.
* [Bevy Reactor / Obsidian](https://github.com/viridia/bevy_reactor): Bevy Reactor is `@viridia`'s fine-grained (Solid.js-style) reactivity framework for Bevy, and Obsidian is a UI framework that builds on top of it. `@viridia` has been spending a _ton_ of time experimenting in the "reactive" space. I consider them to be a trusted expert at this point and their work will definitely help inform upstream Bevy UI's direction.
* [Woodpecker UI](https://github.com/StarArawn/woodpecker_ui): Woodpecker UI is an exciting successor to `@StarArawn`'s previous project [`kayak_ui`](https://github.com/StarArawn/kayak_ui). It is a reactive UI framework that features [`vello`](https://github.com/linebender/bevy_vello) rendering, ECS-first design, and some built in widgets to work with.
* [Sickle UI](https://github.com/UmbraLuminosa/sickle_ui): An impressively complete widget library built on top of Bevy UI.
* [Bevy egui](https://github.com/mvlabat/bevy_egui): The tried and true immediate-mode UI that has been filling in Bevy's UI ecosystem gaps for years now. Still a top tier choice, especially if immediate-mode is what you are looking for.
* [Bevy Cobweb UI](https://github.com/UkoeHB/bevy_cobweb_ui): A layer on top of [`sickle_ui`](https://github.com/UmbraLuminosa/sickle_ui) that uses the [`bevy_cobweb`](https://github.com/UkoeHB/bevy_cobweb) reactivity framework.
* [haalka](https://github.com/databasedav/haalka): A functional-reactive-programming UI library for Bevy powered by [`futures-signals`](https://github.com/Pauan/rust-signals)
* [Bevy Lunex](https://github.com/bytestring-net/bevy_lunex): A retained UI framework for Bevy with a new layout engine and support for world-space UI.  
* [belly](https://github.com/jkb0o/belly): A layer on top of Bevy UI that provides a new macro syntax, style sheets, and data binding features.
* [cuicui](https://github.com/nicopap/cuicui): A reactive UI framework for Bevy with a simple / intuitive layout algorithm, assembled from an impressive collection of modular pieces.
* [bevy_dioxus](https://github.com/JMS55/bevy_dioxus): Bevy integration for the Rust-based [Dioxus](https://dioxuslabs.com/) UI framework.

As you can probably tell, the Bevy community has gone _very_ wide and covered significant ground. Bevy users now have plenty of solid options for 3rd party Bevy UI frameworks, each taking a different opinionated path.

However it is definitely time to start reeling this in. Bevy needs a high quality official UI framework that the community can rally behind.

### Bevy's Upcoming Unified Scene / UI System

The newly-established [Next Generation Scene / UI working group](https://discord.com/channels/691052431525675048/1264881140007702558) will distill the lessons learned above and ideally get as many members of the Bevy UI developer community on board for the next generation of official Bevy UI (and Scenes ... it is important to note that we're building a combined Scene / UI framework here ... a unified Bevy data model).

As stated in my design document, a lot of the work I have done is built in layers. My goal is to establish consensus on each layer and start merging them quickly. As we go "up" the stack, each layer will become more controversial. Eventually we will reach a point in the stack (ex: reactivity and styles) that will require significant discussion and collaboration amongst the many developers above. That being said, I think there are very few unknowns at this point. We've explored the space and we've written plenty of code. Now we just need to steer the ship.

The new Scene / UI system will be a MASSIVE upgrade to Bevy's capabilities. Developing Bevy apps (games, UI apps, tools, editors, etc) will be easier and more fun than ever before. I apologize for being grandiose / wearing my marketing hat, but I really think this will position Bevy to be both a top-tier general-purpose app development framework _and_ a next-generation game development framework (and yes ... game _ENGINE_). 

Reactive scenes will be _big_ and we're [one](https://github.com/SanderMertens/flecs) of the first to do them! We are also positioned to be the first popular engine that allows you to define scenes in code _and_ in asset files (and ultimately visually in the Bevy Editor) using _the same asset format_, thanks to the `bsn!` macro. This will empower developers to use the paradigm that suits them best _and_ to _blend_ paradigms when that suits them.

I am _very_ excited for what this all means for the Bevy developer experience, both as Bevy's Project Lead _and_ as a user of Bevy.

### Even More Real Projects™ Using Bevy

This was a great year for Bevy "in the real world":

* [Tiny Glade](https://store.steampowered.com/app/2198150/Tiny_Glade/), a game that uses Bevy ECS and Bevy App (alongside a custom renderer) has _over a million wishlists on Steam_. They just wrapped up a very well received public demo and by all accounts are looking to have an excellent release.
* [Tunnet](https://store.steampowered.com/app/2286390/Tunnet/) is a 3D "computer network building game" that released on [Steam](https://store.steampowered.com/app/2286390/Tunnet/) and [itch.io](https://puzzled-squid.itch.io/tunnet) and received great reviews.
* [DEATHTRIP](https://store.steampowered.com/app/2909010/DEATHTRIP/) is an in-development "daring, stylish, high-octane FPS catered to the speedfreak".
* [Astortion](https://store.steampowered.com/app/1993980/Astortion/) is an in-development "minimalistic, atmospheric puzzle-platformer that revolves around gravity".
* [Jarl Game](https://x.com/jarl_game) is an in-development colony building game with beautiful real time 2D lighting.
* [Times of Progress](https://store.steampowered.com/app/2628450/Times_of_Progress/) is an in-development city builder set during the industrial revolution.
* [Foresight Spatial Labs](https://www.fslabs.ca/) specializes in building data rendering tools with physically based rendering, high performance simulation, and spatial streaming. They continue to be our biggest sponsor and they regularly contribute upstream!
* [Storyteller.ai](https://storyteller.ai/) is an AI music and film creation tool. It was used live in concert by David Guetta and on stream by PewDiePie and xQc. It uses Bevy in the browser and on the server for its core scene generation capabilities.
* airbus [used Bevy](https://www.youtube.com/live/XLefuzE-ABU?si=cc2MUF67QRDRNc_2&t=4534) to "help modernize on-board software" for their satellites
* [Gunbug](https://store.steampowered.com/app/2946990/Gunbug/) is an in-development "centipede shoot 'em up".
* [Roids](https://store.steampowered.com/app/2248000/Roids/) is an in-development space game where you explore a procedurally generated galaxy
* [El Mono](https://store.steampowered.com/app/3060580/El_Mono/) is an "ape rampage" game that recently released on [Steam](https://store.steampowered.com/app/3060580/El_Mono/).
* [GLOW](https://store.steampowered.com/app/2896110/GLOW/) is "an addictive physics-based arcade game that will challenge you to your limits" that released on [Steam](https://store.steampowered.com/app/2896110/GLOW/)
* [Zoolitaire](https://apps.apple.com/us/app/zoolitaire/id6479218498) is a released mobile puzzle game
* [Elevated Arcana](https://elevatedarcana.com/) is an in-development mobile tower defense game.
* [Ethertia](https://www.youtube.com/watch?v=lFIfp8o0CiI) is a minecraft-like sandbox voxel game
* [Greenfeet Haven](https://store.steampowered.com/app/2791310/Greenfeet_Haven/): is an in-development "colony sim set in a fantasy world where you start from nothing into the wild".

Lots of people using Bevy in production! And those are just the ones I'm personally aware of!

### Bevy ECS Maturity

Unlike previous years, which each saw large and sweeping changes to highly trafficked Bevy ECS APIs ([Schedule v3](/news/bevy-0-10/#ecs-schedule-v3), [Schedule-First APIs](/news/bevy-0-11/#schedule-first-ecs-apis), [Bevy ECS V2](/news/bevy-0-5/#bevy-ecs-v2), etc), this year our core ECS APIs largely remained stable. We were able to layer big new features on top like [Hooks and Observers](/news/bevy-0-14/#ecs-hooks-and-observers), [Dynamic Queries](/news/bevy-0-13/#dynamic-queries), and [One Shot Systems](/news/bevy-0-12/#one-shot-systems) without people needing to rework their code.

I expect this trend to continue going forward. We've spent _years_ iterating on the core Bevy ECS APIs and I think we've finally landed on something that can stand the test of time. Bevy ECS is ergonomic, delightful to use, and fast!

This is also a pattern I expect to see in other areas of the engine: initial rapid and aggressive iteration to find the ideal designs, followed by stability and iterative improvements to solid foundations.

## There Is Always Room For Improvement

![improve](improve.svg)

### Still No Editor

Yes, another year has passed without the Bevy Editor. We _have_ made significant strides in that direction:

* As mentioned above, we're nearing an MVP of the Next Generation Scene / UI system. This is a cornerstone of the Bevy Editor, both because we will use it to build the UI of the editor _and_ because the Bevy Editor's scene editor will use the new scene system.
* We are in the process of [implementing the Bevy Remote Protocol](https://github.com/bevyengine/bevy/pull/13563), which will enable the Bevy Editor to communicate and interact with Bevy Apps as they run (even on different machines / platforms).
* We landed Bevy Asset V2, which will allow the Bevy Editor to drive asset configuration and pre-processing.
* We have kicked off the [Bevy Editor Prototypes effort](https://github.com/bevyengine/bevy_editor_prototypes/discussions/1), a low-management-overhead / low-design-constraints place for Bevy contributors to work together on editor experiments. This unblocks interested developers from building out editor experiences while we wrap up the foundational pieces. Things have been a bit quiet there recently, but outside of that umbrella there are already a number of Bevy editor prototypes out there: [bevy-web-editor](https://makeshift-bevy-web-editor.vercel.app/), [space_editor](https://github.com/rewin123/space_editor), `@viridia` is [building editor things on top of their UI work](https://discord.com/channels/691052431525675048/692648638823923732/1238326822119149620), and there is also the excellent [BLENVY](https://github.com/kaosat-dev/Blenvy), which turns Blender into a Bevy scene editor!

So yes, the Bevy Editor is happening. We're all working toward it as fast as we can. I wanted us to have something usable by our fourth birthday, but sadly we missed that mark.

Fortunately, unlike previous years, we're finally actually on the path and many of the hard problems have already been solved. There is still a lot of work to do, but I'm starting to see the light at the end of the tunnel.

We could have (and in retrospect probably should have) taken more shortcuts to get there faster. In the world of developer tooling, having _something_, even if it isn't perfect, is better than having _nothing_. But for better or for worse we're nearing the end of the longer, more measured and intentional path. That means that we'll have much stronger foundations to build on from the start.

### Bevy UI Stagnated For Too Long

We're finally starting to reach consensus and make serious progress on the next steps for Bevy UI (see the more positive UI Ecosystem Development section above). That being said, by letting it drag on this long I've also caused a lot of fractured effort. I believe a lack of a strong center of gravity made people feel like if they wanted to see progress, they had to go out and build something on their own. This _did_ yield fruit, as evidenced by the proliferation of 3rd party Bevy UI frameworks. But I think much of that effort could have and should have been invested in a single upstream project. This was a clear lack of leadership on my part. I knew this vacuum existed, but I chose to focus on my personal Scene / UI experiments, The Bevy Foundation, Bevy Asset V2, etc. My [first Scene / UI proposal](https://github.com/bevyengine/bevy/discussions/9538) at the start of the year kicked off a lot of good discussion, but then everyone (including me) went off to do their own things. Well, we've done that ... for too long, and now it is _definitely time to reel things in_. With the [Next Generation Scene / UI working group kickoff](https://github.com/bevyengine/bevy/discussions/14437), I now have the time and the focus to make sure I don't make the same mistake again. I'm going to push for consensus and tangible _upstream_ progress ASAP.

### We Have Unfilled SME Roles

Bevy uses a [Subject Matter Expert](https://github.com/bevyengine/bevy/blob/main/docs/the_bevy_organization.md#subject-matter-expert-sme) (SME) system to empower developers that have proven themselves within a given development area to shape the future of their area in Bevy. This has generally proven to be a great way to scale out our development capacity. We've seen it work for ECS, Rendering, and to an extent Animation and Input.

If you pull up the [Bevy People](https://bevyengine.org/community/people/) page, you will notice that there are no SMEs listed for Audio, UI, or Assets. Each of which is a major pillar of Bevy (or any game engine really). This means that the Maintainers and the Project Lead (me) are back to being the bottleneck. This is inefficient, and it is also an indicator that knowledge in these areas is too consolidated across too few people.

The following are all true:

1. There are people in the community that are capable of filling these roles
2. By not filling these roles, we are bottlenecking progress in the given areas
3. We have _intentionally_ not filled these roles yet

What gives?

There is a common thread across all of these areas: they are all "in flux" right now. Another way to interpret "in flux" in this case is "there is an investment, leadership, or vision vacuum". This makes it hard to identify SMEs, as an SME must demonstrate investment, leadership, and vision _before_ we appoint them.

1. **Audio**: Bevy's audio implementation has historically been bare-minimum / just barely enough to cover common scenarios. It has improved over the years, but it is still minimal. We've prioritized other things above it, and nobody has tried to take long term ownership of it until recently. [`bevy_kira_audio`](https://github.com/NiklasEi/bevy_kira_audio) has filled the gap well enough that upstream pressure to improve was further relieved. Fortunately there is now a [Better Audio working group](https://discord.com/channels/691052431525675048/1236113088793677888), so I'm hoping we'll find some SME candidates this way after designs are proposed and implemented.
2. **UI**: There are a _ton_ of people who have demonstrated investment and vision in 3rd party Bevy UI crates. However because none of these efforts have been blessed, by definition none of these have "aligned vision" with upstream Bevy. Therefore the SME vacuum here is _my fault_ because I've chosen to withhold blessing someone else's vision wholesale in favor of developing my own vision first through experimentation and research.
3. **Assets**: This is an area that I've largely claimed for myself over the past couple of years. I haven't done nearly enough work to pull other people in and help make them experts in Bevy Asset V2. And I've since moved on to Scene / UI / Editor work, leaving a big gaping hole in the asset space.

The dominating pattern is Bevy leadership (often me) isn't spending the required time to unblock a space. This has generally been intentional: we've prioritized our many other responsibilities above doing this (or in the case of UI, we've been spending a _ton_ of time trying to unblock the space).

That being said I'm pretty certain we could have and should have unblocked these areas sooner. Fortunately I think we'll be positioned to appoint SMEs in these areas in the near future.

## Did We Learn From Last Year?

It is important for organizations (and leaders) to learn from their mistakes. Here is my list of "improvement areas" from last year's birthday post, followed by how I think we handled them this year:

* **I Want to Write More (of my own) Code**: Despite The Bevy Foundation taking up a lot of my time, I did manage to write a reasonable chunk of code. Most of this fell into either my [Next Generation Scene / UI](https://github.com/bevyengine/bevy/discussions/14437) work or [Bevy Asset V2](/news/bevy-0-12/#bevy-asset-v2) and related followup work. I did also spend _a lot_ of time adopting / iterating on / tweaking other peoples' work, as is my tendency. I'm reasonably satisfied with the balance I struck this year, but I do still think I need to be spending more time writing code. Hopefully, now that The Bevy Foundation is ready to go and Alice is full-time, this next year will be even more productive on this front.
* **Still No Editor**: As covered above, the Bevy Editor _still_ isn't here. We've made significant progress, but this is still by far the biggest hole in the Bevy developer story, and the biggest continued failure of both myself and the organization at large (and as Project Lead I definitely deserve the brunt of the blame here).
* **Funding Bevy is Confusing**: This is fully resolved! The Bevy Foundation is now the simple, unified, unambiguous way to fund Bevy development. If you like what we're doing, [please donate to The Bevy Foundation](/donate)!

## Can @cart Predict The Future?

In last year's birthday post I [made some predictions for the next year](/news/bevys-third-birthday/#the-next-year-of-bevy). Lets see how I did!

> **Visual Scene Editor**: With asset system work out of the way, I want to direct my focus toward developing visual scene editing workflows (and polishing up Bevy UI in relevant areas).

Once again, I over promised and under-delivered on this. See the above section for my full thoughts.

> **Nested Bevy Scenes**: Developers need scene files to be compositional to develop games. And they shouldn't need to drop down to code to compose multiple scenes. We should add nesting to scenes (and look to existing 3rd party Bevy plugins like bevy_proto for inspiration).

We did not _quite_ land nested Bevy scenes, but this is _almost_ ready / has been designed and implemented (in an MVP form) as part of my [Next Generation Scene / UI](https://github.com/bevyengine/bevy/discussions/14437) work. We should have this in the hands of Bevy developers in the near future.

> **Landing Bevy Asset V2**: Bevy Asset V2 has entered the final review phase. We definitely want to land this soon as it will feed into scene-driven workflows and other visual asset editing scenarios.

We did [land Bevy Asset V2](/news/bevy-0-12/#bevy-asset-v2)!

> **A Legal Bevy Entity**: As mentioned above, we have reached the point where a legal Bevy entity (such as a Bevy Foundation) is necessary for the health of the project.

We did [set up The Bevy Foundation](/news/bevy-foundation/)!

## The Next Year of Bevy

![next year](next_year.svg)

The Bevy Community and I take a relatively organic and reactive approach to developing Bevy. It doesn't make sense to outline a long list of "plans" when that isn't really how development works in practice.

That being said, here are some of my personal hopes, dreams, and personal priorities for the next year of Bevy:

* **Next Generation Scene / UI**: I've already discussed this at length. See [this discussion](https://github.com/bevyengine/bevy/discussions/14437) for more information.
* **The Bevy Editor**: A visual scene editor (among other things) that builds on the Next Generation Scene / UI work. Lets hope this is the last time I say this in one of these posts!
* **Relations**: Optimized "links" between ECS entities (for example, a `Parent` relationship). This will fill one of the largest / clearest remaining gaps in the Bevy ECS feature set.

We have [plenty of other work in the pipeline](https://github.com/bevyengine/bevy/pulls), but I'm once again choosing to keep this _very_ focused this year to convey my personal priorities.

One last reminder that Bevy community members should write their own Bevy Birthday blog posts. [Submit them here](https://github.com/bevyengine/bevy-website/issues/1592)!

If any of this excites you, we would love your help! Check out our code on [GitHub](https://github.com/bevyengine/bevy) and start participating in the [Bevy Community](/community/).

Also _please_ consider [donating to The Bevy Foundation](/donate) to ensure we can continue building and leading this wildly ambitious project. The more funds the have, the more we can scale Bevy development!

To many more years of Bevy!

\- [@cart](https://github.com/cart/)

<img src="/assets/bevy_logo_dark.svg" style="height: 4.0rem; margin-top: 1.5rem" />
