+++
title = "Bevy's Second Birthday"
date = 2022-08-10
[extra]
author = "Carter Anderson"
twitter = "cart_cart"
github = "cart"
youtube = "cartdev"
image = "bevy_birthday.svg"
show_image = true
+++

[@cart](https://www.twitter.com/cart_cart) here (Bevy's creator, lead developer, and project manager) with another exciting announcement:

It has now been two years since the initial Bevy release! As is (now) tradition, I will take this as a chance to reflect on the past year and outline our plans for the future. If you're curious, check out [last year's birthday post](/news/bevys-first-birthday).

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. Bevy is also free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. We have a [Quick Start Guide](/learn/book/getting-started/) and a [Bevy Book](/learn/book/introduction/). You can also check out [Bevy Assets](/assets/) for a library of community-developed plugins, crates, games, and learning resources.

<!-- more -->

## A Year of Milestones

![milestones](milestones.svg)

* **January 1**: Alice Cecile (@alice-i-cecile) gets scoped merge rights for documentation work.
* **January 8**: [Bevy 0.6](/news/bevy-0-6)
  * We added a brand new modern renderer that is prettier, faster, and simpler to extend, directional and point light shadows, clustered forward rendering, frustum culling, significantly faster sprite rendering with less boilerplate, native WebGL2 support, high level custom Materials, more powerful shaders: preprocessors, imports, WGSL support, Bevy ECS ergonomics and performance improvements, no more .system(), and more!
* **January 8**: Train Release Model
  * Bevy adopts a rough "3 month train release model". We no longer hold releases back for specific features. Approximately every 3 months, we kick off the release process.
