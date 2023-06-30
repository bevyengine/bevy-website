+++
title = "Bevy 0.11"
date = 2023-07-07
[extra]
author = "Bevy Contributors"
+++

Thanks to **X** contributors, **X** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), we're happy to announce the **Bevy 0.11** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/book/getting-started/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.11**, check out our [0.10 to 0.11 Migration Guide](/learn/migration-guides/0.10-0.11/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

* **Feature**: description

## Parallax Mapping

<div class="release-feature-authors">authors: @nicopap, @superdump</div>

Bevy now supports parallax mapping and depth maps. Parallax mapping puts normal
maps to shame when it comes to giving "illusion of depth" to a material.


<video controls loop><source  src="earth-parallax.webm" type="video/webm"/></video>

Notice how it is not merely the light shading of pixel that changes, but their
actual position on screen. Notice how mountaintops hide mountain ridges behind
themselves. Notice how mountaintops move faster than coastal areas.

Parallax mapping moves pixels according to the perspective and depth on the
surface of the geometry. Adding true 3D depth to flat surfaces.

All of that, without adding a single vertex to the geometry. The whole globe
has exactly 648 vertices. Unlike a more primitive shader, such as displacement
mapping, parallax mapping only requires an additional grayscale image, called
the `depth_map`.

Games often use parallax mapping for cobblestones or brick walls, so
let's make a brick wall in bevy real quick. First, we spawn a mesh:

```rust
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(30.0, 10.0, 1.0).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        }),
        ..default()
    });
```

![A 3D desert scene with two flat white walls and a pebble path winding between them](parallax_mapping_none.jpg)

Of course, it's just a flat white box, we didn't add any texture.
So let's add a normal map:

```rust
            normal_map_texture: Some(assets.load("normal_map.png")),
```

![The same scene with normal maps](parallax_mapping_normals.jpg)

This is much better. The shading changes according to the light direction too!
However, the specular highlights on the corner are overbearing, almost noisy.

On a purely physical levels, _we shouldn't see the specular highlights_,
because the lighted part of the bricks are facing the sun (duh) not the camera.

Let's see how a depth map can help:

```rust
            depth_map: Some(assets.load("depth_map.png")),
```

![The same scene with a depth texture](parallax_mapping_depth.jpg)

We eliminated the noise! The ambient light color (blue) is also much more
visible on the brick sides. There is also that sweet 3D feel reminiscent of
90's games pre-rendered cinematic sequences.

Notice how, without parallax mapping, we always see both the shaded and lighted
sides of the ridge between bricks. And how, with parallax mapping, the lighted
side of the ridge is only visible for the lower bricks.

This is because parallax mapping insets the ridges between bricks, so that they
are occluded by the bricks themselves.

![Illustration of the previous paragraph](ridge-light-view-1.svg)

Since normal maps do not "move" the shaded areas, merely shade them
differently, we get those awkward specular highlights. With parallax mapping,
they are gone.

![A montage of the three preceding images, contrasting each effect](parallax_mapping_compare.jpg)

Parallax mapping in bevy is still very limited. The most painful aspect is that
it is not a standard glTF feature, meaning that the depth texture needs to be
programmatically added to materials, including if the material was loaded from a glTF file.

On top of that, parallax mapping is incompatible with the temporal antialiasing
shader, doesn't work well on curved surfaces and doesn't affect object's
silhouettes.

However, those are not fundamental limitations of parallax mapping, and may be
fixed in the future.

## <a name="what-s-next"></a>What's Next?

* **X**: Y

Check out the [**Bevy 0.12 Milestone**](https://github.com/bevyengine/bevy/milestone/14) for an up-to-date list of current work being considered for **Bevy 0.12**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:

* @author
