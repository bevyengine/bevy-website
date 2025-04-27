+++
title = "Bevy's Third Birthday"
date = 2023-08-10
authors = ["Carter Anderson"]
[extra]
github = "cart"
youtube = "cartdev"
image = "bevy_birthday.svg"
padded_list_image = true
show_image = true
+++

[@cart](https://www.twitter.com/cart_cart) here (Bevy's creator and Project Lead) with an exciting announcement ... it has now been three years since the initial Bevy release!

As is tradition, I will take this as a chance to reflect on the past year and outline our hopes and dreams for the future. If you're curious, check out Bevy's [First Birthday](/news/bevys-first-birthday) and [Second Birthday](/news/bevys-second-birthday) posts.

This year, I am also highly encouraging everyone to write their own "Bevy Birthday" reflection posts. Just publish your post somewhere (and to social media if you want) and [link to it here](https://github.com/bevyengine/bevy-website/issues/728). One month from now, we will do a followup "Reflecting on Bevy's Third Year" rollup post that aggregates these in one place. This is our chance as a community to celebrate our wins, identify improvement areas, and calibrate our path for the next year.

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. Bevy is also free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. We have a [Quick Start Guide](/learn/quick-start/introduction). You can also check out [Bevy Assets](/assets/) for a library of community-developed plugins, crates, games, and learning resources.

<!-- more -->

## A Year of Milestones

<img src="milestones.svg" alt="milestones" class="img-in-card" />

* **August 19**: [Bevy Jam #2: Combine](https://itch.io/jam/bevy-jam-2)
  * The second official Bevy game jam! 404 people joined, 85 people submitted games, and people left 2,674 ratings. [USA Football League Scouting Combine XLV](https://ramirezmike2.itch.io/usa-football-league-scouting-combine-xlv) won!
* **October 18**: Bevy hits 20,000 stars on GitHub!
* **November 12**: [Bevy 0.9](/news/bevy-0-9/)
  * We added HDR Post Processing, Tonemapping, and Bloom, FXAA, Deband Dithering, Post Processing API Improvements, a New Scene Format, Code Driven Scene Construction, Improved Entity/Component APIs, Exclusive System Rework, Enum Reflection, Time Shader Globals, Plugin Settings, Bevy UI Z-Indices, and more!
* **November 20**: Bevy becomes the [second most popular game engine on GitHub](https://mastodon.social/@cart/109377799039003731)
* **January 14**: James Liu (@james7132) [becomes a Maintainer](/news/scaling-bevy-development/#a-new-maintainer)
* **January 14**: New Bevy Organization Role: [Subject Matter Expert](/news/scaling-bevy-development)
* **January 14**: The [Bevy People](/community/people/) page is launched
* **March 6**: [Bevy 0.10](/news/bevy-0-10/)
  * We added ECS Schedule v3, Cascaded Shadow Maps, Environment Map Lighting, Depth and Normal Prepass, Smooth Skeletal Animation Transitions, Improved Android Support, Revamped Bloom, Distance and Atmospheric Fog, StandardMaterial Blend Modes, More Tonemapping Choices, Color Grading, Parallel Pipelined Rendering, Windows as Entities, Renderer Optimizations, ECS Optimizations, and more!
* **April 1**: [Bevy Jam #3: Side Effects](https://itch.io/jam/bevy-jam-3)
  * The third official Bevy game jam! 353 people joined, 78 people submitted games, and people left 2,158 ratings. [Link Sider](https://kuviman.itch.io/linksider) won!
* **July 9**: [Bevy 0.11](/news/bevy-0-11/)
  * We added Screen Space Ambient Occlusion (SSAO), Temporal Anti-Aliasing (TAA), Morph Targets, Robust Contrast Adaptive Sharpening (RCAS), WebGPU Support, Improved Shader Imports, Parallax Mapping, Schedule-First ECS APIs, Immediate Mode Gizmo Rendering, ECS Audio APIs, UI Borders, Grid UI Layout, UI Performance Improvements, and more!
* **August 10**: Bevy is now three years old!

## A Year By The Numbers

<img src="numbers.svg" alt="numbers" class="img-in-card" />

* **741** unique Bevy contributors on [GitHub](https://github.com/bevyengine) (up from 470)
* **25,222** [GitHub](https://github.com/bevyengine) stars (up from 17,830)
* **2,541** forks on [GitHub](https://github.com/bevyengine) (up from 1,693)
* **5,732** pull requests (3,993 merged) on [GitHub](https://github.com/bevyengine) (up from 3,610 prs and 2,354 merged)
* **3497** issues (2290 closed) on [GitHub](https://github.com/bevyengine) (up from 2,228 and 1,303 closed)
* **5214** commits on [GitHub](https://github.com/bevyengine) (up from 3,629)
* **874**  [GitHub Discussions](https://github.com/bevyengine/bevy/discussions) (up from 470)
* **281** [Bevy Assets](/assets/) (plugins, crates, games, apps, and learning materials) (up from 191)
* **661,020** downloads on [crates.io](https://crates.io/crates/bevy) (up from 206,328)
* **712** [@BevyEngine](https://twitter.com/BevyEngine) retweets of Bevy community content on Twitter (up from 382)
* **14,244** [Bevy Discord](https://discord.com/invite/bevy) members (up from 9,686)
* **3202** community #showcase entries in the [Bevy Discord](https://discord.com/invite/bevy) (up from 1,789)
* **1,424,903** messages in the [Bevy Discord](https://discord.com/invite/bevy) (up from 968,290)

Note that for consistency and clarity all of these numbers are given in "absolute totals", as that is how they are generally reported. For example, we now have 25,222 _total_ GitHub stars ... the number you will see in our repo. I've included the totals as reported last year as well, which can be used to calculate the change in the numbers since last year.

## Things I'm Proud Of

<img src="proud.svg" alt="proud" class="img-in-card" />

I'll try not to repeat myself here, but note that I am still extremely proud of the things I outlined in Bevy's [First Birthday](/news/bevys-first-birthday) and [Second Birthday](/news/bevys-second-birthday) posts.

### Scaling the Organization

This year we rolled out [Subject Matter Experts](/news/scaling-bevy-development) to give autonomy to developers that have proven their technical chops and alignment with Bevy project direction in key subject areas. Historically, only the Project Lead (me) could resolve "controversial" changes (large, foundational, or far reaching changes to the engine). Subject Matter Expert approvals on PRs count as "votes". If two Subject Matter Experts approve a controversial PR, it can be merged without Project Lead approval.

This gave people that have helped make Bevy amazing a seat at the table and allowed us to scale out development. We merged 345 more PRs this year than we did last year!

### Renderer Features

This was an outstanding year for the Bevy Renderer. We added [Screen Space Ambient Occlusion](/news/bevy-0-11/#screen-space-ambient-occlusion), [Cascaded Shadow Maps](/news/bevy-0-10/#cascaded-shadow-maps), [Environment Map Lighting](/news/bevy-0-10/#environment-map-lighting), [Skyboxes](/news/bevy-0-11/#skyboxes), [FXAA](/news/bevy-0-9/#fxaa-fast-approximate-anti-aliasing), [TAA](/news/bevy-0-11/#temporal-anti-aliasing), [Parallax Mapping](/news/bevy-0-11/#parallax-mapping), [Morph Targets](/news/bevy-0-11/#morph-targets), [HDR Tonemapping and Bloom](/news/bevy-0-9/#hdr-post-processing-tonemapping-and-bloom), [Revamped Bloom](/news/bevy-0-10/#revamped-bloom), [Distance and Atmospheric Fog](/news/bevy-0-10/#distance-and-atmospheric-fog), [Smooth Skeletal Animation Transitions](/news/bevy-0-10/#smooth-skeletal-animation-transitions), [More Tonemapping Choices](/news/bevy-0-10/#more-tonemapping-choices), [Robust Contrast Adaptive Sampling](/news/bevy-0-11/#robust-contrast-adaptive-sharpening), [WebGPU Support](/news/bevy-0-11/#webgpu-support), [Enabled Parallel Pipelined Rendering](/news/bevy-0-10/#parallel-pipelined-rendering) , [StandardMaterial Blend Modes](/news/bevy-0-10/#standardmaterial-blend-modes), [Depth and Normal Prepass](/news/bevy-0-10/#depth-and-normal-prepass), [Shadow Mapping Using Prepass Shaders](/news/bevy-0-10/#shadow-mapping-using-prepass-shaders), [Gizmos](/news/bevy-0-11/#gizmos), [Deband Dithering](/news/bevy-0-9/#deband-dithering), [Post Processing APIs](/news/bevy-0-9/#post-processing-view-target-double-buffering), [KTX2 Array/ Cubemap / Cubemap Array Textures](/news/bevy-0-9/#ktx2-array-cubemap-cubemap-array-textures).

And those are just the highlights! If you click on the links above you'll note that there are a _ton_ of different people building out Bevy's renderer. The [`#rendering-dev`](https://discord.gg/bevy) channel on our Discord has really picked up steam.  

### Bevy ECS API Goodness

I am very pleased with the state of Bevy ECS right now. This year the trend was "make more things express-able" and "make expressing things easier":

* [ECS Schedule V3](/news/bevy-0-10/#ecs-schedule-v3) made it possible (and easy) to insert arbitrary sync points (and exclusive systems) into schedules, removing the need for creating new "stages" in these cases.
* [System Sets](/news/bevy-0-10/#configurable-system-sets) made it easier to configure groups of systems.
* You can now add multiple systems with a single [add_systems](/news/bevy-0-10/#adding-systems) call.
* You can configure arbitrary groups of these systems with [nested System tuples and chaining](/news/bevy-0-11/#nested-system-tuples-and-chaining) and [run_if for tuples of systems](/news/bevy-0-11/#run-if-for-tuples-of-systems).
* [Schedule-First Apis](/news/bevy-0-11/#schedule-first-ecs-apis) removed a lot of API bloat and sanded down the remaining rough edges from the scheduler rework.
* Exclusive systems are now [just normal systems](/news/bevy-0-9/#exclusive-system-rework).
* We implemented Bundle for Component which enabled us to [unify our entity APIs](/news/bevy-0-9/#improved-entity-component-apis).
* There were internal quality improvements such as [GATs](/news/bevy-0-9/#bevy-ecs-now-uses-gats) and [UnsafeWorldCell](/news/bevy-0-10/#unsafeworldcell-and-unsafeentitycell), which made the internals easier and safer to work on.

I think that as of **Bevy 0.11**, building apps with Bevy ECS has reached a new peak, especially when it comes to scheduling systems. The schedules you want to express, you generally can. And the APIs are more ergonomic, crisp, and consolidated than they have ever been.

Of course there are still plenty of new ECS features on the horizon. Our ECS team is always cooking up something new!

### Reflection and Scene Format

Bevy Reflect (Bevy's custom Rust reflection library) is also in a solid spot right now. We've filled in the remaining type system gaps such as [Enum reflection](/news/bevy-0-9/#enum-reflection). We improved our [Reflect Proxies](/news/bevy-0-11/#better-reflect-proxies). We defined a new [Stable TypePath](/news/bevy-0-11/#stable-typepath) to give us stability guarantees that Rust's [type_name](https://doc.rust-lang.org/std/any/fn.type_name.html) could not (and more features). We made [Reflect derives imply FromReflect](/news/bevy-0-11/#fromreflect-ergonomics) to improve the ergonomics of defining Reflected types.

These improvements also fed into our scene system. We defined a [New Scene Format](/news/bevy-0-9/#new-scene-format) that is easier to read and compose. We added [Resource support to Scenes](/news/bevy-0-11/#resource-support-in-scenes). And we made it possible to [filter out Components and Resources from Scenes](/news/bevy-0-11/#scene-filtering).

At this point I think the foundations are largely laid. The only things missing at this point are higher level features like "nested scenes" and "property overloading".

### More Real Projects™ Using Bevy

Bevy, despite still being "pre 1.0", continues to be adopted by serious projects. [Tiny Glade](https://store.steampowered.com/app/2198150/Tiny_Glade/) is a [highly anticipated](https://twitter.com/anastasiaopara/status/1626570875842469888) relaxing free-form building game built using Bevy ECS and Bevy App with a custom renderer. [Fish Folk](https://github.com/fishfolk) had a [successfully funded Kickstarter](https://www.kickstarter.com/projects/erlendsh/fish-folk) for their bundle of arcade-style multiplayer games (and associated framework) built on top of Bevy. xyzw is building a [Dwarf-Fortress-like](https://twitter.com/xyzw_io/status/1677340923292786689) with a beautiful custom 2D lighting system. Anselmo is building a [city builder game set during the industrial revolution](https://twitter.com/ElmoSampedro/status/1684920884811698176).

These join the likes of [Foresight](https://www.foresightmining.com/), [Noumenal](https://noumenal.app/), and [Molecoole](https://store.steampowered.com/app/1792170/Molecoole/) from the previous year.

### Our Maintainers

I'm very proud of our Maintainers, who regularly go above and beyond to make Bevy amazing:

* **[Alice Cecile (@alice-i-cecile)](https://github.com/sponsors/alice-i-cecile)** for constantly keeping the PR merge train rolling by reviewing and merging PRs every week. Check out the [`#bevymergetrain`](https://elk.zone/tech.lgbt/tags/bevymergetrain) hashtag on Mastodon for an overview of this prolific work (the hashtag is new but the train has been rolling for a long time). And also for helping make ECS Schedule V3 a reality!
* **[François Mockers (@mockersf)](https://github.com/sponsors/mockersf)** for improving our CI by [orders](/news/bevy-0-11/#new-ci-jobs) of [magnitude](/news/bevy-0-10/#ci-improvements) this year, and for doing the polyglot job of making sure things "actually work" across most areas of the engine (especially the web platform).
* **[Rob Swain (@superdump)](https://github.com/superdump)** for being our rendering ring leader: developing new features, helping other renderer developers get things done, and writing long detailed threads about renderer feature design and rationale.
* **[James Liu (@james7132)](https://github.com/sponsors/james7132)** for [meticulous attention to performance](https://github.com/bevyengine/bevy/issues?q=author%3Ajames7132+label%3AC-Performance+), especially in the ECS area.

I am very fortunate to work with such capable people. Bevy would be a very different (and much worse) project without them. Please show your support by [sponsoring them](/donate/)!

## There Is Always Room For Improvement

<img src="improve.svg" alt="improve" class="img-in-card" />

### I Want to Write More (of my own) Code

My work on Bevy this last year is characterized by three large areas:

1. Designing and building new "big" things like [Bevy Asset V2](https://github.com/bevyengine/bevy/pull/8624), [Schedule-First ECS](https://github.com/bevyengine/bevy/pull/8079), etc
2. Helping others design and build things
3. Running the project: preparing releases, writing blog posts, answering questions, social media, moderating, etc

This year I spent _a lot_ of time on (2) and (3). Like most of it. When someone else makes a big (or important) change, I make sure I grok the change in its entirety, carefully consider the design space, build my ideal design in my head, and then "navigate the diff" between that design and the community member's design (either by making the change myself or asking for changes). I was the co-author of 53 pull requests this year (in addition to my own). I spent countless days hashing out designs with other people.

I find myself logging in every day, popping the queue on (2) and (3), and finding that I have less time for (1) than I would like.

Many times I spend significantly more time "navigating the diff between the right design and a community member's design" than it would take me to design and build the thing myself. And most times, this work (while cool and useful) isn't something at the top of my priority list for Bevy (otherwise I would be building it!).

On the one hand, "navigating the diff" is an important investment in the health of the project. This was a very productive year for the Bevy project _because_ we have so many people building things. And we have more and more changes coming in with a "required change diff" that is tiny or non-existent _because_ seasoned developers have helped onboard people to the project and align them with our view for the future.

But I find myself increasingly dissatisfied with the balance I am striking here. I'm here to build an engine. Because I'm good at it and because it brings me happiness. I am not bad at (2) and (3) (in fact, I think I'm pretty good at them), but they do not bring me joy or fulfillment in the same way.

And it is increasingly clear that on their own they don't drive the project forward in the areas we agree need to be driven forward. I am the Project Lead. It is my _job_ (as in the community pays me) to make sure the project is moving forward in the direction we agree it needs to. I will be striking a new balance that allows myself (and others) to more effectively build out the things that the engine needs. Last year I was too "reactive" to whatever shows up in the pull request / message queue.

This year I will spend more time building what needs to be built and less time "doing everything else". This doesn't mean I will stop reviewing other peoples' code and designs. But I will be much more protective of my "produce outputs in areas that matter the most" time. I will scale out our Maintainer and SME team as necessary to make sure people building cool things outside of "top priority areas" can still make progress.

### Still No Editor

This is disappointing for everyone, including me. It is likely confusing (and annoying) to hear me keep saying _we need the Bevy Editor right now_ and then not deliver it (or even break ground on it). Part of this I attribute to the issues outlined in the previous section. But this is largely due to me failing to accept the truth that it wasn't _really_ my priority. I said "we need the editor now", then spent the year building Bevy Asset V2, joining the ECS scheduling efforts, and reviewing renderer code. A year _feels_ like a long time when I have a whole one in front of me. I see the amazing things we've accomplished so far and believe we can build anything in a year. But when one elapses it feels _so terribly short_.

This will change. The Editor _is actually my top priority now_ and I aim to make tangible strides here as soon as possible. This will start with Bevy UI improvements, but I will be developing Editor prototypes in parallel with those improvements to [dogfood](https://en.wikipedia.org/wiki/Eating_your_own_dog_food) them. There are also plenty of other people chomping at the bit to do editor work. I will do my best to unblock them and spin up "community editor development" as soon as possible. I will also be much more careful going forward about hyping up new features that aren't directly next on our plate.

### Funding Bevy is Confusing

In the early days, I was the only person meaningfully working on Bevy. It made sense to have the Donate button link directly to my [GitHub Sponsors page](https://github.com/sponsors/cart). But things have changed! The [Bevy Org](/community/people/#the-bevy-organization) is now huge and there are plenty of people spending significant amounts of time making Bevy awesome. This year to help account for that, we made a new [Donate Page](/community/donate/) that describes the structure of the Bevy Org and provides a list of people accepting sponsorships. The Donate button now links to this page instead of directly to me.

However this is still suboptimal. How does a company or individual pick who to sponsor? It takes insider knowledge to know where the money would be "best spent". In the end I'm certain most people will opt for name recognition. And when I am often the public face of the project, that likely often means me. Bevy's funding should not be a popularity contest.

I've also noticed that new sponsorships have gone way down since we moved to this system. I no longer have a global view of sponsorships with this system (I can only see my own), but I strongly suspect that prospective sponsors "bounce off" the complexity of the current system. The old system of "click donate and give money to `@cart`", despite its flaws, was at least straightforward.

We need a new system that is:

1. **Simple**: People press the donate button, pick a tier, and setup payment. No additional decision making required.
2. **Logical**: We should distribute funds based on the needs of the project, not popularity or randomness.
3. **Centralized**: The Bevy Organization needs a single pool to draw from when directing money to project members.

This almost certainly means setting up a legal entity for Bevy (ex: a Bevy Foundation 501(c)(3)). I would like to take notes from the [Blender Foundation](https://www.blender.org/about/foundation/) and [Godot Foundation](https://godot.foundation/). I would also like to cut out as many "middle men" crowdfunding platforms / payment processors as I can to make sure donors get the most bang for their buck.

I hear that as of this year 501(c)(3) status is harder to get for software projects, but that doesn't mean we can't try! I would like to have some answer to the funding question by the end of the year.

## Did We Learn From Last Year?

It is important for organizations (and leaders) to learn from their mistakes. Here is my list of "improvement areas" from last year's birthday post, followed by how I think we handled them this year:

* **Bevy 0.6 Took Too long**: Fortunately we learned from this one! No more _9 month_ releases! The train release model we adopted right after **Bevy 0.6** continues to serve us well. **Bevy 0.8** to **0.9** was 3.5 months, **Bevy 0.9** to **0.10** was 3.8 months, and **Bevy 0.10** to **0.11** was 4.2 months. Theres definitely an upward trend that needs some correction though. "3 months plus some buffer" has been getting closer to "4 months plus some buffer".
* **Pipelined Rendering Isn't Actually Pipelined Yet**: We did [enable parallel pipelined rendering](/news/bevy-0-10/#parallel-pipelined-rendering)!
* **We Need The Bevy Editor Now!**: Sadly another year went by without a visual Bevy Editor. I discussed the "how" and "why" of this above.
* **The Rise And Fall of Bevy Merch**: Haha I fortunately did not pick another low-quality merch provider. We still don't have a merch provider though. Sad!

## Can @cart Predict The Future?

In last year's birthday post I [made some predictions for the next year](/news/bevys-second-birthday/#the-next-year-of-bevy). Lets see how I did!

> **The Bevy Editor**: This year, we will start experimenting in the "visual scene editing" space. By the end of the year, I would like to have basic scene editing workflows proved out in a "minimum viable product" fashion.

Sadly another year went by without a visual Bevy Editor. I discussed the "how" and "why" of this above.

> **Bevy Assets**: We will add asset preprocessing (optimizing and processing assets at development time, such as precompiling shaders), which is a critical piece of modern gamedev. This will enable faster asset loading, smaller CPU and GPU memory footprints, per-asset import configuration stored in the filesystem, and smaller deployed app sizes.

[Bevy Asset V2](https://github.com/bevyengine/bevy/pull/8624) didn't _quite_ land in [Bevy 0.11](/news/bevy-0-11/), but it _has_ entered the final review phase and should land in the **Bevy 0.12** (mid October).

> **Bevy UI**: Right now we have a reasonable DOM-like core. This year, we will flesh out what it means to define Bevy UI widgets, improve the ergonomics of defining UI trees, sort out hierarchical UI event handling, and explore higher level dataflow (such as "reactivity"). This will be developed in parallel with the Bevy Editor, which will dogfood Bevy UI and prove out patterns.

While we did improve Bevy UI in a number of areas, we did not make progress on any of the areas mentioned above. Filling in the gaps in Bevy UI is the first step in my "start building the Editor ASAP" plan. I'm starting work on this _tomorrow_.

> **Bevy Scenes**: Scenes will get a prettier format, support scene nesting, and scene property overloading. These features will be developed in parallel with the Bevy Editor to ensure the experience is cohesive and pleasant.

Scenes [did get a prettier format](/news/bevy-0-9/#new-scene-format), but we did not add support for scene nesting or property overloading. I'll call this a partial win!

> **Bevy ECS**: We will merge "stageless" ECS, making it easier / clearer for systems to depend on changes from other systems and resolving some of the biggest remaining UX issues in Bevy ECS.

We did in fact merge "stageless" ECS in the form of [ECS Schedule V3](/news/bevy-0-10/#ecs-schedule-v3). Nice!

> **The New Bevy Book**: We will replace the current Bevy Book with the New Bevy Book. This will be an iterative process. I care more about opening the doors to community Bevy Book development and building that into our developer culture than achieving specific milestones here. We've put this off for too long.

We did not replace the Bevy Book with the New Bevy Book. We _have_ however, finally opened the floodgates to contributions and [developed](https://github.com/bevyengine/bevy-website/issues/623#issuecomment-1518887997) a [drafting process](https://github.com/bevyengine/bevy-website/pull/725). We have [New Book content](https://github.com/bevyengine/bevy-website/pull/624) in the pipeline, which we should hopefully start merging soon. In short: we didn't get nearly as far as I wanted, but the wheels have started turning!

## The Next Year of Bevy

<img src="next_year.svg" alt="next year" class="img-in-card" />

I am no longer outlining explicit "plans for the next year", as I have twice now been pretty bad at making predictions. The Bevy Community and I take a relatively organic and reactive approach to developing Bevy. It doesn't make sense to outline a long list of "plans" when that isn't really how development works in practice.

That being said, here are some of my personal hopes, dreams, and personal priorities for the next year of Bevy.

* **Visual Scene Editor**: With asset system work out of the way, I want to direct my focus toward developing visual scene editing workflows (and polishing up Bevy UI in relevant areas).
* **Nested Bevy Scenes**: Developers need scene files to be compositional to develop games. And they shouldn't need to drop down to code to compose multiple scenes. We should add nesting to scenes (and look to existing 3rd party Bevy plugins like [`bevy_proto`](https://github.com/MrGVSV/bevy_proto) for inspiration).
* **Landing Bevy Asset V2**: [Bevy Asset V2](https://github.com/bevyengine/bevy/pull/8624) has entered the final review phase. We definitely want to land this soon as it will feed into scene-driven workflows and other visual asset editing scenarios.
* **A Legal Bevy Entity**: As mentioned above, we have reached the point where a legal Bevy entity (such as a Bevy Foundation) is necessary for the health of the project.

We have [plenty of other work in the pipeline](https://github.com/bevyengine/bevy/pulls), but I'm keeping this _very_ focused this year to convey my personal priorities.

One last reminder that you should write your own Bevy Birthday blog posts. [Submit them here](https://github.com/bevyengine/bevy-website/issues/728)!

If any of this excites you, we would love your help! Check out our code on [GitHub](https://github.com/bevyengine/bevy), start participating in the [Bevy Community](/community/), and consider [sponsoring our work](/donate) to ensure we can continue building and leading this wildly ambitious project.

I'm looking forward to spending another year building Bevy with you all!

\- [@cart](https://github.com/cart/)

<img src="/assets/bevy_logo_dark.svg" style="height: 4.0rem; margin-top: 1.5rem" class="invertable" />
