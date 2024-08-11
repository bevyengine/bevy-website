+++
title = "I landed my dream job. Now what?"
date = 2024-08-10
authors = ["Alice I. Cecile"]
[extra]
github = "alice-i-cecile"
image = "TODO.svg"
show_image = true
+++

So, I landed my dream job. Full-time open source, making a game engine, in Rust. Absolutely no meetings. What more could you ask for?
With a very unconventional background (plant ecology, self-taught programming and years of disability), it's flatly astonishing to be [so supported](https://bevyengine.org/donate/): surrounded and empowered by a community making incredible things together.

But now what?
[Bevy's fourth birthday](../2024-08-10-bevys-fourth-birthday/index.md) has rolled around, and @cart has put out a call for others to reflect on the year behind us, and plan for the year ahead.
How have my first few months of Serious Employment at the Bevy Foundation gone, and what am I cooking up?

## Quasi-Hobby to Day Job

For years before I started working at the Bevy Foundation, I've helped out around Bevy: writing docs, triaging issues, reviewing PRs, designing complex systems and polishing off rough corners.

I'd pitch in where I could, tackling the million small tasks in moments between working on side projects, the occasional bits of consulting and making sure my home was running smoothly.
And over the weeks, months, and years that really added up!
Reading, thinking, communicating: I've never been the sort to write code day after day, or get down and dirty to track down obscure bugs.
But if I listened to people talk about what they needed, chewed on it for a bit, and then passed it on to the people who wanted to help, I found I could be *remarkably* effective!

It was a good groove, but all of a sudden: this was my day job!
40 hours a week, 8 hours a day, 9-5 Monday to Friday in the office, right?
Well, okay, sure, no one cares when I work. Or exactly how much I work per day. And we don't even have an office!
But surely, that was the platonic ideal of what Real Work should look like, and I should be aspiring to it, even if it would forever be out of reach.

Actually no, not so much.
At first, I tried to stick to this (entirely of my own volition!): Monday to Friday, 8 hours a day, strict start and stopping time.
I focused on big initiatives, tried to code more, and made *sure* I wasn't working outside of the allowed time blocks.

It made me *miserable*.
Pushing myself to work for long blocks at a time was physically and mentally demanding.
I had to actively force myself *not* to triage that issue, leave that comment, or tackle that PR.
And conversations across our team of dozens of globally-distributed contributors simply didn't have the decency to stay nicely confined to a 9-5 schedule!

So, how did I square the circle? The time and freedom to tackle bigger projects was a huge advantage,
but all those little tasks still mattered!
And even though a more flexible schedule was probably *good* for me,
I needed to make sure that work didn't consume my whole life.

My solution: **focus tasks**.
Every work day (but not days off!), I would pick a single task to focus on.
Writing a piece of documentation, reviewing a complex PR, adding a feature, refactoring a complex bit of code, preparing a design doc, running a Merge Train...
I would focus on that when I was feeling well, get done what I could, and as soon as my focus broke or the task was completed, that was it. I was *done work* for the day.

To complement this, the strict 9-5 M-F schedule would be relaxed. I could work whenever I felt like it (turns out, for me, that's quite a bit):
tackling the million little things that help keep Bevy ticking along.
But there would be no *obligation*: no sense that I *must* do these things, or must do them in a timely fashion.
I could be there for the impromptu late night design chats, but still disconnect during date night because, after all, I finished work *hours* ago.

This balance has worked really well for me: letting me drive forward larger initiatives (`bevy_color`! A huge `leafwing-input-manager` refactor! The 0.14 release!) without falling behind on the flood of notifications.
Would I recommend you use it? Probably not! It requires a remarkable degree of flexibility that many organizations won't afford you, you need to be driven and self-motivation and frankly, the work-life boundary is far blurrier than most people can live with.
But for someone working full-time in open source? Absolutely!

## Dream Jobs are Still Jobs

So, having worked at the Bevy Foundation for a few months: is this really my dream job? How does it live up to my expectations?
Despite my boundless idealism, working for a non-profit you care about is not a panacea: if the working conditions suck,
the fact that you're Doing Good won't get you through the day.

On the bright side, I have:

- a mission I believe in
- a comfortable living (my latest budget says $84k CAD pre-tax)
- the opportunity to meet and learn from incredible people: within Bevy, with Rust and within game dev more broadly
- a work-from-home arrangement, with incredible flexibility
- huge levels of agency over what I work on
- generous European-style vacation and sick-leave policies

But it's not perfect, that's all balanced out by:

- a remarkably public role, where everything I do from day-to-day is visible
- an incredibly small non-volunteer team to serve as a backstop for all of the needful but tedious things that need to be done
- fuzzy work-life boundaries
- a salary is *much* lower than what I could be making
- existential dread caused by relying entirely on [generous donors](https://bevyengine.org/donate) to keep both myself and the project I love afloat

Honestly, it's a lot like founding a startup.
It's just instead of having a small-but-nonzero chance of becoming wealthier than anyone ever needs to be,
I have a chance to change an industry for the better and help people make cool things!

## Learn or Drown

The most striking thing about it though, is the extent to which I have to *keep* learning and growing.
Rust? ECS? Technical writing? Project management? Input management? Community management?

I can't get complacent and only work within my comfort zone of skillsets that I've mastered.
There's always more work to be done, areas that need leadership, and brilliant but complex PRs to review.
Even if my natural proclivities are focused on design and communication, I need to be *fluent* in every single area needed to make games.
Experts are great, but keeping things moving along means I have to be able to understand what they're saying and integrate it into a broader project context.

Over the next year, I hope to get comfortable with basic custom rendering, muck about with Bevy's asset solution and learn about reactive UI in earnest.
Should be fun, and it might even drive some progress on my *own* games.

## Working Groups: Self-Organization and Empowerment

I've long believed that building systems and altering incentives is the best way to fix problems for good.
When I first started working at Bevy, we had a pair of twin problems: contributors were frustrated by the lack of direction and reviews for complex but valuable initiatives, while maintainers were out of bandwidth and frustrated with efforts that petered out as individual contributors got busy.

Enter [working groups](https://github.com/bevyengine/bevy/pull/13162): scoped, collaborative efforts to define, explore and tackle complex problems.
I've been incredibly pleased to see how they've turned out:

- an emphasis on clear, well-motivated designs
- a clear list of things that we're actively tackling
- a space to focus community efforts towards a shared goal
- a ready-made source of reviewers for complex efforts
- a mechanism for ambitious contributors to build consensus and radically change things

The `bevy_color` working group was a wonderful bubbling hub of activity that tackled all of the gnarly corners of writing a color library from scratch,
the 0.14 release working group really helped take the pressure off of Cart and I, even if it was a slog.

I'm really excited to see what the open groups (relations, text, audio, picking, contributing guide, scenes, render graph and curves) put together over the next year!
I'll stir the pot periodically to keep things moving along smoothly, but overall I'm delighted by how well this experiment in self-organization has gone.

## Technical Future: UI that doesn't Suck

Right now, Bevy has a critical technical limitation: our UI solution sucks.
While of course you could argue that *all* UI solutions sucks, `bevy_ui` is remarkably underbaked.
There's too much boilerplate, there are virtually no premade widgets, and most of the hard problems are handed off to the user.

While things *have* improved substantially since Bevy's initial release, `bevy_ui` operates at the same level as HTML: just raw data structures to be manipulated.
And for most of our users, that simply isn't enough!
Bevy's an incredible choice for CAD software, or complex simulation games, or dev tooling (like the fabled editor), *except* that building UIs takes too long and polishing them is nearly impossible.

While I've shared [my own vision for what `bevy_ui` could be](https://hackmd.io/@bevy/HkjcMkJFC), I trust @cart (and @viridia, @StarArawn, @UkoeHB and @SanderMertens) to figure out most of the fancy incremental, reactive, data-driven bits.
Instead, I want to clean up all the low-hanging fruit that drags our UI solution down, no matter what solution they land on.

Over the course of the next year, I want to:

- swap to `cosmic_text`, and properly support non-Latin character sets
- make it easier to reason about fonts and how they're related
- support basic localization and markup-style rich text
- port `bevy_ui` to a polished picking paradigm
- make sure that simple alternatives to flexbox layout can be used for those who prefer it
- add focus management, making it easy to navigate UI with keyboards, gamepads and screen readers
- upstream `leafwing-input-manager`, giving Bevy first-party support for keybindings for both UI and gameplay
- ship a modest collection of functional standard widgets: radio buttons, text input, sliders and more
- write examples that cover real use cases: splash screens, settings menus, drag-and-drop inventories, UI mockups for various game genres...

None of these things will radically change how `bevy_ui` works, but taken together, should lead to a night-and-day difference in the experience for both devs and end users.

## Product Future: Beyond Rust Gamedev

In both Rust-focused and gamedev-focused spaces, Bevy is often *defined* by the fact that it's written in Rust.
But thinking about game engines in this way is a trap, both for Bevy and for the ecosystem as a whole.

Overwhelmingly, professional game developers don't decide that they're going to use Rust, look at the options within the set of Rust game engines and then choose one.
While hobbyists and those seeking to learn Rust *might* choose an engine that way (Rust is delightful and making games is a great way to learn),
it's an ineffective way to make business-critical decisions!

Instead, they look at the set of *full* set of engines and what they have to offer: Unity, Unreal, Godot and dozens more.
Every team and every project has different needs and idosyncratic preferences: there will never be an ur-engine that others are simply *wrong* for not choosing.
But if you want to compete within a crowded space, you need to both carve out and communicate a niche: a set of things that you're *uniquely* good at.

Simply being the best, most popular, or most featureful game engine in Rust isn't enough, and frankly, barely matters at all.
Rust is fantastic, and I think in the long-run it'll be a major advantage, but right now, the game industry perception is that it's immature and slow to develop in.

To survive as anything beyond a hobby engine, you need to attract commercial teams building serious games, built on a budget with mixed teams of programmers, artists and designers.
While you don't need to be better than the big kids in *every* way, you need to be better than them in some ways, and the *rational* choice for some set of teams needs to be to pick your engine.

While we're not the rational choice yet for most projects and teams, I want to set us up for real success in the future.
In the coming year, I want to network more with gamedevs outside of Rust (say hi!), learn more about the workflows that real indie teams use and challenges that they face.
While the small improvements trickle in, one PR at a time, I want to set my eyes on the horizon, understand what talented small teams are looking for out of their next engine, and make sure we're building towards those goals.s

No one is writing the next Call of Duty in Bevy (yet!): the requirements around rendering, tooling, training, console support and risk are way too strict.
But what would it take to convince teams to write the next Factorio, Terraria, Slay the Spire, Hollow Knight or Hades in Bevy?
Time to find out!