* **February 24**: [The first Bevy game is released on Steam: Molecoole](https://store.steampowered.com/app/1792170/Molecoole/)
* **February 25**: [Bevy Jam #1](https://itch.io/jam/bevy-jam-1)
  * The first official Bevy Jam! 431 people joined, 74 people submitted games, and people left 1,618 ratings. [Petty Party](https://jabuwu.itch.io/petty-party) won!
* **April 10 (approximately)** Bevy hits 15,000 stars on GitHub!
* **April 15**: [Bevy 0.7](/news/bevy-0-7)
  * We added skeletal animation and mesh skinning, GLTF animation importing, unlimited* point lights in a scene, improved clustered forward rendering,
    compressed texture support (KTX2 / DDS / .basis), compute shader / pipeline specialization,
    render to texture, flexible mesh vertex layouts in shaders, ECS improvements: Order systems using their names, Query::many_mut, ParamSets, WorldQuery derives, documentation improvements: better examples, more doc tests and more coverage, more audio control: pause, volume, speed, and looping, power usage options, and more!
* **April 21**: [New Project Role: Maintainer](/news/bevy-0-8/#bevy-org-changes)
  * The "maintainer" role replaces our old "delegated merge rights" roles, providing more decision making power, removing explicit scope/area restrictions, while still preserving my ability to ensure consistent project direction when that is important.
  * Everyone with "delegated / scoped merge rights" is now a "maintainer" (@alice-i-cecile, @mockersf).
* **April 21**: Rob Swain (@superdump) is made a maintainer.
* **June 10 (approximately)**: Bevy becomes the [3rd most popular game engine on GitHub](https://github.com/topics/game-engine)
* **June 18**: [The first Bevy iOS app is published to the Apple App Store: Noumenal](https://apps.apple.com/us/app/noumenal/id1584884105)
* **July 30**: [Bevy 0.8](/news/bevy-0-8)
  * We added a new Material system / simpler custom shaders / AsBindGroup derives, camera-driven rendering (simple split screen, portals, render to texture, renderer layering, etc), built-in shader modularization (importable built in shader types and functions, custom shaders can now import the PBR shader logic), spotlights, visibility inheritance, automatic mesh tangent generation, renderer optimizations: parallel frustum culling and unstable sorts for unbatched render phases, scene bundle, scripting / modding progress via untyped ECS APIs, ECS query ergonomics and usability, ECS internals refactors/cleanups, reflection improvements: support for reflecting more types, ECS resource reflection, untyped reflection, improved reflection internals, transactional hierarchy commands, [`taffy`](https://github.com/DioxusLabs/taffy) layout, and more!
* **August 10**: Bevy is now two years old!

## A Year By The Numbers

![numbers](numbers.svg)

* **470** unique Bevy contributors on [GitHub](https://github.com/bevyengine) (up from 255)
* **17,830** [GitHub](https://github.com/bevyengine) stars (up from 10,030)
* **1,693** forks on [GitHub](https://github.com/bevyengine) (up from 837)
* **3,610** pull requests (2354 merged) on [GitHub](https://github.com/bevyengine) (up from 1,501 prs and 1060 merged)
* **2,228** issues (1,303 closed) on [GitHub](https://github.com/bevyengine) (up from 1,112)
* **3,629** commits on [GitHub](https://github.com/bevyengine) (up from 1,895)
* **470**  [GitHub Discussions](https://github.com/bevyengine/bevy/discussions) (up from 153)
* **191** [Bevy Assets](/assets/) (plugins, crates, games, apps, and learning materials) (up from 110)
* **206,328** downloads on [crates.io](https://crates.io/crates/bevy) (up from 57,349)
* **382** [@BevyEngine](https://twitter.com/BevyEngine) retweets of Bevy community content on Twitter (up from 93)
* **9,686** [Bevy Discord](https://discord.com/invite/bevy) members (up from 4,871)
* **1,789** community #showcase entries in the [Bevy Discord](https://discord.com/invite/bevy) (up from 771)
* **968,290** messages in the [Bevy Discord](https://discord.com/invite/bevy) (up from 420,250)

Note that for consistency and clarity all of these numbers are given in "absolute totals", as that is how they are generally reported. For example, we now have 17,830 _total_ GitHub stars ... the number you will see in our repo. I've included the totals as reported last year as well, which can be used to calculate the change in the numbers since last year.

## Things I'm Proud Of

![proud](proud.svg)

I'll try not to repeat myself here, but note that I am still extremely proud of the [things I outlined in last year's retrospective](/news/bevys-first-birthday).

### The New Bevy Renderer

Last year we _heavily_ invested in Bevy's renderer. At the start of the year, we [built a new renderer from scratch](/news/bevy-0-6/#the-new-bevy-renderer). From that foundation we continued to iterate with success. Bevy's renderer is now:

1. **Faster**: frustum culling, clustered forward rendering, sprite batching, more "internal" parallelism, lower render costs per entity, parallel pipelining (to be enabled soon ... I'll cover this in a bit)
2. **Easier to Understand**: clearer layers of abstraction and dataflow (low: raw wgpu, mid: Render Graph, Extract/Prepare/Queue, PipelineCache, ShaderType, high: Material, Meshes)
3. **Easier to Extend**: new high level Material system for simple custom shaders, camera driven rendering / per camera render graphs, flexible mesh vertex layouts, shader imports, modularized / extensible shader logic (such as our PBR logic), better Rust->GPU dataflow via ShaderType
4. **More Featureful**: More lighting options (point / directional / spot / spherical), shadows, shader imports and preprocessing, compute shaders, skeletal animation, camera driven rendering (render to texture, split screen, portals, etc), unlimited* point lights, compressed gpu textures, visibility inheritance, mesh tangent generation, better pipeline specialization, etc

We've invested a lot in making the renderer modular and extensible. That plus a clearer / more principled architecture has resulted in a serious uptick in renderer feature development, both upstream and in the wider ecosystem. Community projects like [bevy_hikari](https://github.com/cryscan/bevy-hikari), [Chris Biscardi's Bevy shader videos on YouTube](https://www.youtube.com/c/chrisbiscardi), and the increasingly pretty content in the [`#showcase` channel on Bevy's Discord](https://discord.gg/bevy) make it feel like we're on the right track here. People are more willing and able to write Bevy renderer code than they were with the previous renderer (including our core contributors).

There is still plenty of work to do, but I think we've largely laid the foundations at this point. Now we can now focus on features, performance, and improving the user experience.

### Delegation and Project Scaling

Last year I struggled a lot with [scaling the project](/news/bevys-first-birthday/#delegating-work-and-responsibility) to account for a rapidly growing community, managing burnout, maintaining control, etc.

I'm happy to say that while we haven't _solved_ this problem, we're in a much better spot than we were before! We've defined a new ["maintainer" role](/news/bevy-0-8/#bevy-org-changes) and filled it with capable people (@mockersf, @alice-i-cecile, @superdump). Maintainers have more freedom to merge code and make decisions, but we've also defined a "controversial change" process that allows me to dictate project direction when that is necessary (ex: ensuring consistent vision, resolving disputes, quality control, etc).

[Alice Cecile](https://github.com/sponsors/alice-i-cecile) (@alice-i-cecile) has been working full time and has done a great job wrangling issues, merging uncontroversial PRs expediently, picking up areas like documentation, working on the ECS, and directing contributor energy in the right direction. This eased my burden significantly and enabled me to focus my energy in areas that need it the most (making core architectural decisions and building out core features, reviewing controversial / technical changes, managing the project, etc). They recently [gave a fantastic talk about project management at Rust Conf](https://rustconf.com/schedule#your-open-source-repo-needs-a-project-manager)!

[François Mockers](https://github.com/sponsors/mockersf) (@mockersf) has proven to be a technically proficient jack of all trades that has focused on making our CI top-tier, fixing critical issues before anyone else is even aware of them, and picking up high value work that is sometimes (regrettably) lower visibility. On top of that, they have demonstrated skill and interest in pretty much every pillar of the engine.

Rob Swain (@superdump) (our newest maintainer) has designed and implemented a huge portion of the pretty renderer features (lighting, shadows, clustered forward rendering, etc) and has clearly established themselves as a technical leader in that area. Bevy's renderer would not be what it is today without their dedication and expertise.

### A Thriving Ecosystem

Being a young engine, Bevy still has a number of functionality gaps. But thanks to our focus on modularity, many of these gaps have already been filled by the community!

This is great because Bevy app developers are unblocked in these areas. The community gets to experiment with and iterate on ideas. And we (as Bevy engine developers) get to observe these spaces, learn what patterns work, and ultimately either adopt them or iterate on them when the time comes.

In particular, the following Bevy plugins are particularly popular and useful:

* **Physics**: [bevy_rapier](https://github.com/dimforge/bevy_rapier) (the official [Rapier](https://rapier.rs/) plugin maintained by the Rapier developers) is by far the best option if you need physics in your Bevy App. We will likely upstream some form of Rapier integration this year.
* **UI**: Bevy UI is still in the early stages of development. It is useful for basic game UIs (like health bars / huds / etc), but it isn't quite ready for more complicated UI-heavy scenarios. In these cases, [bevy_egui](https://github.com/mvlabat/bevy_egui) provides a fully featured immediate-mode GUI solution that plays very nicely with Bevy ECS.
* **Audio**: Bevy's audio system is also still bare bones. For more flexible audio control, [bevy_kira_audio](https://github.com/NiklasEi/bevy_kira_audio) has your back.
* **Editor**: Bevy doesn't yet have an official visual editor. [bevy_editor_pls](https://github.com/jakobhellermann/bevy_editor_pls) is a 3rd party Bevy Plugin that adds editor functionality to your game. Still a work in progress, but very useful!
* **Networking**: Game networking is a pretty opinionated space, so it is unsurprising that there are a lot of options here! [bevy_ggrs](https://crates.io/crates/bevy_ggrs), [bevy_renet](https://crates.io/crates/bevy_renet), [bevy_backroll](https://crates.io/crates/bevy_backroll), and [naia](https://github.com/naia-lib/naia/tree/main/demos/bevy) are some of the most popular choices.

There are _tons_ of other plugins listed on [Bevy Assets](/assets/#assets).

Thanks to our first [Bevy Jam](https://itch.io/jam/bevy-jam-1), games listed on [Bevy Assets](/assets/#games), and the many [Bevy games listed on itch.io](https://itch.io/games/tag-bevy), we now have hundreds of examples of Bevy games, many of which are open source.

While we wait for the new Bevy Book, [Bevy Cheatbook](https://bevy-cheatbook.github.io/) continues to be a top tier community developed learning resource.

We've also seen an uptick in educational Bevy content on YouTube. [Tantan](https://www.youtube.com/c/Tantandev), [Logic Projects](https://www.youtube.com/channel/UC7v3YEDa603x_84PgCPytzA), [Chris Biscardi](https://www.youtube.com/c/chrisbiscardi), [PhaestusFox](https://www.youtube.com/c/PhaestusFox), [Jeremy Chone](https://www.youtube.com/c/JeremyChone), [Griffin](https://www.youtube.com/channel/UCwTl4_-_pAaloW07Q2o-tSw), and others have been creating some really interesting stuff.

As I recently [ranted on Twitter](https://twitter.com/cart_cart/status/1547699794092498945), developer ecosystems are what really drive the success of an engine. And in my opinion the Bevy developer community is the _primary_ driver of our success.

### People and Companies Are Using Bevy for Real Projects™

[Foresight](https://www.foresightmining.com/), [Noumenal](https://noumenal.app/), [Molecoole](https://store.steampowered.com/app/1792170/Molecoole/), and [Punchy / Fish Folk](https://github.com/fishfolks/punchy) are some notable examples of projects that involve people that are getting paid for their work using Bevy. We've had 16 Bevy-related job postings on [our Discord](https://discord.gg/bevy)!

I find it interesting that some of the most notable Bevy projects are tools / non-games (Foresight and Noumenal). It seems like the modularity of the engine makes us a compelling choice for tooling that requires a renderer and a principled app model.

Of course, my "Bevy is still young and we make breaking changes regularly" warning still applies. Before picking us for a serious project: do your research, experiment with Bevy, and get a feel for what Bevy development looks like _today_. But I think given the progression we're seeing here, I might need to consider changing or removing that warning sooner than I thought. More and more projects are choosing Bevy and reporting positive experiences.

## There Is Always Room For Improvement

![improve](improve.svg)

### Bevy 0.6 Took Too long

I chose to hold back **Bevy 0.6** until the new renderer was in a good spot, which in retrospect was a mistake. No sense in preventing people from using features that are ready just because another unrelated feature isn't ready! This was generally bad for morale and a large gap in public Bevy announcements was also suboptimal. In response to that drawn out release, we adopted a "train release schedule", which protected us from large, drawn out releases blocking on specific features. [Bevy 0.7](/news/bevy-0-7) and [Bevy 0.8](/news/bevy-0-8) used the new model and it resulted in faster, lower stress, and more predictable releases. We're still refining the process, but so far from what I can tell both Bevy developers and Bevy users are liking the new system.

### Pipelined Rendering Isn't Actually Pipelined Yet

For **Bevy 0.6** we went through a lot of trouble to design and implement a renderer that supports parallel pipelining. The entire design philosophy revolves around that, such as the "extract" pattern for reading data from the "main app" to the "render app". However in the interest of getting **Bevy 0.6** out the door ASAP, we made the call to hold off on enabling pipelining. There were some unanswered questions about how to handle threading in a way that accommodates edge cases like "non send" resources. I've had a [branch that enables parallel pipelining](https://github.com/cart/bevy/tree/actual-pipelining) since before **Bevy 0.6** dropped.

I continually deprioritized the last-mile effort required to enable pipelining in favor of building out other renderer features and unblocking other engine efforts. In some cases, I think this was the right call. User experience is generally way more important than eking out more performance. But letting these gains sit unrealized for about 8 months is regrettable, especially given how close we are and how much we've already invested. I will be prioritizing this last mile work.

### We Need The Bevy Editor Now!

Modern gamedev workflows for most game types require visual editors. Being able to visually and interactively move entities around in a scene is critical if you are building visual, interactive experiences. Bevy has a scene system, but without visual tooling it is hard to get any value out of it. Because of this, Bevy Engine developers don't have serious incentives or feedback loops to expand and improve the scene system, and Bevy users don't have serious incentives to adopt it.

I've been deliberately holding back Bevy Editor development since Bevy first released so that we can build out Bevy's core systems in a focused way. I've been saying we need Bevy UI to be "editor ready" (aka ensuring it has the required features, widgets, patterns, and UX sorted out) before we start building out editor experiences. I no longer believe that. I think we should take a more iterative approach. Unlock the value now. Learn from experience. Refine. Iterate. Repeat. As Bevy UI evolves, we will need to rewrite aspects of the editor. Yes this is retreading ground, but I now believe it is Worth It.

### The Rise And Fall of Bevy Merch

Many of you may have noticed that [we announced the Bevy Merch store](https://twitter.com/BevyEngine/status/1498026079738421248) in February. There is a lot of demand for Bevy apparel / stickers / etc and we wanted to provide that option. Many people bought Bevy Merch and for awhile, life was good.

Sadly as the months progressed, [I noticed that the print on my Bevy sweater started peeling after 3 washes](https://twitter.com/cart_cart/status/1525984877773877248). One other Bevy community member reached out and reported the same thing. Rather than sitting around and waiting for more issues to pop up, I opted to [close Bevy Merch](https://twitter.com/BevyEngine/status/1525983991391612930) in May and offered everyone refunds out of my own pocket.

If anyone is looking for an apparel / printing service, please steer clear of Spring (our old merch provider). Their print quality and longevity is _poor_.

We will almost certainly find another merch provider and re-open the store, but before opening our doors to the public I will do thorough quality testing over a long period of time. The Bevy community put their trust in me and I put my trust in Spring (which they did not earn and promptly lost). When your money is involved you all deserve more thoroughness than I gave you. I am sincerely sorry. I won't make the same mistake again.

## Did We Learn From Last Year?

It is important for organizations to learn from their mistakes. Now that we have a year in our "birthday buffer", its worth looking back to see how we did. Here is my list of "improvement areas" from [last year's birthday post](/news/bevys-first-birthday/#there-is-always-room-for-improvement), followed by how I think we handled them this year:

* **Delegating Work and Responsibility / Issue and Pull Response Times**: As mentioned in the "Things I'm Proud Of" section, we made significant strides this year. But this is a battle we will be fighting for as long as Bevy continues to grow.
* **Project Planning and Communicating Project Direction**: This year we retired the old "focus areas" model in favor of a more organic approach. We have heavily embraced [domain-specific project boards](https://github.com/orgs/bevyengine/projects?type=new), which helps people follow along with what we are working on. This is an improvement over the old _perpetually and actively wrong_ focus areas. And the direct ties to our GitHub issues and pull requests anchors these things in reality. That being said, I think there is still a gap when it comes to communicating our big / high level priorities in an easy-to-digest manner.
* **The Old Renderer's Mid Level APIs**: We scrapped the old renderer entirely! I won't repeat what I said in the "Things I'm Proud Of" section, but I consider this largely resolved. On the topic of "understandability", I think our biggest gap at this point is documentation and learning paths.
* **Filling Niches**: Last year, I wanted us to focus on making Bevy a competitive experience for specific classes of apps. For our first year we went "wide" to cover as many domains as possible (2d, 3d, ui, etc) but within a given domain, there were gaps everywhere. We've definitely improved a lot here. 3D now has key features like skeletal animation, which was blocking pretty much every serious 3D game from being produced. Thanks to the new renderer and sprite batching our core 2D performance is in a much better spot and sprite-heavy games are now possible. That being said, in spirit, I don't think we've checked this box. When it comes to features, Bevy still isn't remotely competitive with the other big players in the space. That is certainly not a gap we can close in a year of time, but this year we _really_ need to start focusing more on asset workflows, scenes, and editor experiences.
* **I'm Not Drawing Enough Birds**: For the first year of Bevy development, I offered a "custom Bevy bird avatar" reward tier for my sponsors. I was neglectfully slow when it came to drawing those birds. I'm happy to say that I successfully cleared my "custom Bevy bird avatar" backlog. Then I promptly removed that reward tier. It was fun, but my time is better spent building and managing Bevy.

## Can @cart Predict The Future?

For our last birthday, I [made some predictions for the next year](/news/bevys-first-birthday/#the-next-year-of-bevy). Lets see how well I did :)

* **Bevy 0.6 will be released in the near future**: We did indeed [release Bevy 0.6](/news/bevy-0-6), but as mentioned above, it took another 4 months. That is a stretch of the phrase "near future".
* **Asset Pipeline Maturity**: The asset pipeline did not get asset pre-processing, import configuration, or better dependency management. This area _is_ my primary focus for the next release cycle, but it was deprioritized last year. However we have done a lot of [design work and experimentation](https://github.com/bevyengine/bevy/discussions/3972) in preparation for this.
* **Next Generation Bevy UI**: We did not meaningfully rework Bevy UI to improve the user experience or capabilities (other than swapping out the [`stretch`](https://github.com/vislyhq/stretch) layout lib for [`taffy`](https://github.com/DioxusLabs/taffy), which fixed the _extremely poor_ nested layout performance). Since **Bevy 0.8** dropped, we have started ramping up our efforts here and consolidating our plans to make Bevy UI excellent. But third party UI plugins like [bevy_egui](https://github.com/mvlabat/bevy_egui) currently still provide a more pleasant and featureful experience.
* **Break Ground on The Bevy Editor**: We did not break ground on the Bevy Editor. As mentioned above, we _need_ to start prioritizing this work.
* **Predicted Scene Improvements**: We did not get support for nested scenes, make the scene format prettier, or add property overloading. We have a new PR for a [prettier scene format](https://github.com/bevyengine/bevy/pull/4561) that builds on all of the Bevy Reflect work we did last year, but we are still reviewing it.
* **The New Bevy Book**: We did make progress on the new Bevy Book, but we have not released it yet.
* **Predicted Bevy ECS Features**: We invested heavily in Bevy ECS, but we did not get "reactive ECS" / entity-relationship indexing. The parallel system scheduling api _was_ improved, but we did not make it more granular (ex: "stageless ECS" is nearly ready, but we haven't merged it yet).
* **Unified Property Animation System**: We did do some design work ([RFC 49](https://github.com/bevyengine/rfcs/pull/49), [RFC 51](https://github.com/bevyengine/rfcs/pull/51)) and there were some proposed implementations ([PR 1429](https://github.com/bevyengine/bevy/pull/1429)), but we have not merged an implementation yet.
* **Predicted 2D Features**: We did implement sprite batching and sprite render layers! We didn't meaningfully add tileset workflow improvements (3rd party crates like [bevy_ecs_tilemap](https://github.com/StarArawn/bevy_ecs_tilemap) are often still preferable). And we also didn't add visual editor workflows (because the Bevy Editor doesn't exist yet).
* **Predicted 3D Features**: We did implement skeletal animation, configurable / flexible / good looking shadows, and added a few more PBR Material configuration options! We did not add a global illumination implementation (although 3rd party plugins like [bevy-hikari](https://github.com/cryscan/bevy-hikari) now exist). We also didn't add visual editor workflows (again, because the Bevy Editor doesn't exist yet).
* **First Bevy Game Jam**: We did indeed [have our first Bevy Jam](https://itch.io/jam/bevy-jam-1)!

As you can see, I largely failed to predict the future. Many of my predictions were in the vein of "higher level asset and editor driven workflows", but we ended up focusing on other pillars like the Bevy Renderer, Bevy ECS, Bevy Reflect, and platform support (mainly Web, some iOS and Android).

I think we made the right calls here: we had a lot of foundational work to do, especially on the rendering front. Now that we've largely fleshed out Bevy's core (and code driven workflows), we can spend the next year realizing our plans for higher level editor driven workflows. Note that while our focus is shifting in this direction, we still plan to make code-first workflows first class and pleasant!

## The Next Year of Bevy

![next year](next_year.svg)

Here are some of our "big ticket" items planned for the next year. I'm keeping this list smaller and more scoped than last year because some of these items are _huge_ efforts and I was clearly overconfident in my predictions last year. That being said, I think we now have a better understanding of where we are in our "tech tree", we've started scaling out the Bevy Org and delegating effectively, and we are more capable of prioritizing these things (instead of needing to direct efforts to other foundational pieces). I believe we will make all of these things happen.

* **The Bevy Editor**: This year, we will start experimenting in the "visual scene editing" space. By the end of the year, I would like to have basic scene editing workflows proved out in a "minimum viable product" fashion.
* **Bevy Assets**: We will add asset preprocessing (optimizing and processing assets at development time, such as precompiling shaders), which is a critical piece of modern gamedev. This will enable faster asset loading, smaller CPU and GPU memory footprints, per-asset import configuration stored in the filesystem, and smaller deployed app sizes.
* **Bevy UI**: Right now we have a reasonable DOM-like core. This year, we will flesh out what it means to define Bevy UI widgets, improve the ergonomics of defining UI trees, sort out hierarchical UI event handling, and explore higher level dataflow (such as "reactivity"). This will be developed in parallel with the Bevy Editor, which will dogfood Bevy UI and prove out patterns.
* **Bevy Scenes**: Scenes will get a prettier format, support scene nesting, and scene property overloading. These features will be developed in parallel with the Bevy Editor to ensure the experience is cohesive and pleasant.
* **Bevy ECS**: We will merge "stageless" ECS, making it easier / clearer for systems to depend on changes from other systems and resolving some of the biggest remaining UX issues in Bevy ECS.
* **The New Bevy Book**: We will replace the current Bevy Book with the New Bevy Book. This will be an iterative process. I care more about opening the doors to community Bevy Book development and building that into our developer culture than achieving specific milestones here. We've put this off for too long.

We have [plenty of other work in the pipeline](https://github.com/bevyengine/bevy/pulls), but I'm keeping this scoped to highlight the areas I believe we should be focusing on (and where I will personally be directing my own efforts as the project lead).

[Bevy Jam #2](https://itch.io/jam/bevy-jam-2) is also coming up on August 19th. Game jams are a great way to dive in to game development, even if you have never built a game (or used Bevy)! They are also a great way to meet like minded people. I _highly_ recommend participating. I personally had a blast during the [last one](https://itch.io/jam/bevy-jam-1)!

If any of this excites you, we would love your help! Check out our code on [GitHub](https://github.com/bevyengine/bevy), start participating in the [Bevy Community](/community/), and consider [sponsoring our work](/news/bevy-0-8/#support-bevy) to ensure we can continue building and leading this wildly ambitious project.

This year is going to be a big one for Bevy. I'm looking forward to spending it with you all!

\- [@cart](https://github.com/cart/)

<img src="/assets/bevy_logo_dark.svg" style="height: 4.0rem; margin-top: 1.5rem" />
