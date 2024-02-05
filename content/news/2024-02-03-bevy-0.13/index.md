+++
title = "Bevy 0.13"
date = 2024-02-03
[extra]
author = "Bevy Contributors"
image = "TODO.gif"
show_image = true
image_subtitle = "TODO"
image_subtitle_link = "TODO"

+++

Thanks to **TODO** contributors, **TODO** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), we're happy to announce the **Bevy 0.13** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/book/getting-started/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.
And to see what the engine has to offer hands-on, check out the entries in the [latest Bevy Jam](https://itch.io/jam/bevy-jam-4/entries), including the winner [That's a lot of beeeeees](https://andrewb330.itch.io/thats-a-lot-of-beeeeees)

To update an existing Bevy App or Plugin to **Bevy 0.13**, check out our [0.12 to 0.13 Migration Guide](/learn/migration-guides/0.12-0.13/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

* **First-party primitive shapes:** basic shapes are a core building block of both game engines and video games: we've added a polished collection of them for you to use!
* **Dynamic queries:** refining queries from within systems is extremely expressive, and is the last big puzzle piece for runtime-defined types and third-party modding and scripting integration.
* **Automatically inferred command flush points:** tired of reasoning about where to put `apply_deferred` and confused about why your commands weren't being applied? Us too! Now, Bevy's scheduler uses ordinary `.before` and `.after` constraints and inspects the system parameters to automatically infer (and deduplicate) synchronization points.
* **Slicing, tiling and ninepatch sprites and UI:** ninepatch layout is a popular tool for smoothly scaling stylized tilesets and UIs. Now in Bevy!
* **Lightmaps:** the first step towards baked global illumination: a fast, popular and pretty lighting technique.
* **Animation interpolation modes:** Bevy now supports non-linear interpolation modes in exported glTF animations.

## Primitive shapes

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Dynamic queries

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Entity optimizations

<div class="release-feature-authors">authors: @Bluefinger, @notverymoe, @scottmcm, @bushrat011899, @james7132</div>

`Entity` received a number of changes this cycle, combining laying some more groundwork for relations alongside _related_, and nice to have, performance optimizations. The work here involved a lot of deep-diving into compiler codegen/assembly output, with running lots of benchmarks and testing in order to ensure all changes didn't cause breakages or major problems. Although the work here was dealing with mostly _safe_ code, there were lots of underlying assumptions being changed that could have impacted code elsewhere. This was the most "micro-optimization" oriented set of changes in Bevy 0.13.

### Changing `Entity`'s Layout

This is a story told in two parts, with part one dealing with two PRs: The "Unified Identifier for entities & relations" PR ([#9797](https://github.com/bevyengine/bevy/pull/9797) by @Bluefinger) and "Change Entity::generation from u32 to NonZeroU32 for niche optimization" ([#9907](https://github.com/bevyengine/bevy/pull/9907) by @notverymoe). Fundamentally, they both involved changing the layout and assumptions of `Entity`, unlocking both a needed building block for the Relations feature being worked towards, as well as gaining a needed memory/codegen optimization for `Entity`.

What [#9797](https://github.com/bevyengine/bevy/pull/9797) changed was create a new `Identifier` struct that set out a unified layout for `Entity` and future ID types. The `Identifier` spec lays out a struct with a `u32` low segment and a `u32` high segment, resulting in a struct that can be effectively represented as a special-cased `u64` value. A diagram of `Identifier` layout is as follows, going from Most Significant Bit to Lowest Significant Bit:

```text
|F| High value / Generation       | Low value / Index              |
|-|-------------------------------|--------------------------------|
|1| 31 bits                       | 32 bits                        |

F = Bit Flags
```

```rust
#[derive(Clone, Copy)]
struct Identifier {
    low: u32,
    high: u32,
}
```

The low part can be used as a full `u32` value, but the high part consists of a packed `u32` structure, with the most significant bit reserved as a flag, and the remaining 31-bits to be used as a value. _How_ the low segment and the value part of the high segment are used is not specified by the `Identifer`, only that there are reserved bits in the high segment. This bit is reserved for the purposes of a future `Pair` identifier, which would represent a different _kind_ of identifier with separate semantics/usage compared to an `Entity`. The work here is basically an implementation of the prior art established by Flecs ([citation](https://ajmmertens.medium.com/doing-a-lot-with-a-little-ecs-identifiers-25a72bd2647)), as eventually, these new entities and entity kinds will be used to describe special component types needed for relations. In the future, more flag bits will be reserved at the cost of less available value bits in the high segment.

But on top of this came [#9907](https://github.com/bevyengine/bevy/pull/9907). This PR had the effect of changing both `Entity`/`Identifier` to have a high segment that was `NonZeroU32`:

```rust
#[derive(Clone, Copy)]
struct Identifier {
    low: u32,
    high: NonZeroU32,
}
```

By including a non-zero property within `Entity`/`Identifier`, it allowed the compiler to be able to optimise `Option<Entity>` and so forth to take 8 bytes instead of 12 bytes, as it could now use invalid representations of `Entity`/`Identifier` as the `None` equivalent. The benefit here was reduced memory/cache usage for storing `Option<Entity>` as well as better codegen around methods that use or return `Option<Entity>`. The choice to modify the generation/high segment to be non-zero had the following benefits: It allowed the low/index portion of `Entity` to be zero, so index access of `Entity` from storages/archetypes/buffers in hot code paths remained untouched. Code that touched the high segment or the generation in `Entity` was in less performance sensitive sections, and when spawning, most of that code would be exactly the same, but with initialising generation with a constant `1` instead of a `0`. For `Entity` however, incrementing a generation meant it needed to ensure that if an overflow occurred, it overflowed at 31-bits **and** also overflowed to `1` instead of `0`.

More over, this choice _did not interfere_ with the unified `Identifier` approach. While for `Entity`, the generation would have to be initialised with `1`; for other ID types, the high segment would always be _non-zero_ due to flag bits being set. That means a `Pair` type (which would have an ID in both high and low segments) would be unaffected as it could reference an `Entity` id in the low segment, and have its own id in the high segment.

### Optimizing `Entity`'s Representation and Codegen further

Part two of the `Entity` optimization story continues with initial work on improving `Entity`'s `PartialEq` implementation ([#10519](https://github.com/bevyengine/bevy/pull/10519) by @scottmcm). With `Entity`'s structure, the standard `PartialEq` derive was yielding poor codegen and leaving performance on the table as the compiler was unable to make the correct inferences on how to load `Entity` for comparisons. By removing the short-circuiting implementation that is the default for `PartialEq` derivation, the compiler could output much less assembly for the same operation, yielding some performance improvements in some areas of the codebase.

However, work in this area did not stop with [#10519](https://github.com/bevyengine/bevy/pull/10519). There was further optimization potential still on the table, culminating with the PR "Optimise Entity with repr align & manual PartialOrd/Ord" ([#10558](https://github.com/bevyengine/bevy/pull/10558) by @Bluefinger). There was prior art with this PR that attempted to land similar improvements previously ([#2372](https://github.com/bevyengine/bevy/pull/2372) & [#3788](https://github.com/bevyengine/bevy/pull/3788)) but these didn't get merged due to problems at the time that didn't justify performance gains at the expense of regressions elsewhere or from complicating the `Entity` codebase.

By default, `Entity`/`Identifier`'s representation had two `u32` segments, so the struct had an alignment of 4 bytes for a structure that was 8 bytes in size. Though the same size as a `u64` value, these structs were _under-aligned_ and as such, the compiler could not treat `Entity` as if it were a `u64` value. For reference, a `u64` value has a size and alignment of 8 bytes. Certain optimizations were being left out as the necessary assumptions could not be made at compile time regarding `Entity`. So `Entity`/`Identifier` were changed to have a manually defined layout and alignment, making it clearer to the compiler to treat these structs as if it were a `u64` value:

```rust
#[derive(Clone, Copy)]
#[repr(C, align(8))]
struct Identifier {
    #[cfg(target_endian = "little")]
    low: u32,
    high: NonZeroU32,
    #[cfg(target_endian = "big")]
    low: u32,
}
```

By defining the struct with a `repr(C)`, we could tell the compiler the layout of the struct exactly for both little endian and big endian platforms. By also defining the alignment of the struct to be 8 bytes, `Entity`/`Identifier` both appear to the compiler as if it were a `u64` value. The effect of this is that the `to_bits` method for `Entity`/`Identifier` becomes a simple `mov` operation which could be completely optimised away with LTO and inlining.

Before:

```asm
to_bits:
    shl     rdi, 32
    mov     eax, esi
    or      rax, rdi
    ret
```

After:

```asm
to_bits:
    mov     rax, rdi
    ret
```

_But it doesn't stop there_. This had the effect of making hashing `Entity` even _faster_, yielding further gains on top of [#9903](https://github.com/bevyengine/bevy/pull/9903) (which landed in 0.12). It turned out that the `PartialEq` implementation could be made even quicker, by comparing directly against the bits of one `Entity` with another. With manual implementations of `PartialOrd`/`Ord`, the codegen around `Entity` was improved considerably as the compiler was now being able to treat `Entity` as a pseudo-`u64` value.

For example, this was the codegen for `Entity > Entity` before the various changes:

```asm
greater_than:
    cmp     edi, edx
    jae     .LBB3_2
    xor     eax, eax
    ret

.LBB3_2:
    setne   dl
    cmp     esi, ecx
    seta    al
    or      al, dl
    ret
```

Afterwards, it compiles to this:

```asm
greater_than:
    cmp     rdi, rsi
    seta    al
    ret
```

But why didn't this land before? It turned out that imposing an alignment of 8 bytes on `Entity` made `Option<Entity>` increase in size from the original 12 bytes to 16 bytes. As such, some code paths _suffered_ performance regressions in needing to load and move around 16 byte values. But with niching now possible, the new representation could stay at 8 bytes in size even as an `Option`, preventing the regressions from occurring in the first place.

The micro-optimizations around `EntityHasher` did not end here though, as there was a PR made to "Save an instruction in `EntityHasher`" ([#10648](https://github.com/bevyengine/bevy/pull/10648) by @scottmcm). As [#10558](https://github.com/bevyengine/bevy/pull/10558) provided a significant improvement already to `EntityHasher` performance, the hashing algorithm was revised in a way that allowed LLVM to remove one further instruction in the compiled output. The compiler was already being clever enough to combine a multiply-shift-or operation into a single multiplication, but by expressing the algorithm with slight changes, an `or` instruction could be removed while retaining the desired hashing behaviour for `EntityHasher`.

![EntityHasher assemply output diff](entityhasher_output.png)

This had a detectable improvement in benchmarks, ranging from 3-6% improvement in lookups for `EntityHashMap`. These small optimizations may not amount to much on their own, but together they can provide meaningful improvements downstream.

![Benchmark results of optimisation work](entity_hash_optimsation_benches.png)

The above results show from where we started (`optimised_eq` being the first PR that introduced the benchmarks with the "Optimise Eq" feature) to where we are now with all the optimisations in place (`optimised_entity`). Improvements across the whole board, with clear performance benefits that should impact multiple areas of the codebase, not just with entity hashing.

### Porting `Query::for_each` to `QueryIter::fold` override

Currently to get the full performance out of iterating over queries, `Query::for_each` must be used in order to take advantage of auto-vectorization and internal iteration optimizations that the compiler can apply. However, this isn't idiomatic rust and is not an iterator method so you can't use it on an iterator chain. However, it is possible to get the same benefits for some iterator methods, for which [#6773](https://github.com/bevyengine/bevy/pull/6773/) by @james7132 sought to achieve. By providing an override to `QueryIter::fold`, it was possible to port the iteration strategies of `Query::for_each` so that `Query::iter` and co could achieve the same gains. Not _every_ iterator method currently benefits from this, as they require overriding `QueryIter::try_fold`, but that is currently still a nightly-only optimisation. This approach is the same used within `std` code.

The result was deduplication of code in a few areas, such as no longer requiring both `Query::for_each` and `Query::for_each_mut`, as one just needs to call `Query::iter` or `Query::iter_mut` instead. So code like:

```rust
fn some_system(mut q_transform: Query<&mut Transform, With<Npc>>) {
    q_transform.for_each_mut(|transform| {
        // Do something...
    });
}
```

Becomes:

```rust
fn some_system(mut q_transform: Query<&mut Transform, With<Npc>>) {
    q_transform.iter_mut().for_each(|transform| {
        // Do something...
    });
}
```

The assembly output was compared as well between what was on main branch versus the PR, with no tangible differences being seen between the old `Query::for_each` and the new `QueryIter::for_each()` output, validating the approach and ensuring the internal iteration optimizations were being applied.

As a plus, the same iternal iteration optimizations in `Query::par_for_each` now reuse code from `for_each`, deduplicating code there as well and enabling users to make use of `par_iter().for_each()`. As a whole, this means there's no longer any need for `Query::for_each`, `Query::for_each_mut`, `Query::_par_for_each`, `Query::par_for_each_mut` so these methods have been deprecated for 0.13 and will be removed in 0.14.

### Reducing `TableRow` `as` casting

Not all improvements were focused around performance. Some small changes were done to improve type safety and tidy-up some of the codebase to have less `as` casting being done on various call sites for `TableRow`. The problem with `as` casting is that in some cases, the cast will fail by truncating the value silently, which could then cause havoc by accessing the wrong row and so forth. [#10811](https://github.com/bevyengine/bevy/pull/10811) by @bushrat011899 was put forward to clean up the API around `TableRow`, providing convenience methods backed by `assert`s to ensure the casting operations could never fail, or if they did, they'd panic correctly.

Naturally, _adding_ asserts in potentially hot codepaths were cause for some concern, necessitating considerable benchmarking efforts to confirm there were regressions and to what level. With careful placing of the new `assert`s, the detected regression for these cases was in the region of 0.1%, but such regressions could easily be masked by compiler randomness, optimizations, etc. But the benefit was a less error-prone API and more robust code, which for a complex codebase such as Bevy's ECS code, every little helps.

### Entity optimizations notes

* [Making the most of ECS identifiers](https://ajmmertens.medium.com/doing-a-lot-with-a-little-ecs-identifiers-25a72bd2647)
* [`Option` representation](https://doc.rust-lang.org/core/option/index.html#representation)

#### QueryIter::fold` override notes

* [Assembly Sanity check for bevyengine/bevy#6773](https://github.com/james7132/bevy_asm_tests/commit/309947cd078086b7edc4b8b5f29b1d04255b1b9a#diff-4c4b34cf83f523fced3bd396ad7ab8e228b4d35bf65c1f0457f7e4e58b14ccc5)
* [rustc bug for autovectorising internal iteration](https://github.com/rust-lang/rust/issues/104914)
* [std `Iter::fold` overriding for perf gains](https://github.com/rust-lang/rust/blob/master/library/core/src/array/iter.rs#L265-L277)

#### `TableRow` Casting notes

* [Rustonomicon on Casts](https://doc.rust-lang.org/nomicon/casts.html)

## WorldQuery trait split

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Automatically inserted sync points

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Input for one-shot systems

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## WGPU upgrade

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Texture atlas rework

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Sprite slicing and tiling

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Exposure settings

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Minimal reflection probes

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Light maps

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Light RenderLayers

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Approximate indirect specular occlusion

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Unload render assets from RAM

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Bind group layout entries

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Type-safe labels for the `RenderGraph`

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Camera-driven UI

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Winit upgrade

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Animation interpolation

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## gltF extensions

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Extensionless asset support

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Gizmo configuration

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## <a name="what-s-next"></a>What's Next?

We have plenty of work in progress! Some of this will likely land in **Bevy 0.14**.

Check out the [**Bevy 0.14 Milestone**](https://github.com/bevyengine/bevy/milestone/20) for an up-to-date list of current work that contributors are focusing on for **Bevy 0.14**.

* **More editor experimentation:** TODO
* **bevy_dev_tools:** TODO
* **A revised scene format:** TODO
* **bevy_ui improvements:** TODO
* **The steady march towards relations:** TODO
* **Animation blending:** TODO
* **Irradiance volumes:** TODO

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the 185 contributors that made this release (and associated docs) possible! In random order:

TODO: add contributors

## Full Changelog

The changes mentioned above are only the most appealing, highest impact changes that we've made this cycle.
Innumerable bug fixes, documentation changes and API usability tweaks made it in too.
For a complete list of changes, check out the PRs listed below.

TODO: add full changelog, sorting by area.
