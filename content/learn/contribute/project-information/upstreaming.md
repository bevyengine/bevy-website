+++
title = "Upstreaming work"
insert_anchor_links = "right"
[extra]
weight = 6
+++

The Bevy community is full of wonderful projects created to meet the needs of real users.
While a thriving, active third-party ecosystem is essential to Bevy's success,
some things are best suited to be part of Bevy itself.

The process of moving code from the third-party ecosystem (downstream, as it depends on `bevy`)
into Bevy itself is called **upstreaming**.

## Determining what should be upstreamed

Like with any feature we might add to Bevy, the most important things to consider are:

- is this useful?
- is this well-made?
- is this worth the maintenance burden?

While maintainers will ultimately make the final call on merging code,
it can be helpful to reflect on these tradeoffs when making or evaluating proposals to upstream something.

Wen thinking about "should we upstream this crate?" in particular, there are both good and bad arguments to both upstream and not upstream, creating a two-by-two matrix of arguments to consider.
The most obvious quadrant is "good reasons to upstream", which includes:

- this work is widely useful to most Bevy users
- this work is needed by another part of the engine itself (e.g. the Editor)
- users expect game engines to ship with this functionality out of the box
- this functionality would be much easier to implement or maintain if it was in one of Bevy's own crates
- this would be a major improvement over Bevy's existing solution

However, there are also "bad reasons to upstream", which on their own are not compelling:

- this crate is popular (good reasons may exist, but this is not a factor on its own)
- the crate author(s) needs help with maintenance
- someone important made this crate and thinks it's neat
- a commercial user or sponsor of Bevy wants us to upstream it

We can flip this on its head, and think about "good reasons not to upstream".
These center around risks, strategy and ongoing costs:

- this crate is not MIT + Apache licensed, or not compliant with Bevy's policies
- this crate has serious architectural flaws and would make a poor foundation to build on
- this crate does not meet the level of quality needed (see below) to fit into Bevy
- the crate is still rapidly evolving, and would benefit from a more relaxed review or release policy
- the maintenance burden of the crate would be unusually high
  - it may need special testing, unusual hardware, be tracking an evolving standard...
- incorporating this would create a second, competing solution for the same problem inside Bevy itself

Many of these problems can be fixed *before* upstreaming, and doing so is often easier due to the relatively relaxed review policies of third-party crates.

Finally, we can round out our quadrants with "bad reasons not to upstream":

- multiple competing solutions exist and we don't want to pick a winner
  - there are social difficulties here, and care should be taken
  - Bevy's modular design can help alleviate the effects on innovation
  - creating shared unopinionated interfaces can help ease switching costs
  - ultimately, if the functionality is needed, we have to move forward with a choice
- there are more features that would be nice to have
- there are minor bugs
- documentation could be improved

Just like when considering greenfield PRs, projects don't need to be *finished* to be considered for upstreaming:
they just need to be a solid foundation to build on.

## Starting the upstreaming process

The level of deliberation involved in upstreaming something should be proportional to the
deliberation that would be taken to build this functionality from scratch.

Is this a tiny helper crate? Just submit a PR, giving credit!
A huge project with foundational implications?
Please make a [Goal](./project-goals.md) and form a [Working Group](./working-groups.md) to keep track of everything.
Somewhere in between? Talk it over with the relevant experts, then make a PR!

Before you propose upstreaming work, please check in with the author to ask if they're okay with the plan.
Having your crate upstreamed can be validating and relieving,
but it can also be frustrating, as your pet project grows and changes without your say.
While permissive FOSS licenses allow you to upstream work without permission,
this is the hard work of humans we're talking about.
They often have important context on the state of the project,
and it's good to be respectful of their wishes for the things they've built.
Bevy will generally try to respect these wishes, but in the absence of a reply,
may upstream promising permissively licensed work anyways.

While it's common for upstreaming to be the end of a crate's independent lifecycle,
some authors may want to keep their own version going, giving them stability,
creative control or the freedom to experiment,
That's fine too: like with virtually all Bevy functionality, it should be easy to
simply disable Bevy's now-upstreamed feature and replace it with your own equivalent.

## Quality control before upstreaming

While Bevy will always blend pragmatic get-stuff-done and research-oriented idealism,
we have to maintain a certain level of quality to both meet user expectations and avoid collapsing under the weight of a complex project.

Before upstreaming, projects must meet the following basic, objective standards:

- pass `cargo fmt`
- pass `cargo clippy`, using similar settings to Bevy
- be MIT + Apache 2.0 dual-licensed
  - note: Apache-licensed crates can be unilaterally relicensed to also support MIT, but the reverse is not true

More subjectively, they should also:

- minimize the use of `unsafe`, not have any UB, and not present serious security concerns
  - `unsafe` may be unavoidable when interacting with performance-critical or non-Rust code
- have good tests with reasonable coverage (which pass!)
- be judicious in their use of dependencies
  - this is particularly true for other crates which rely on `bevy`!
- have clear and helpful documentation and examples, ideally at the crate, module and API level
- offer the basic functionality expected by users trying to do the thing the crate was made to do
- have a pleasant API that feels Bevy-idiomatic
- present a solid foundation to build on, by making good architectural decisions

In-depth evaluations of these factors are *incredibly* valuable, even from rank-and-file contributors.
Try to be kind but critical while making these assessments:
we need an accurate view of any problems, but there's no need to be cruel.

Once you have completed your evaluation of a candidate crate, be sure to record it somewhere durable:
in a design doc for a working group, as a comment on an issue, or as feedback in a PR review.

Failure to meet these quality bars is rarely a sign that upstreaming is impossible or a poor idea:
instead, it means that work should be done in the candidate crate to improve matters before doing so.
Architectural problems are the most important thing to look out for, as they can mean the difference
between adopting existing work and rewriting it based on lessons learned.

## Project planning for upstreaming

As discussed before, simple work doesn't need a complex process to be upstreamed!
If it's fewer than 1000 lines of code and doesn't need a new crate, you can probably just open a PR.

But some larger projects are quite complex, and careful planning can help things go smoothly.
There are effectively two strategies, and both can be valid:

1. All-at-once
   1. Dump the code upstream in a single PR
   2. Usually in its own crate or well-isolated module
   3. Requires extensive vetting and quality control in the original repo beforehand
   4. If replacing an existing solution, add the new solution first, then deprecate and remove the old one in follow-up PRs, testing as you go
   5. Consider cutting questionable features: they can always be added back later
2. One-bite-at-a-time
   1. Find small, independently useful elements that can be contributed upstream and open modest PRs for them
   2. Change the source repo to target `bevy/main`, and gradually delete code to use the now-upstream version
   3. Continue to add functionality one small feature at a time until all code has been absorbed

The all-at-once strategy works best for monolithic, mature projects (e.g. an entire physics engine),
while the one-bite-at-a-time approach is a good fit for experimental prototypes that should be refined through critical PR review.
