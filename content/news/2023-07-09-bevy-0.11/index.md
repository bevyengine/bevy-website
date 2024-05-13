+++
title = "Bevy 0.11"
date = 2023-07-09
authors = ["Bevy Contributors"]
[extra]
image = "with_ssao.png"
show_image = true
+++

Thanks to **166** contributors, **522** pull requests, community reviewers, and our [**generous sponsors**](/donate), we're happy to announce the **Bevy 0.11** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/quick-start/introduction/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.11**, check out our [0.10 to 0.11 Migration Guide](/learn/migration-guides/0.10-0.11/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

* **Screen Space Ambient Occlusion (SSAO)**: Increase scene render quality by simulating occlusion of "indirect" diffuse light
* **Temporal Anti-Aliasing (TAA)**: A popular anti-aliasing technique that blends the current frame with past frames using motion vectors to smooth out artifacts
* **Morph Targets**: Animate vertex positions on meshes between predefined states. Great for things like character customization!
* **Robust Contrast Adaptive Sharpening (RCAS)**: Intelligently sharpens renders, which pairs nicely with TAA
* **WebGPU Support**: Bevy can now render on the web faster and with more features using the modern WebGPU web API
* **Improved Shader Imports**: Bevy shaders now support granular imports and other new features
* **Parallax Mapping**: Materials now support an optional depth map, giving flat surfaces a feel of depth through parallaxing the material's textures
* **Schedule-First ECS APIs**: A simpler and more ergonomic ECS system scheduling API
* **Immediate Mode Gizmo Rendering**: Easily and efficiently render 2D and 3D shapes for debugging and editor scenarios
* **ECS Audio APIs**: A more intuitive and idiomatic way to play back audio
* **UI Borders**: UI nodes can now have configurable borders!
* **Grid UI Layout**: Bevy UI now supports CSS-style grid layout
* **UI Performance Improvements**: The UI batching algorithm was changed, yielding significant performance wins

## Screen Space Ambient Occlusion

<div class="release-feature-authors">authors: @JMS55, @danchia, @superdump </div>

{{ compare_slider(
    left_title="Without SSAO",
    left_alt = "The Sponza scene without SSAO, it contains a lot of persian-style velvet curtains, they look awkward.",
    left_image="no_ssao.png",
    right_title="With SSAO",
    right_alt = "The Sponza scene with SSAO, the curtains look much more realistic and sculptued. SSAO darkens the ridges between the folds, making the curtains much more interesting to look at",
    right_image="with_ssao.png"
) }}

**SSAO Only**
![ssao_only](ssao_only.png)

Bevy now supports Screen Space Ambient Occlusion (SSAO). While Bevy already supported shadows from direct lights
([`DirectionalLight`], [`PointLight`], [`SpotLight`]) via shadow mapping, Bevy now supports shadows from _indirect_ diffuse lighting such as [`AmbientLight`] or [`EnvironmentMapLight`].

These shadows give scenes a more "grounded" feel, by estimating how much surrounding geometry blocks incoming light via the screen-space depth and normal prepasses. You can try it out in the new [SSAO example](https://github.com/bevyengine/bevy/blob/v0.11.0/examples/3d/ssao.rs).

Note that using SSAO with the newly added Temporal Anti-Aliasing leads to a _large_ increase in quality and noise reduction.

Platform support is currently limited - Only Vulkan, DirectX12, and Metal are currently supported. WebGPU support will come at a later date. WebGL likely won't be supported because it doesn't have compute shaders.

Special thanks to Intel for their open source [XeGTAO](https://github.com/GameTechDev/XeGTAO) project, which was a huge help in developing this feature.

[`DirectionalLight`]: https://docs.rs/bevy/0.11.0/bevy/pbr/struct.DirectionalLight.html
[`PointLight`]: https://docs.rs/bevy/0.11.0/bevy/pbr/struct.PointLight.html
[`SpotLight`]: https://docs.rs/bevy/0.11.0/bevy/pbr/struct.SpotLight.html
[`AmbientLight`]: https://docs.rs/bevy/0.11.0/bevy/pbr/struct.AmbientLight.html

## Temporal Anti-Aliasing

<div class="release-feature-authors">authors: @JMS55, @DGriffin91</div>

{{ compare_slider(
    left_title="MSAA",
    left_alt="The Helmet model with MSAA, anti-aliasing. The edge between meshes are well aliased, but crenellation is noticeable on sharp shadows and specular highlights",
    left_image="msaa_helmet.png",
    right_title="TAA",
    right_alt="With TAA, little crenellation is visible, but it feels a bit 'smudgy'",
    right_image="taa_helmet.png",
    start_slider_at="39%",
    height="619px"
) }}

Alongside MSAA and FXAA, Bevy now supports Temporal Anti-aliasing (TAA) as an anti-aliasing option.

TAA works by blending the newly rendered frame with past frames in order to smooth out aliasing artifacts in the image. TAA has become increasingly popular in the industry because of its ability to cover up so many rendering artifacts: it smooths out shadows (both global illumination and "casted" shadows), mesh edges, textures, and reduces specular aliasing of light on reflective surfaces. However because the "smoothing" effect is so apparent, some people prefer other methods.

Here's a quick rundown of the following advantages and disadvantages of each anti-aliasing method that Bevy supports:

* **Multi-Sample Antialiasing (MSAA)**
  * Does a good job at smoothing the edges of meshes (anti geometric aliasing). Does not help with specular aliasing. Performance cost scales with triangle count, and performs very poorly on scenes with many triangles
* **Fast Approximate Antialiasing (FXAA)**
  * Does a decent job of dealing with both geometric and specular aliasing. Very little performance cost in all scenes. Somewhat blurry and low quality results
* **Temporal Antialiasing (TAA)**
  * Does a very good job at dealing with both geometric and specular aliasing. Does a good job at dealing with temporal aliasing, where high-frequency details flicker over time or as you move the camera around or as things animate. Performance cost is moderate, and scales only with screen resolution. Chance of "ghosting" where meshes or lighting effects may leave trails behind them that fade over time. Although TAA helps with reducing temporal aliasing, it may also introduce additional temporal aliasing, especially on thin geometry or texture detail rendered at a distance. Requires 2 view's worth of additional GPU memory, as well as enabling the motion vector and depth prepasses. Requires accurate motion vector and depth prepasses, which complicates custom materials

TAA implementations are a series of tradeoffs and rely on heuristics that are easy to get wrong. In Bevy 0.11, TAA is marked as an experimental feature for the following reasons:

* TAA does not currently work with the following Bevy features: skinning, morph targets, and parallax mapping
* TAA currently tends to soften the image a bit, which can be worked around via post-process sharpening
* Our TAA heuristics are not currently user-configurable (and these heuristics are likely to change and evolve)

We will continue to improve quality, compatibility, and performance in future releases. Please report any bugs you encounter!

You can compare all of our anti-aliasing methods in Bevy's improved [anti-aliasing example](https://github.com/bevyengine/bevy/blob/v0.11.0/examples/3d/anti_aliasing.rs).

## Robust Contrast Adaptive Sharpening

<div class="release-feature-authors">authors: @Elabajaba</div>

Effects like TAA and FXAA can cause the final render to become blurry. Sharpening post processing effects can help counteract that. In **Bevy 0.11** we've added a port of AMD's Robust Contrast Adaptive Sharpening (RCAS).

{{ compare_slider(
    left_title="TAA",
    left_image="rcas_off.png",
    right_title="TAA+RCAS",
    right_image="rcas_on.png",
    start_slider_at="55%"
) }}

Notice that the texture on the leather part of the helmet is much crisper!

## Morph Targets

<div class="release-feature-authors">authors: @nicopap, @cart</div>

Bevy, since the 0.7 release, supports 3D animations.

But it only supported _skeletal_ animations. Leaving on the sidewalk a common
animation type called _morph targets_ (aka blendshapes, aka keyshapes, and a slew
of other names). This is the grandparent of all 3D character animation!
[Crash Bandicoot]'s run cycle used morph targets.

<video controls><source src="morph_targets_video.mp4" type="video/mp4"/></video>
<div style="font-size: 1.0rem" class="release-feature-authors">Character model by <a href="https://www.artstation.com/zambrah">Samuel Rosario</a> (© all rights reserved), used with permission. Modified by nicopap, using the <a href="https://studio.blender.org/characters/snow/v2/">Snow</a> character texture by Demeter Dzadik for Blender Studios <a href="https://creativecommons.org/licenses/by/4.0/">(🅯 CC-BY)</a>.
</div>
<!-- The previous paragraph requires the <a href> tags, since zola doesn't
process markdown markup within tags -->

Nowadays, an animation artist will typically use a skeleton rig for wide
moves and morph targets to clean up the detailed movements.

When it comes to game assets, however, the complex skeleton rigs used by
artists for faces and hands are too heavy. Usually, the poses are
"baked" into morph poses, and facial expression transitions are handled
in the engine through morph targets.

Morph targets is a very simple animation method. Take a model, have a base
vertex position, move the vertices around to create several poses:

<div style="flex-direction:row;display:flex;justify-content:space-evenly">
<div style="display:flex;flex-direction:column;align-items:center;width:20%"><p><b>Default</b></p><img alt="A wireframe rendering of a character's face with a neutral expression" src="default-pose-bw.png"></div>
<div style="display:flex;flex-direction:column;align-items:center;width:20%"><p><b>Frown</b></p><img alt="Wireframe rendering of a frowning character" src="frown-pose-bw.png"></div>
<div style="display:flex;flex-direction:column;align-items:center;width:20%"><p><b>Smirk</b></p><img alt="Wireframe rendering of a smirking character" src="smirk-pose-bw.png"></div>
</div>

Store those poses as a difference between the default base mesh and the variant
pose, then, at runtime, _mix_ each pose. Now that we have the difference with
the base mesh, we can get the variant pose by simply adding to the base
vertices positions.

That's it, the morph target shader looks like this:

```rust
fn morph_vertex(vertex: Vertex) {
    for (var i: u32 = 0u; i < pose_count(); i++) {
        let weight = weight_for_pose(i);
        vertex.position += weight * get_difference(vertex.index, position_offset, i);
        vertex.normal += weight * get_difference(vertex.index, normal_offset, i);
    }
}
```

In Bevy, we store the weights per pose in the `MorphWeights` component.

```rust
fn set_weights_system(mut morph_weights: Query<&mut MorphWeights>) {
    for mut entity_weights in &mut morph_weights {
        let weights = entity_weights.weights_mut();

        weights[0] = 0.5;
        weights[1] = 0.25;
    }
}
```

Now assuming that we have two morph targets, (1) the frown pose, (2)
the smirk pose:

<div style="flex-direction:row;display:flex;justify-content:space-evenly">
<div style="display:flex;flex-direction:column;align-items:center;width:12%">
  <p><b>[0.0, 0.0]</b></p>
  <p style="margin:0;font-size:75%">default pose</p>
  <img alt="Neutral face expression" src="morph_target_default-0.png">
</div>
<div style="display:flex;flex-direction:column;align-items:center;width:12%">
  <p><b>[1.0, 0.0]</b></p>
  <p style="margin:0;font-size:75%">frown only</p>
  <img alt="Frowning" src="morph_target_frown-0.png">
</div>
<div style="display:flex;flex-direction:column;align-items:center;width:12%">
  <p><b>[0.0, 1.0]</b></p>
  <p style="margin:0;font-size:75%">smirk only</p>
  <img alt="Smirking" src="morph_target_smirk.png">
</div>
<div style="display:flex;flex-direction:column;align-items:center;width:12%">
  <p><b>[0.5, 0.0]</b></p>
  <p style="margin:0;font-size:75%">half frown</p>
  <img alt="Slightly frowning" src="morph_target_frown-half-0.png">
</div>
<div style="display:flex;flex-direction:column;align-items:center;width:12%">
  <p><b>[1.0, 1.0]</b></p>
  <p style="margin:0;font-size:75%">both at max</p>
  <img alt="Making faces" src="morph_target_both-0.png">
</div>
<div style="display:flex;flex-direction:column;align-items:center;width:12%">
  <p><b>[0.5, 0.25]</b></p>
  <p style="margin:0;font-size:75%">bit of both</p>
  <img alt="Slightly frowning/smirking" src="morph_target_smirk-quarter-frown-half-0.png">
</div>
</div>

While conceptually simple, it requires communicating to the GPU a tremendous
amount of data. Thousand of vertices, each 288 bits, several model variations,
sometimes a hundred.

We store the vertex data as pixels in a 3D texture. This allows morph targets to not only
run on WebGPU, but also on the WebGL2 wgpu backend.

This could be improved in a number of ways, but it is sufficient for an
initial implementation.

<video controls><source src="morph_target_smirk.mp4" type="video/mp4"/></video>

[Crash Bandicoot]: https://en.wikipedia.org/wiki/Crash_Bandicoot_(video_game)#Gameplay

## Parallax Mapping

<div class="release-feature-authors">author: @nicopap</div>

Bevy now supports parallax mapping and depth maps. Parallax mapping puts normal
maps to shame when it comes to giving "illusion of depth" to a material. The top half of this video uses parallax mapping plus a normal map, whereas the bottom half only uses a normal map:

<video controls loop><source alt="a rotating view of the earth, the top half of the screen uses parallax mapping, while the bottom half does not" src="earth-parallax.mp4" type="video/mp4"/></video>
<div style="font-size: 1.0rem" class="release-feature-authors">earth view, elevation & night view by NASA (public domain)</div>

Notice how it is not merely the shading of pixels that changes, but their
actual position on screen. The mountaintops hide mountain ridges behind
themselves. High mountains move faster than coastal areas.

Parallax mapping moves pixels according to the perspective and depth on the
surface of the geometry. Adding true 3D depth to flat surfaces.

All of that, without adding a single vertex to the geometry. The whole globe
has exactly 648 vertices. Unlike a more primitive shader, such as displacement
mapping, parallax mapping only requires an additional grayscale image, called
the `depth_map`.

Games often use parallax mapping for cobblestones or brick walls, so
let's make a brick wall in Bevy! First, we spawn a mesh:

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

![A 3D desert scene with two flat white walls and a pebble path winding between them](parallax_mapping_none_mini.jpg)

Of course, it's just a flat white box, we didn't add any texture.
So let's add a normal map:

```rust
normal_map_texture: Some(assets.load("normal_map.png")),
```

![The same scene with normal maps](parallax_mapping_normals_mini.jpg)

This is much better. The shading changes according to the light direction too!
However, the specular highlights on the corner are overbearing, almost noisy.

Let's see how a depth map can help:

```rust
depth_map: Some(assets.load("depth_map.png")),
```

![The same scene with a depth texture](parallax_mapping_depth_mini.jpg)

We eliminated the noise! There is also that sweet 3D feel reminiscent of
90's games pre-rendered cinematic sequences.

So what's going on, why does parallax mapping eliminate the ugly specular
lights on the wall?

This is because parallax mapping insets the ridges between bricks, so that they
are occluded by the bricks themselves.

![Illustration of the previous paragraph](ridge-light-view-1.svg)

Since normal maps do not "move" the shaded areas, merely shade them
differently, we get those awkward specular highlights. With parallax mapping,
they are gone.

{{ compare_slider(
    left_title="Normals Only",
    left_image="parallax_mapping_normals.jpg",
    right_title="Parallax & Normal Mapping",
    right_image="parallax_mapping_depth.jpg",
    start_slider_at="40%"
) }}

Parallax mapping in Bevy is still very limited. The most painful aspect is that
it is not a standard glTF feature, meaning that the depth texture needs to be
programmatically added to materials if they came from a GLTF file.

Additionally, parallax mapping is incompatible with the temporal antialiasing
shader, doesn't work well on curved surfaces, and doesn't affect object's
silhouettes.

However, those are not fundamental limitations of parallax mapping, and may be
fixed in the future.

## Skyboxes

<div class="release-feature-authors">authors: @JMS55, @superdump</div>

![skybox](skybox.png)

Bevy now has built-in support for displaying an HDRI environment as your scene background.

Simply attach the new [`Skybox`] component to your [`Camera`]. It pairs well with the existing [`EnvironmentMapLight`], which will use the environment map to light the scene.

We also plan to add support for built-in procedural skyboxes sometime in the future!

[`Skybox`]: https://docs.rs/bevy/0.11.0/bevy/core_pipeline/struct.Skybox.html
[`Camera`]: https://docs.rs/bevy/0.11.0/bevy/render/camera/struct.Camera.html
[`EnvironmentMapLight`]: https://docs.rs/bevy/0.11.0/bevy/pbr/struct.EnvironmentMapLight.html

## WebGPU Support

<div class="release-feature-authors">authors: @mockersf, many others throughout Bevy's development</div>

![webgpu](webgpu.svg)

Bevy now supports WebGPU rendering on the web (in addition to WebGL 2). WebGPU support is still rolling out, but if you have [a supported web browser][webgpu-support] you can explore our new [live WebGPU examples](/examples-webgpu) page.

### What is WebGPU?

WebGPU is an [exciting new web standard](https://github.com/gpuweb/gpuweb) for doing modern GPU graphics and compute. It takes inspiration from Vulkan, Direct3D 12, and Metal. In fact, it is generally implemented on top of these APIs under the hood. WebGPU gives us access to more GPU features than WebGL2 (such as compute shaders) and also has the potential to be much faster. It means that more of Bevy's native renderer features are now also available on the web. It also uses the new [WGSL shader language](https://www.w3.org/TR/WGSL). We're very happy with how WGSL has evolved over time and Bevy uses it internally for our shaders. We also added usability features like imports! But with Bevy you still have the option to use GLSL if you prefer.

### How it Works

Bevy is built on top of the [wgpu] library, which is a modern low-level GPU API that can target pretty much every popular API: Vulkan, Direct3D 12, Metal, OpenGL, WebGL2, and WebGPU. The best backend API is selected for a given platform. It is a "native" rendering API, but it generally follows the WebGPU terminology and API design. Unlike WebGPU, it can provide direct access to the native APIs, which means Bevy [enjoys a "best of all worlds" situation](/news/bevy-webgpu/#how-it-works).

### WebGPU Examples

Click one of the images below to check out our live WebGPU examples (if your [browser supports it][webgpu-support]):

[![webgpu examples](webgpu_examples.png)](/examples-webgpu)

[wgpu]: https://github.com/gfx-rs/wgpu
[webgpu-support]: https://caniuse.com/webgpu

## Improved Shader Imports

<div class="release-feature-authors">authors: @robtfm</div>

Bevy's rendering engine has a lot of great options and features. For example, the PBR `StandardMaterial` pipeline supports desktop/webgpu and webgl, 6 optional mesh attributes, 4 optional textures, and a plethora of optional features like fog, skinning, and alpha blending modes, with more coming in every release.

Many feature combos need specialized shader variants, and with over 3000 lines of shader code split over 50 files in total, the text-substitution-based shader processor was beginning to creak at the seams.

This release we've switched to using [naga_oil](https://github.com/bevyengine/naga_oil), which gives us a module-based shader framework. It compiles each file individually to naga's IR and then combines them into a final shader on demand. This doesn't have much visible impact yet, but it does give a few immediate benefits:

* The engine's shader code is easier to navigate and less magical. Previously there was only a single global scope, so items could be referenced even if they were only imported indirectly. This sometimes made it hard to locate the actual code behind the reference. Now items must be explicitly imported, so you can always tell where a variable or function originated just by looking at the current file: <br/>
![imported items](imported_items.png)
* Shaders now have codespan reporting, an error will point you to the shader file and line number, preventing a lot of hair pulling in complex shader codebases: <br/>
![codespan](codespan.png)
* naga_oil's preprocessor supports a few more conditional directives, you can use `#else if` and `#else ifndef` as well as `#else ifdef` which was previously supported
* Functions, variables and structs are all properly scoped so a shader file doesn't need to use globally unique names to avoid conflicts
* Shader defs can be added to modules directly. For example, any shader that imports `bevy_pbr::mesh_view_types` now has `MAX_DIRECTIONAL_LIGHTS` automatically defined, there's no longer a need to remember to add it for every new pipeline that uses the module.

The future possibilities are more exciting. Using naga IR opens the door to a bunch of nice features that we hope to bring in future releases:

* Automatic bind slot allocation will let plugins extend the core view bindgroup, which means self-contained plugins for features like lighting and shadow methods, common material properties, etc become viable. This will allow us to modularise the core pipelines to make growing the codebase - while keeping support for multiple targets - more sustainable
* "Virtual" shader functions will allow user modifications to core functions (like lighting), and potentially lead to a template-style material system, where users can provide "hooks" that will be called at the right point in the pipeline
* Language interop: mix and match glsl and wgsl, so bevy's pbr pipeline features could be accessed from your glsl material shader, or utils written for glsl could be used in wgsl code. We're hopeful that this can extend to spirv (and rust-gpu) as well
* More cool stuff we haven't thought of yet. Being able to inspect and modify shaders at runtime is very powerful and makes a lot of things possible!

## UI Node Borders

<div class="release-feature-authors">authors: @ickshonpe</div>

UI nodes now draws borders, whose color can be configured with the new [`BorderColor`] component:

![borders](borders.png)

```rust
commands.spawn(ButtonBundle {
    style: Style {
        border: UiRect::all(Val::Px(5.0)),
        ..default()
    },
    border_color: BorderColor(Color::rgb(0.9, 0.9, 0.9)),
    ..default()
})
```

Each side of the border is configurable:

![border sides](border-sides.png)

[`BorderColor`]: https://docs.rs/bevy/0.11.0/bevy/ui/struct.BorderColor.html

## Grid UI Layout

<div class="release-feature-authors">authors: @nicoburns</div>

In Bevy UI we wired up the new `grid` feature in the layout library we use ([Taffy](https://github.com/DioxusLabs/taffy)). This enables CSS-style grid layouts:

![grid](grid.png)

This can be configured on the [`Style`] component:

```rust
Style {
    /// Use grid layout for this node
    display: Display::Grid,
    /// Make the grid have a 1:1 aspect ratio
    /// This means the width will adjust to match the height
    aspect_ratio: Some(1.0),
    // Add 24px of padding around the grid
    padding: UiRect::all(Val::Px(24.0)),
    /// Set the grid to have 4 columns all with sizes minmax(0, 1fr)
    /// This creates 4 exactly evenly sized columns
    grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
    /// Set the grid to have 4 rows all with sizes minmax(0, 1fr)
    /// This creates 4 exactly evenly sized rows
    grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
    /// Set a 12px gap/gutter between rows and columns
    row_gap: Val::Px(12.0),
    column_gap: Val::Px(12.0),
    ..default()
},
```

[`Style`]: https://docs.rs/bevy/0.11.0/bevy/ui/struct.Style.html

## Schedule-First ECS APIs

<div class="release-feature-authors">authors: @cart</div>

In **Bevy 0.10** we introduced [ECS Schedule V3](/news/bevy-0-10/#ecs-schedule-v3), which _vastly_ improved the capabilities of Bevy ECS system scheduling: scheduler API ergonomics, system chaining, the ability to run exclusive systems and apply deferred system operations at any point in a schedule, a single unified schedule, configurable System Sets, run conditions, and a better State system.

However it pretty quickly became clear that the new system still had some areas to improve:

* **Base Sets were hard to understand and error prone**: What _is_ a Base Set? When do I use them? Why do they exist? Why is my ordering implicitly invalid due to incompatible Base Set ordering? Why do some schedules have a default Base Set while others don't? [Base Sets were confusing!](https://github.com/bevyengine/bevy/pull/8079#base-set-confusion)
* **There were too many ways to schedule a System**: We've accumulated too many scheduling APIs. As of Bevy **0.10**, we had [_SIX_ different ways to add a system to the "startup" schedule](https://github.com/bevyengine/bevy/pull/8079#unify-system-apis). Thats too many ways!
* **Too much implicit configuration**: There were both default Schedules and default Base Sets. In some cases systems had default schedules or default base sets, but in other cases they didn't! [A system's schedule and configuration should be explicit and clear](https://github.com/bevyengine/bevy/pull/8079#schedule-should-be-clear).
* **Adding Systems to Schedules wasn't ergonomic**: Things like `add_system(foo.in_schedule(CoreSchedule::Startup))` were not fun to type or read. We created special-case helpers, such as `add_startup_system(foo)`, but [this required more internal code, user-defined schedules didn't benefit from the special casing, and it completely hid the `CoreSchedule::Startup` symbol!](https://github.com/bevyengine/bevy/pull/8079#ergonomic-system-adding).

### Unraveling the Complexity

If your eyes started to glaze over as you tried to wrap your head around this, or phrases like "implicitly added to the `CoreSet::Update` Base Set" filled you with dread ... don't worry. After [a lot of careful thought](https://github.com/bevyengine/bevy/pull/8079) we've unraveled the complexity and built something clear and simple.

In **Bevy 0.11** the "scheduling mental model" is _much_ simpler thanks to **Schedule-First ECS APIs**:

```rust
app
    .add_systems(Startup, (a, b))
    .add_systems(Update, (c, d, e))
    .add_systems(FixedUpdate, (f, g))
    .add_systems(PostUpdate, h)
    .add_systems(OnEnter(AppState::Menu), enter_menu)
    .add_systems(OnExit(AppState::Menu), exit_menu)
```

* **There is _exactly_ one way to schedule systems**
  * Call `add_systems`, state the schedule name, and specify one or more systems
* **Base Sets have been entirely removed in favor of Schedules, which have friendly / short names**
  * Ex: The `CoreSet::Update` Base Set has become `Update`
* **There is no implicit or implied configuration**
  * Default Schedules and default Base Sets don't exist
* **The syntax is easy on the eyes and ergonomic**
  * Schedules are first so they "line up" when formatted

<details>
    <summary>To compare, expand this to see what it used to be!</summary>

```rust
app
    // Startup system variant 1.
    // Has an implied default StartupSet::Startup base set
    // Has an implied CoreSchedule::Startup schedule
    .add_startup_systems((a, b))
    // Startup system variant 2.
    // Has an implied default StartupSet::Startup base set
    // Has an implied CoreSchedule::Startup schedule
    .add_systems((a, b).on_startup())
    // Startup system variant 3.
    // Has an implied default StartupSet::Startup base set
    .add_systems((a, b).in_schedule(CoreSchedule::Startup))
    // Update system variant 1.
    // `CoreSet::Update` base set and `CoreSchedule::Main` are implied
    .add_system(c)
    // Update system variant 2 (note the add_system vs add_systems difference)
    // `CoreSet::Update` base set and `CoreSchedule::Main` are implied
    .add_systems((d, e))
    // No implied default base set because CoreSchedule::FixedUpdate doesn't have one
    .add_systems((f, g).in_schedule(CoreSchedule::FixedUpdate))
    // `CoreSchedule::Main` is implied, in_base_set overrides the default CoreSet::Update set
    .add_system(h.in_base_set(CoreSet::PostUpdate))
    // This has no implied default base set
    .add_systems(enter_menu.in_schedule(OnEnter(AppState::Menu)))
    // This has no implied default base set
    .add_systems(exit_menu.in_schedule(OnExit(AppState::Menu)))
```

</details>

Note that normal "system sets" still exist! You can still use sets to organize and order your systems:

```rust
app.add_systems(Update, (
    (walk, jump).in_set(Movement),
    collide.after(Movement),
))
```

The `configure_set` API has also been adjusted for parity:

```rust
// Bevy 0.10
app.configure_set(Foo.after(Bar).in_schedule(PostUpdate))
// Bevy 0.11
app.configure_set(PostUpdate, Foo.after(Bar))
```

## Nested System Tuples and Chaining

<div class="release-feature-authors">authors: @cart</div>

It is now possible to infinitely nest tuples of systems in a `.add_systems` call!

```rust
app.add_systems(Update, (
    (a, (b, c, d, e), f),
    (g, h),
    i
))
```

At first glance, this might not seem very useful. But in combination with per-tuple configuration, it allows you to easily and cleanly express schedules:

```rust
app.add_systems(Update, (
    (attack, defend).in_set(Combat).before(check_health),
    check_health,
    (handle_death, respawn).after(check_health)
))
```

`.chain()` has also been adapted to support arbitrary nesting! The ordering in the example above could be rephrased like this:

```rust
app.add_systems(Update,
    (
        (attack, defend).in_set(Combat),
        check_health,
        (handle_death, respawn)
    ).chain()
)
```

This will run `attack` and `defend` first (in parallel), then `check_health`, then `handle_death` and `respawn` (in parallel).

This allows for powerful and expressive "graph-like" ordering expressions:

```rust
app.add_systems(Update,
    (
        (a, (b, c, d).chain()),
        (e, f),
    ).chain()
)
```

This will run `a` in parallel with `b->c->d`, then after those have finished running it will run `e` and `f` in parallel.

## Gizmos

<div class="release-feature-authors">authors: @devil-ira, @mtsr, @aevyrie, @jannik4, @lassade, @The5-1, @Toqozz, @nicopap</div>

It is often helpful to be able to draw simple shapes and lines in 2D and 3D for things like editor controls, and debug views. Game development is a very "spatial" thing and being able to quickly draw shapes is the visual equivalent of "print line debugging". It helps answer questions like "is this ray casting in the right direction?" and "is this collider big enough?"

In **Bevy 0.11** we've added an "immediate mode" [`Gizmos`] drawing API that makes these things easy and efficient. In 2D and 3D you can draw lines, rects, circles, arcs, spheres, cubes, line strips, and more!

**2D Gizmos**
![2d gizmos](2d_gizmos.png)
**3D Gizmos**
![3d gizmos](3d_gizmos.png)

From any system you can spawn shapes into existence (for both 2D and 3D):

```rust
fn system(mut gizmos: Gizmos) {
    // 2D
    gizmos.line_2d(Vec2::new(0., 0.), Vec2::new(0., 10.), Color::RED);
    gizmos.circle_2d(Vec2::new(0., 0.), 40., Color::BLUE);
    // 3D
    gizmos.circle(Vec3::ZERO, Vec3::Y, 3., Color::BLACK);
    gizmos.ray(Vec3::new(0., 0., 0.), Vec3::new(5., 5., 5.), Color::BLUE);
    gizmos.sphere(Vec3::ZERO, Quat::IDENTITY, 3.2, Color::BLACK)
}
```

Because the API is "immediate mode", gizmos will only be drawn on frames where they are "queued up", which means you don't need to worry about cleaning up gizmo state!

Gizmos are drawn in batches, which means they are very cheap. You can have hundreds of thousands of them!

[`Gizmos`]: https://docs.rs/bevy/0.11.0/bevy/gizmos/gizmos/struct.Gizmos.html

## ECS Audio APIs

<div class="release-feature-authors">authors: @inodentry</div>

Bevy's audio playback APIs have been reworked to integrate more cleanly with Bevy's ECS.

In previous versions of Bevy you would play back audio like this:

```rust
#[derive(Resource)]
struct MyMusic {
    sink: Handle<AudioSink>,
}

fn play_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>
) {
    let weak_handle = audio.play(asset_server.load("my_music.ogg"));
    let strong_handle = audio_sinks.get_handle(weak_handle);
    commands.insert_resource(MyMusic {
        sink: strong_handle,
    });
}
```

That is a lot of boilerplate just to play a sound! Then to adjust playback you would access the [`AudioSink`] like this:

```rust

fn pause_music(my_music: Res<MyMusic>, audio_sinks: Res<Assets<AudioSink>>) {
    if let Some(sink) = audio_sinks.get(&my_music.sink) {
        sink.pause();
    }
}
```

Treating audio playback as a resource created a number of problems and notably didn't play well with things like Bevy Scenes. In **Bevy 0.11**, audio playback is represented as an [`Entity`] with [`AudioBundle`] components:

```rust
#[derive(Component)]
struct MyMusic;

fn play_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("my_music.ogg"),
            ..default()
        },
        MyMusic,
    ));
}
```

The `mode` field in the [`PlaybackSettings`] struct offers a straightforward way to manage the lifecycle of these audio entities.

By passing a [`PlaybackMode`], you are able to choose whether it plays once or repeatedly, using `Once` and `Loop` respectively. If you anticipate that the audio might be played again, you can save resources by temporarily unloading it using `Despawn`, or free up its memory immediately if it is a one-time effect using `Remove`.

```rust
AudioBundle {
    source: asset_server.load("hit_sound.ogg"),
    settings: PlaybackSettings {
        mode: PlaybackMode::Despawn,
        ..default()
    }
}
```

Much simpler! To adjust playback you can query for the [`AudioSink`] component:

```rust
fn pause_music(query_music: Query<&AudioSink, With<MyMusic>>) {
    if let Ok(sink) = query.get_single() {
        sink.pause();
    }
}
```

[`Entity`]: https://docs.rs/bevy/0.11.0/bevy/ecs/entity/struct.Entity.html
[`AudioBundle`]: https://docs.rs/bevy/0.11.0/bevy/audio/type.AudioBundle.html
[`AudioSink`]: https://docs.rs/bevy/0.11.0/bevy/audio/struct.AudioSink.html
[`PlaybackSettings`]: https://docs.rs/bevy/0.11.0/bevy/audio/struct.PlaybackSettings.html
[`PlaybackMode`]: https://docs.rs/bevy/0.11.0/bevy/audio/enum.PlaybackMode.html

## Global Audio Volume

<div class="release-feature-authors">authors: @mrchantey</div>

Bevy now has a global volume level which can be configured via the [`GlobalVolume`] resource:

```rust
app.insert_resource(GlobalVolume::new(0.2));
```

## Resource Support in Scenes

<div class="release-feature-authors">authors: @Carbonhell, @Davier</div>

Bevy's scene format is a very useful tool for serializing and deserializing game state to and from scene files.

Previously, the captured state was limited to only entities and their components.
With **Bevy 0.11**, scenes now support serializing resources as well.

This adds a new `resources` field to the scene format:

```rust
(
    resources: {
        "my_game::stats::TotalScore": (
            score: 9001,
        ),
    },
    entities: {
        // Entity scene data...
    },
)
```

## Scene Filtering

<div class="release-feature-authors">authors: @MrGVSV</div>

When serializing data to a scene, all components and [resources](#resource-support-in-scenes) are serialized by default.
In previous versions, you had to use the given `TypeRegistry` to act as a filter, leaving out the types you don't want included.

In 0.11, there's now a dedicated `SceneFilter` type to make filtering easier, cleaner, and more intuitive.
This can be used with [`DynamicSceneBuilder`](https://docs.rs/bevy/0.11.0/bevy/prelude/struct.DynamicSceneBuilder.html) to have fine-grained control over what actually gets serialized.

We can `allow` a subset of types:

```rust
let mut builder = DynamicSceneBuilder::from_world(&world);
let scene = builder
    .allow::<ComponentA>()
    .allow::<ComponentB>()
    .extract_entity(entity)
    .build();
```

Or `deny` them:

```rust
let mut builder = DynamicSceneBuilder::from_world(&world);
let scene = builder
    .deny::<ComponentA>()
    .deny::<ComponentB>()
    .extract_entity(entity)
    .build();
```

## Default Font

<div class="release-feature-authors">authors: @mockersf</div>

Bevy now supports a configurable default font and embeds a tiny default font (a minimal version of [Fira Mono](https://fonts.google.com/specimen/Fira+Mono)). This is useful if you use a common font throughout your project. And it makes it easier to prototype new changes with a "placeholder font" without worrying about setting it on each node.

![default font](default_font.png)

## UI Texture Atlas Support

<div class="release-feature-authors">authors: @mwbryant</div>

Previously UI `ImageBundle` Nodes could only use handles to full images without an ergonomic way to use `TextureAtlases` in UI.  In this release we add support for an `AtlasImageBundle` UI Node which brings the existing `TextureAtlas` support into UI.

This was achieved by merging the existing mechanisms that allows text rendering to select which glyph to use and the mechanisms that allow for `TextureAtlasSprite`.

<video controls><source src="texture_atlas_ui.mp4" type="video/mp4"/></video>

## Gamepad Rumble API

<div class="release-feature-authors">authors: @johanhelsing, @nicopap</div>

You can now use the `EventWriter<GamepadRumbleRequest>` system parameter to
trigger controllers force-feedback motors.

[`gilrs`], the crate Bevy uses for gamepad support, allows controlling
force-feedback motors. Sadly, there were no easy way of accessing the
force-feedback API in Bevy without tedious bookkeeping.

Now Bevy has the `GamepadRumbleRequest` event to do just that.

```rust
fn rumble_system(
    gamepads: Res<Gamepads>,
    mut rumble_requests: EventWriter<GamepadRumbleRequest>,
) {
    for gamepad in gamepads.iter() {
        rumble_requests.send(GamepadRumbleRequest::Add {
            gamepad,
            duration: Duration::from_secs(5),
            intensity: GamepadRumbleIntensity::MAX,
        });
    }
}
```

The `GamepadRumbleRequest::Add` event triggers a force-feedback motor,
controlling how long the vibration should last, the motor to activate,
and the vibration strength. `GamepadRumbleRequest::Stop` immediately stops all motors.

[`gilrs`]: https://crates.io/crates/gilrs

## New Default Tonemapping Method

<div class="release-feature-authors">authors: @JMS55</div>

In **Bevy 0.10** we [made tonemapping configurable with a ton of new tonemapping options](/news/bevy-0-10/#more-tonemapping-choices). In **Bevy 0.11** we've switched the default tonemapping method from "Reinhard luminance" tonemapping to "TonyMcMapface":

{{ compare_slider(
    left_title="Reinhard-luminance",
    left_image="tm_reinhard_luminance.png",
    right_title="TonyMcMapface",
    right_image="tm_tonymcmapface.png"
) }}

TonyMcMapface ([created by Tomasz Stachowiak](https://github.com/h3r2tic/tony-mc-mapface)) is a much more neutral display transform that tries to stay as close to the input "light" as possible. This helps retain artistic choices in the scene. Notably, brights desaturate across the entire spectrum (unlike Reinhard luminance). It also works much better with bloom when compared to Reinhard luminance.

## EntityRef Queries

<div class="release-feature-authors">authors: @james7132</div>

[`EntityRef`] now implements [`WorldQuery`], which makes it easier to query for arbitrary components in your ECS systems:

```rust
fn system(query: Query<EntityRef>) {
    for entity in &query {
        if let Some(mesh) = entity.get::<Handle<Mesh>>() {
            let transform = entity.get::<Transform>().unwrap();
        }
    }
}
```

Note that [`EntityRef`] queries access every entity and every component in the entire [`World`] by default. This means that they will conflict with any "mutable" query:

```rust
/// These queries will conflict, making this system invalid
fn system(query: Query<EntityRef>, mut enemies: Query<&mut Enemy>) { }
```

To resolve conflicts (or reduce the number of entities accessed), you can add filters:

```rust
/// These queries will not conflict
fn system(
    players: Query<EntityRef, With<Player>>,
    mut enemies: Query<&mut Enemy, Without<Player>>
) {
    // only iterates players
    for entity in &players {
        if let Some(mesh) = entity.get::<Handle<Mesh>>() {
            let transform = entity.get::<Transform>().unwrap();
        }
    }
}
```

Note that it will generally still be more ergonomic (and more efficient) to query for the components you want directly:

```rust
fn system(players: Query<(&Transform, &Handle<Mesh>), With<Player>>) {
    for (transform, mesh) in &players {
    }
}
```

[`EntityRef`]: https://docs.rs/bevy/0.11.0/bevy/ecs/world/struct.EntityRef.html
[`WorldQuery`]: https://docs.rs/bevy/0.11.0/bevy/ecs/query/trait.WorldQuery.html
[`World`]: https://docs.rs/bevy/0.11.0/bevy/ecs/world/struct.World.html

## Screenshot API

<div class="release-feature-authors">authors: @TheRawMeatball</div>

Bevy now has a simple screenshot API that can save a screenshot of a given window to the disk:

```rust
fn take_screenshot(
    mut screenshot_manager: ResMut<ScreenshotManager>,
    input: Res<Input<KeyCode>>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    if input.just_pressed(KeyCode::Space) {
        screenshot_manager
            .save_screenshot_to_disk(primary_window.single(), "screenshot.png")
            .unwrap();
    }
}
```

## RenderTarget::TextureView

<div class="release-feature-authors">authors: @mrchantey</div>

The [`Camera`] [`RenderTarget`] can now be set to a wgpu [`TextureView`]. This allows 3rd party Bevy Plugins to manage a [`Camera`]'s texture. One particularly interesting use case that this enables is XR/VR support. A few community members have already [proven this out!](https://github.com/bevyengine/bevy/issues/115#issuecomment-1436749201)

[`RenderTarget`]: https://docs.rs/bevy/0.11.0/bevy/render/camera/enum.RenderTarget.html
[`TextureView`]: https://docs.rs/bevy/0.11.0/bevy/render/render_resource/struct.TextureView.html

## Improved Text Wrapping

<div class="release-feature-authors">authors: @ickshonpe</div>

Previous versions of Bevy didn't properly wrap text because it calculated the actual text prior to calculating layout. **Bevy 0.11** adds a "text measurement step" that calculates the text size prior to layout, then computes the actual text _after_ layout.

![text wrap](text_wrap.png)

There is also a new `NoWrap` variant on the [`BreakLineOn`] setting, which can disable text wrapping entirely when that is desirable.

[`BreakLineOn`]: https://docs.rs/bevy/0.11.0/bevy/text/enum.BreakLineOn.html

## Faster UI Render Batching

<div class="release-feature-authors">authors: @ickshonpe</div>

We got a huge UI performance win for some cases by avoiding breaking up UI batches when the texture changes but the next node is untextured.

Here is a profile of our "many buttons" stress test. Red is before the optimization and Yellow is after:

![ui profile](ui_profile.png)

## Better Reflect Proxies

<div class="release-feature-authors">authors: @MrGVSV</div>

Bevy's reflection API has a handful of structs which are collectively known as "dynamic" types.
These include [`DynamicStruct`], [`DynamicTuple`], and more, and they are used to dynamically construct types
of any shape or form at runtime.
These types are also used to create are commonly referred to as "proxies", which are dynamic types
that are used to represent an actual concrete type.

These proxies are what powers the [`Reflect::clone_value`] method, which generates these proxies under the hood
in order to construct a runtime clone of the data.

Unfortunately, this results in a few [subtle footguns] that could catch users by surprise,
such as the hashes of proxies differing from the hashes of the concrete type they represent,
proxies not being considered equivalent to their concrete counterparts, and more.

While this release does not necessarily fix these issues, it does establish a solid foundation for fixing them in the future.
The way it does this is by changing how a proxy is defined.

Before 0.11, a proxy was only defined by cloning the concrete type's [`Reflect::type_name`] string
and returning it as its own `Reflect::type_name`.

Now in 0.11, a proxy is defined by copying a reference to the static [`TypeInfo`] of the concrete type.
This will allow us to access more of the concrete type's type information dynamically, without requiring the `TypeRegistry`.
In a [future release], we will make use of this to store hashing and comparison strategies in the `TypeInfo` directly
in order to mitigate the proxy issues mentioned above.

[`DynamicStruct`]: https://docs.rs/bevy/0.11.0/bevy/reflect/struct.DynamicStruct.html
[`DynamicTuple`]: https://docs.rs/bevy/0.11.0/bevy/reflect/struct.DynamicTuple.html
[`Reflect::clone_value`]: https://docs.rs/bevy/0.11.0/bevy/reflect/trait.Reflect.html#tymethod.clone_value
[subtle footguns]: https://github.com/bevyengine/bevy/issues/6601
[`Reflect::type_name`]: https://docs.rs/bevy/0.11.0/bevy/reflect/trait.Reflect.html#tymethod.type_name
[`TypeInfo`]: https://docs.rs/bevy/0.11.0/bevy/reflect/enum.TypeInfo.html
[future release]: https://github.com/bevyengine/bevy/pull/8695

## `FromReflect` Ergonomics

<div class="release-feature-authors">authors: @MrGVSV</div>

Bevy's [reflection API] commonly passes around data using type-erased `dyn Reflect` trait objects.
This can usually be downcast back to its concrete type using `<dyn Reflect>::downcast_ref::<T>`;
however, this doesn't work if the underlying data has been converted to a "dynamic" representation
(e.g. `DynamicStruct` for struct types, `DynamicList` for list types, etc.).

```rust
let data: Vec<i32> = vec![1, 2, 3];

let reflect: &dyn Reflect = &data;
let cloned: Box<dyn Reflect> = reflect.clone_value();

// `reflect` really is a `Vec<i32>`
assert!(reflect.is::<Vec<i32>>());
assert!(reflect.represents::<Vec<i32>>());

// `cloned` is a `DynamicList`, but represents a `Vec<i32>`
assert!(cloned.is::<DynamicList>());
assert!(cloned.represents::<Vec<i32>>());

// `cloned` is equivalent to the original `reflect`, despite not being a `Vec<i32>`
assert!(cloned.reflect_partial_eq(reflect).unwrap_or_default());
```

To account for this, the [`FromReflect`] trait can be used to convert any `dyn Reflect` trait object
back into its concrete type— whether it is actually that type or a dynamic representation of it.
And it can even be called dynamically using the [`ReflectFromReflect`] type data.

Before 0.11, users had to be manually derive `FromReflect` for every type that needed it,
as well as manually register the `ReflectFromReflect` type data.
This made it cumbersome to use and also meant that it was often forgotten about,
resulting in reflection conversions difficulties for users downstream.

Now in 0.11, `FromReflect` is automatically derived and `ReflectFromReflect` is automatically registered for all types that derive `Reflect`.
This means most types will be `FromReflect`-capable by default,
thus reducing boilerplate and empowering logic centered around `FromReflect`.

Users can still opt out of this behavior by adding the [`#[reflect(from_reflect = false)]`][from_reflect = false] attribute to their type.

```rust
#[derive(Reflect)]
struct Foo;

#[derive(Reflect)]
#[reflect(from_reflect = false)]
struct Bar;

fn test<T: FromReflect>(value: T) {}

test(Foo); // <-- OK!
test(Bar); // <-- ERROR! `Bar` does not implement trait `FromReflect`
```

[reflection API]: https://docs.rs/bevy_reflect/latest/bevy_reflect/index.html
[`FromReflect`]: https://docs.rs/bevy_reflect/latest/bevy_reflect/trait.FromReflect.html
[`ReflectFromReflect`]: https://docs.rs/bevy_reflect/latest/bevy_reflect/struct.ReflectFromReflect.html
[from_reflect = false]: https://docs.rs/bevy_reflect/latest/bevy_reflect/derive.Reflect.html#reflectfrom_reflect--false

## Deref Derive Attribute

<div class="release-feature-authors">authors: @MrGVSV</div>

Bevy code tends to make heavy use of the [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) pattern,
which is why we have dedicated derives for [`Deref`](https://docs.rs/bevy/latest/bevy/prelude/derive.Deref.html) and [`DerefMut`](https://docs.rs/bevy/latest/bevy/prelude/derive.DerefMut.html).

This previously only worked for structs with a single field:

```rust
#[derive(Resource, Deref, DerefMut)]
struct Score(i32);
```

For 0.11, we've improved these derives by adding the `#[deref]` attribute, which allows them to be used on structs with multiple fields.
This makes working with generic newtypes much easier:

```rust
#[derive(Component, Deref, DerefMut)]
struct Health<T: Character> {
    #[deref] // <- use the `health` field as the `Deref` and `DerefMut` target
    health: u16,
    _character_type: PhantomData<T>,
}
```

## Simpler RenderGraph Construction

<div class="release-feature-authors">authors: @IceSentry, @cart</div>

Adding `Node`s to the `RenderGraph` requires a lot of boilerplate. In this release, we tried to reduce this for most common operations. No existing APIs have been removed, these are only helpers made to simplify working with the `RenderGraph`.

We added the `RenderGraphApp` trait to the `App`. This trait contains various helper functions to reduce the boilerplate with adding nodes and edges to a graph.

Another pain point of `RenderGraph` `Node`s is passing the view entity through each node and manually updating the query on that view. To fix this we added a `ViewNode` trait and `ViewNodeRunner` that will automatically take care of running the `Query` on the view entity. We also made the view entity a first-class concept of the `RenderGraph`. So you can now access the view entity the graph is currently running on from anywhere in the graph without passing it around between each `Node`.

All these new APIs assume that your Node implements `FromWorld` or `Default`.

Here's what it looks like in practice for the `BloomNode`:

```rust
// Adding the node to the 3d graph
render_app
    // To run a ViewNode you need to create a ViewNodeRunner
    .add_render_graph_node::<ViewNodeRunner<BloomNode>>(
        CORE_3D,
        core_3d::graph::node::BLOOM,
    );

// Defining the node
#[derive(Default)]
struct BloomNode;
// This can replace your `impl Node` block of any existing `Node` that operated on a view
impl ViewNode for BloomNode {
    // You need to define your view query as an associated type
    type ViewQuery = (
        &'static ExtractedCamera,
        &'static ViewTarget,
        &'static BloomSettings,
    );
    // You don't need Node::input() or Node::update() anymore. If you still need these they are still available but they have an empty default implementation.
    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        // This is the result of your query. If it is empty the run function will not be called
        (camera, view_target, bloom_settings): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        // When using the ViewNode you probably won't need the view entity but here's how to get it if you do
        let view_entity = graph.view_entity();

        // Run the node
    }
}
```

## `#[reflect(default)]` on Enum Variant Fields

<div class="release-feature-authors">authors: @MrGVSV</div>

When using the `FromReflect` trait, fields marked `#[reflect(default)]` will be set to their `Default` value if they don't exist on the reflected object.

Previously, this was only supported on struct fields.
Now, it is also supported on all enum variant fields.

```rust
#[derive(Reflect)]
enum MyEnum {
    Data {
        #[reflect(default)]
        a: u32,
        b: u32,
    },
}

let mut data = DynamicStruct::default ();
data.insert("b", 1);

let dynamic_enum = DynamicEnum::new("Data", data);

let my_enum = MyEnum::from_reflect( & dynamic_enum).unwrap();
assert_eq!(u32::default(), my_enum.a);
```

## Delayed Asset Hot Reloading

<div class="release-feature-authors">authors: @JMS55</div>

Bevy now waits 50 milliseconds after an "asset changed on filesystem" event before reloading an asset. Reloading without a delay resulted in reading invalid asset contents on some systems. The wait time is configurable.

## Custom glTF Vertex Attributes

<div class="release-feature-authors">authors: @komadori</div>

It is now possible to load meshes with custom vertex attributes from glTF files. Custom attributes can be mapped to Bevy's [`MeshVertexAttribute`] format used by the [`Mesh`] type in the [`GltfPlugin`] settings. These attrtibutes can then be used in Bevy shaders. For an example, check out our [new example](https://github.com/bevyengine/bevy/blob/v0.11.0/examples/2d/custom_gltf_vertex_attribute.rs).

![custom vertex attribute](custom_vertex.png)

[`MeshVertexAttribute`]: https://docs.rs/bevy/0.11.0/bevy/render/mesh/struct.MeshVertexAttribute.html
[`Mesh`]: https://docs.rs/bevy/0.11.0/bevy/render/mesh/struct.Mesh.html
[`GltfPlugin`]: https://docs.rs/bevy/0.11.0/bevy/gltf/struct.GltfPlugin.html

## Stable TypePath

<div class="release-feature-authors">authors: @soqb, @tguichaoua</div>

Bevy has historically used [`std::any::type_name`][`type_name`] to identify Rust types with friendly names in a number of places: Bevy Reflect, Bevy Scenes, Bevy Assets, Bevy ECS, and others. Unfortunately, Rust makes no guarantees about the stability or format of [`type_name`], which makes it theoretically shakey ground to build on (although in practice it has been stable so far).

There is also no built in way to retrieve "parts" of a type name. If you want the short name, the name of a generic type without its inner types, the module name, or the crate name, you must do string operations on the [`type_name`] (which can be error prone / nontrivial).

Additionally, [`type_name`] cannot be customized. In some cases an author might choose to identify a type with something other than its full module path (ex: if they prefer a shorter path or want to abstract out private / internal modules).

For these reasons, we've developed a new stable [`TypePath`], which is automatically implemented for any type deriving [`Reflect`]. Additionally, it can be manually derived in cases where [`Reflect`] isn't derived.

```rust
mod my_mod {
    #[derive(Reflect)]
    struct MyType;
}

/// prints: "my_crate::my_mod::MyType"
println!("{}", MyType::type_path());
/// prints: "MyType"
println!("{}", MyType::short_type_path());
/// prints: "my_crate"
println!("{}", MyType::crate_name().unwrap());
/// prints: "my_crate::my_mod"
println!("{}", MyType::module_path().unwrap());
```

This also works for generics, which can come in handy:

```rust
// prints: "Option<MyType>"
println!("{}", Option::<MyType>::short_type_path());
// prints: "Option"
println!("{}", Option::<MyType>::type_ident().unwrap());
```

[`TypePath`] can be customized by type authors:

```rust
#[derive(TypePath)]
#[type_path = "some_crate::some_module"]
struct MyType;
```

We are in the process of porting Bevy's internal [`type_name`] usage over to [`TypePath`], which should land in **Bevy 0.12**.

[`type_name`]: https://doc.rust-lang.org/std/any/fn.type_name.html
[`TypePath`]: https://docs.rs/bevy/0.11.0/bevy/reflect/trait.TypePath.html
[`Reflect`]: https://docs.rs/bevy/0.11.0/bevy/reflect/trait.Reflect.html

## `run_if` for Tuples of Systems

<div class="release-feature-authors">authors: @geieredgar</div>

It is now possible to add ["run conditions"](/news/bevy-0-10/#run-conditions) to tuples of systems:

```rust
app.add_systems(Update, (run, jump).run_if(in_state(GameState::Playing)))
```

This will evaluate the "run condition" exactly once and use the result for each system in the tuple.

This allowed us to remove the `OnUpdate` system set for states (which was previously used to run groups of systems when they are in a given state).

## `Has` Queries

<div class="release-feature-authors">authors: @wainwrightmark</div>

You can now use `Has<Component>` in queries, which will return true if that component exists and false if it does not:

```rust
fn system(query: Query<Has<Player>>) {
    for has_player in &query {
        if has_player {
            // do something
        }
    }
}
```

## Derive `Event`

<div class="release-feature-authors">authors: @CatThingy</div>

The Bevy [`Event`] trait is now derived instead of being auto-impled for everything:

```rust
#[derive(Event)]
struct Collision {
    a: Entity,
    b: Entity,
}
```

This prevents some classes of error, makes [`Event`] types more self-documenting, and provides consistency with other Bevy ECS traits like Components and Resources. It also opens the doors to configuring the [`Event`] storage type, which we plan to do in future releases.

[`Event`]: https://docs.rs/bevy/0.11.0/bevy/ecs/event/trait.Event.html

## Cubic Curve Example

<div class="release-feature-authors">authors: @Kjolnyr</div>

An example that shows how to draw a 3D curve and move an object along the path:

![cubic_curve](cubic_curve.png)

## Size Constraints Example

<div class="release-feature-authors">authors: @ickshonpe</div>

An interactive example that shows how the various [`Style`] size constraints affect UI nodes.

![size constraints](size_constraints.png)

## Display and Visibility Example

<div class="release-feature-authors">authors: @ickshonpe</div>

An example that shows how display and visibility settings affect UI nodes.

![display and visibility](display_and_visibility.png)

## No More Bors!

<div class="release-feature-authors">authors: @cart, @mockersf</div>

Bevy has historically used the Bors merge system to ensure we never merge a pull request on GitHub that breaks our CI validation. This was a critical piece of infrastructure that ensured we could collaborate safely and effectively. Fortunately GitHub has _finally_ rolled out [Merge Queues](https://github.blog/changelog/2023-02-08-pull-request-merge-queue-public-beta/), which solve the same problems as Bors, with the benefit of being more tightly integrated with GitHub.

For this release cycle we migrated to Merge Queues and we're very happy with the experience!

## New CI Jobs

<div class="release-feature-authors">authors: @mockersf</div>

We've added a number of new CI jobs that improve the Bevy development experience:

* A daily job that runs Bevy's mobile examples on real Android and iOS devices! This helps protect against regressions that might not be caught by the compiler
* Added the ability to take screenshots in CI, which can be used to validate the results of Bevy example runs
* A job that leaves a GitHub comment on PRs that are missing a feature or example doc update

## <a name="what-s-next"></a>What's Next?

We have plenty of work that is pretty much finished and is therefore very likely to land in **Bevy 0.12**:

* **Bevy Asset V2**: A brand new asset system that adds "asset preprocessing", optional asset .meta files, recursive asset dependency tracking and events, async asset IO, better asset handles, more efficient asset storage, and a variety of usability improvements! The work here is [pretty much finished](https://github.com/bevyengine/bevy/pull/8624). It _almost_ made it in to Bevy 0.11 but it needed a bit more time to cook.
* **PBR Material Light Transmission**: Transmission / screen space refractions allows for simulating materials like glass, plastics, liquids and gels, gemstones, wax, etc. This one is also pretty much [ready to go](https://github.com/bevyengine/bevy/pull/8015)!
* **TAA Improvements**: We have a number of changes in the works for TAA that will improve its quality, speed, and support within the engine.
* **GPU Picking**: Efficiently and correctly [select entities on the GPU](https://github.com/bevyengine/bevy/pull/8784) by using color ids to identify meshes in renders.
* **PCF For Directional and Spotlight Shadows**: [Reduce aliasing on the edges of shadows](https://github.com/bevyengine/bevy/pull/8006)
* **UI Node Border Radius and Shadows**: Add [curvature and "drop shadows"](https://github.com/bevyengine/bevy/pull/8973) to your UI nodes!
* **Deferred Rendering**: Bevy already does "mixed mode" forward rendering by having optional separate passes for depth and normals. We are currently experimenting with supporting "fully deferred" rendering as well, which opens the doors to new effects and different performance tradeoffs.

From a high level, we plan to focus on the Asset System, UI, Render Features, and Scenes during the next cycle.

Check out the [**Bevy 0.12 Milestone**](https://github.com/bevyengine/bevy/milestone/14) for an up-to-date list of current work being considered for **Bevy 0.12**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the 166 contributors that made this release (and associated docs) possible! In random order:

* @TheBlek
* @hank
* @TimJentzsch
* @Suficio
* @SmartManLudo
* @BlondeBurrito
* @lewiszlw
* @paul-hansen
* @boringsan
* @superdump
* @JonahPlusPlus
* @airingursb
* @Sheepyhead
* @nakedible
* @Testare
* @andresovela
* @SkiFire13
* @doup
* @BlackPhlox
* @nicoburns
* @wpederzoli
* @adtennant
* @LoopyAshy
* @KernelFreeze
* @ickshonpe
* @jim-ec
* @mrchantey
* @frengor
* @Joakker
* @arendjr
* @MJohnson459
* @TheTacBanana
* @IceSentry
* @ItsDoot
* @Anti-Alias
* @mwbryant
* @inodentry
* @LiamGallagher737
* @robtfm
* @mockersf
* @ndarilek
* @samtenna
* @Estus-Dev
* @InnocentusLime
* @p-hueber
* @B-Reif
* @Adamkob12
* @payload
* @JohnTheCoolingFan
* @djeedai
* @SludgePhD
* @s-lambert
* @kjolnyr
* @Skovrup1
* @Ababwa
* @Illiux
* @Carter0
* @luca-della-vedova
* @Neo-Zhixing
* @coreh
* @helvieq499
* @Carbonhell
* @BrandonDyer64
* @hymm
* @JMS55
* @iiYese
* @mtsr
* @jannik4
* @natasria
* @Trouv
* @minchopaskal
* @chrisjuchem
* @marlyx
* @valaphee
* @hankjordan
* @rparrett
* @Selene-Amanita
* @opstic
* @loganbenjamin
* @MrGunflame
* @pyrotechnick
* @mjhostet
* @VitalyAnkh
* @CatThingy
* @maniwani
* @Themayu
* @SET001
* @jakobhellermann
* @MrGVSV
* @nicopap
* @Wcubed
* @aevyrie
* @NiklasEi
* @bonsairobo
* @cart
* @TotalKrill
* @raffaeleragni
* @Aceeri
* @Shatur
* @orzogc
* @UncleScientist
* @Elabajaba
* @vyb
* @komadori
* @jnhyatt
* @harudagondi
* @konsti219
* @james7132
* @mvlabat
* @neithanmo
* @dgunay
* @Shfty
* @hate
* @B-head
* @MinerSebas
* @chescock
* @BorMor
* @lupan
* @CrazyRoka
* @bzm3r
* @Sixmorphugus
* @JoJoJet
* @eltociear
* @gakit
* @geieredgar
* @tjamaan
* @alice-i-cecile
* @NoahShomette
* @james-j-obrien
* @tinrab
* @Olle-Lukowski
* @TheRawMeatball
* @sarkahn
* @RobWalt
* @johanhelsing
* @SneakyBerry
* @beeryt
* @Vrixyz
* @wainwrightmark
* @EliasPrescott
* @konsolas
* @ameknite
* @Connor-McMillin
* @Weibye
* @SpecificProtagonist
* @danchia
* @vallrand
* @atornity
* @soqb
* @devil-ira
* @AnthonyKalaitzis
* @yyogo
* @NiseVoid
* @gajop
* @Gingeh
* @zendril
* @ezekg
* @ickk
* @Leonss23
* @kellencataldo
* @akappel
* @hazelsparrow
* @mattfbacon
* @gianzellweger
* @lakrsv
* @laundmo

## Full Changelog

### Rendering

* [Webgpu support][8336]
* [improve shader import model][5703]
* [Screen Space Ambient Occlusion (SSAO) MVP][7402]
* [Temporal Antialiasing (TAA)][7291]
* [Immediate Mode Line/Gizmo Drawing][6529]
* [Make render graph slots optional for most cases][8109]
* [Split opaque and transparent phases][8090]
* [Built-in skybox][8275]
* [Add parallax mapping to bevy PBR][5928]
* [Add port of AMD's Robust Contrast Adaptive Sharpening][7422]
* [Add RenderGraphApp to simplify adding render nodes][8007]
* [Add screenshot API][7163]
* [Add morph targets][8158]
* [Screenshots in wasm][8455]
* [Add ViewNode to simplify render node management][8118]
* [Bias texture mipmaps][7614]
* [Instanced line rendering for gizmos based on `bevy_polyline`][8427]
* [Add `RenderTarget::TextureView`][8042]
* [Change default tonemapping method][8685]
* [Allow custom depth texture usage][6815]
* [Use the prepass normal texture in main pass when possible][8231]
* [Left-handed y-up cubemap coordinates][8122]
* [Allow SPIR-V shaders to process when shader defs are present][7772]
* [Remove unnecesssary values Vec from DynamicUniformBuffer and DynamicStorageBuffer][8299]
* [Add `MAY_DISCARD` shader def, enabling early depth tests for most cases][6697]
* [Add `Aabb` calculation for `Sprite`, `TextureAtlasSprite` and `Mesh2d`][7885]
* [Color::Lcha constructors][8041]
* [Fix Color::as_rgba_linear for Color::Lcha][8040]
* [Added Globals struct to prepass shader][8070]
* [Derive Copy and Clone for Collision][8121]
* [Fix crash when enabling HDR on 2d cameras][8151]
* [Dither fix][7977]
* [Compute `vertex_count` for indexed meshes on `GpuMesh`][8460]
* [Run update_previous_view_projections in PreUpdate schedule][9024]
* [Added `WebP` image format support][8220]
* [Add support for pnm textures][8601]
* [fix invalid bone weights][8316]
* [Fix pbr shader breaking on missing UVs][8412]
* [Fix Plane UVs / texture flip][8878]
* [Fix look_to resulting in NaN rotations][7817]
* [Fix look_to variable naming][8627]
* [Fix segfault with 2d gizmos][8223]
* [Use RenderGraphApp in more places][8298]
* [Fix viewport change detection][8323]
* [Remove capacity fields from all Buffer wrapper types][8301]
* [Sync pbr_types.wgsl StandardMaterial values][8380]
* [Avoid spawning gizmo meshes when no gizmos are being drawn][8180]
* [Use a consistent seed for AABB gizmo colors][9030]
* [bevy_pbr: Do not cull meshes without Aabbs from cascades][8444]
* [Handle vertex_uvs if they are present in default prepass fragment shader][8330]
* [Changed (Vec2, Vec2) to Rect in Camera::logical_viewport_rect][7867]
* [make glsl and spirv support optional][8491]
* [fix prepass normal_mapping][8978]
* [conversions between [u8; 4] and Color][8564]
* [Add option to disable gizmo rendering for specific cameras][8952]
* [Fix morph target prepass shader][9013]
* [Fix bloom wasm support][8631]
* [Fix black spots appearing due to NANs when SSAO is enabled][8926]
* [fix normal prepass][8890]
* [Refs #8975 -- Add return to RenderDevice::poll()][8977]
* [Fix WebGL mode for Adreno GPUs][8508]
* [Fix parallax mapping][9003]
* [Added Vec append to BufferVec - Issue #3531][8575]
* [Fix CAS shader with explicit FullscreenVertexOutput import][8993]
* [Make `TextureAtlas::texture_handles` `pub` instead of `pub(crate)` (#8633)][8643]
* [Make Material2d pipeline systems public][8642]
* [Fix screenshots on Wayland + Nvidia][8701]
* [Apply codebase changes in preparation for `StandardMaterial` transmission][8704]
* [Use ViewNode for TAA][8732]
* [Change Camera3dBundle::tonemapping to Default][8753]
* [Remove `Component` derive for AlphaMode][8804]
* [Make setup of Opaque3dPrepass and AlphaMask3dPrepass phase items consistent with others][8408]
* [Rename `Plane` struct to `HalfSpace`][8744]
* [Expand `FallbackImage` to include a `GpuImage` for each possible `TextureViewDimension`][6974]
* [Cascaded shadow maps: Fix prepass ortho depth clamping][8877]
* [Fix gizmos in WebGPU][8910]
* [Fix AsBindGroup derive, texture attribute, visibility flag parsing][8868]
* [Disable camera on window close][8802]
* [Reflect `Component` and `Default` of `BloomSettings`][8283]
* [Add Reflection Macros to TextureAtlasSprite][8428]
* [Implement Reflect on NoFrustumCulling][8801]

### Audio

* [ECS-based API redesign][8424]
* [Ability to set a Global Volume][7706]
* [Expose `AudioSink::empty()`][8145]

### Diagnostics

* [Allow systems using Diagnostics to run in parallel][8677]
* [add a feature for memory tracing with tracy][8272]
* [Re-add the "frame" span for tracy comparisons][8362]
* [log to stderr instead of stdout][8886]

### Scenes

* [bevy_scene: Add SceneFilter][6793]
* [(De) serialize resources in scenes][6846]
* [add position to scene errors][8065]
* [Bugfix: Scene reload fix (nonbreaking)][7951]
* [avoid panic with parented scenes on deleted entities][8512]

### Transform + Hierarchy

* [Fix transform propagation of orphaned entities][7264]

### Gizmo

* [Add a bounding box gizmo][8468]
* [Added `arc_2d` function for gizmos][8448]
* [Use AHash to get color from entity in bevy_gizmos][8960]
* [do not crash when rendering only one gizmo][8434]

### Reflection

* [reflect: stable type path v2][7184]
* [bevy_reflect: Better proxies][6971]
* [bevy_reflect: FromReflect Ergonomics Implementation][6056]
* [bevy_reflect: Allow `#[reflect(default)]` on enum variant fields][8514]
* [Add FromReflect where Reflect is used][8776]
* [Add get_at_mut to bevy_reflect::Map trait][8691]
* [Reflect now requires DynamicTypePath. Remove Reflect::get_type_path()][8764]
* [bevy_ui: Add `FromReflect` derives][8495]
* [Add Reflect and FromReflect for AssetPath][8531]
* [bevy_reflect: Fix trailing comma breaking derives][8014]
* [Fix Box<dyn Reflect> struct with a hashmap in it panicking when clone_value is called on it][8184]
* [bevy_reflect: Add `ReflectFromReflect` to the prelude][8496]
* [bevy_reflect: Allow construction of MapIter outside of the bevy_reflect crate.][8723]
* [bevy_reflect: Disambiguate type bounds in where clauses.][8761]
* [adding reflection for Cow<'static, [T]>][7454]
* [Do not require mut on ParsedPath::element_mut][8891]
* [Reflect UUID][8905]
* [Don't ignore additional entries in `UntypedReflectDeserializerVisitor`][7112]
* [Construct Box<dyn Reflect> from world for ReflectComponent][7407]
* [reflect: avoid deadlock in GenericTypeCell][8957]

### App

* [Allow tuples and single plugins in `add_plugins`, deprecate `add_plugin`][8097]
* [Merge ScheduleRunnerSettings into ScheduleRunnerPlugin][8585]
* [correctly setup everything in the default run_once runner][8740]
* [Fix `Plugin::build` detection][8103]
* [Fix not calling App::finish and App::cleanup in `ScheduleRunnerPlugin`][9054]
* [Relaxed runner type from Fn to FnOnce][8961]
* [Relax FnMut to FnOnce in app::edit_schedule][8982]

### Windowing + Reflection

* [Register missing types in bevy_window][7993]
* [bevy_reflect: implement Reflect for SmolStr][8771]

### Hierarchy

* [fix panic when moving child][8346]
* [Remove `Children` component when calling `despawn_descendants`][8476]
* [Change `despawn_descendants` to return `&mut Self`][8928]

### Time

* [Fix timer with zero duration][8467]

### Assets

* [Delay asset hot reloading][8503]
* [Add support for custom glTF vertex attributes.][5370]
* [Fix panic when using debug_asset_server][8485]
* [`unused_variables` warning when building with `filesystem_watcher` feature disabled][7938]
* [bevy_asset: Add `LoadContext::get_handle_untyped`][8470]

### Windowing

* [Move cursor position to internal state][7988]
* [Set cursor hittest during window creation][7966]
* [do not set hit test unconditionally on window creation][7996]
* [Add winit's `wayland-csd-adwaita` feature to Bevy's `wayland` feature][8722]
* [Support to set window theme and expose system window theme changed event][8593]
* [Touchpad magnify and rotate events][8791]
* [Fix windows not being centered properly when system interface is scaled][8903]
* [Expose WindowDestroyed events][9016]

### Animation

* [Register bevy_animation::PlayingAnimation][9023]

### UI

* [Ui Node Borders][7795]
* [Add CSS Grid support to `bevy_ui`][8026]
* [`text_system` split][7779]
* [Replace the local text queues in the text systems with flags stored in a component][8549]
* [`NoWrap` `Text` feature][8947]
* [add a default font][8445]
* [UI texture atlas support][8822]
* [Improved UI render batching][8793]
* [Consistent screen-space coordinates][8306]
* [`UiImage` helper functions][8199]
* [Perform text scaling calculations per text, not per glyph][7819]
* [Fix size of clipped text glyphs.][8197]
* [Apply scale factor to  `ImageMeasure` sizes][8545]
* [Fix WebGPU error in "ui_pipeline" by adding a flat interpolate attribute][8933]
* [Rename Interaction::Clicked -> Interaction::Pressed][9027]
* [Flatten UI `Style` properties that use `Size` + remove `Size`][8548]
* [Split UI `Overflow` by axis][8095]
* [Add methods for calculating the size and postion of UI nodes][7930]
* [Skip the UV calculations for untextured UI nodes][7809]
* [Fix text measurement algorithm][8425]
* [Divide by UiScale when converting UI coordinates from physical to logical][8720]
* [`MeasureFunc` improvements][8402]
* [Expose sorting methods in `Children`][8522]
* [Fix min and max size using size value][7948]
* [Fix the `Text2d` text anchor's incorrect horizontal alignment][8019]
* [Remove `Val::Undefined`][7485]
* [`Val` viewport unit variants][8137]
* [Remove the corresponding measure from Taffy when a `CalculatedSize` component is removed.][8294]
* [`UiRect` axes constructor][7656]
* [Fix the UV calculations for clipped and flipped ImageNodes][8195]
* [Fix text systems broken when resolving merge conflicts in #8026][8422]
* [Allow `bevy_ui` crate to compile without the `text` feature enabled][8437]
* [Fix the double leaf node updates in `flex_node_system`][8264]
* [also import the default handle when feature disabled][8456]
* [`measure_text_system` text query fix][8466]
* [Fix panic in example: text_wrap_debug.rs][8497]
* [UI layout tree debug print][8521]
* [Fix `Node::physical_rect` and add a `physical_size` method][8551]
* [Perform `relative_cursor_position` calculation vectorwise in `ui_focus_system`][8795]
* [Add `UiRect::px()` and `UiRect::percent()` utils][8866]
* [Add missing dependencies to `bevy_text` feature][8920]
* [Remove "bevy_text" feature attributes on imports used by non-text systems][8907]
* [Growing UI nodes Fix][8931]

### ECS

* [Schedule-First: the new and improved add_systems][8079]
* [Add OnTransition schedule that is ran between OnExit and OnEnter][7936]
* [`run_if` for `SystemConfigs` via anonymous system sets][7676]
* [Remove OnUpdate system set][8260]
* [Rename apply_system_buffers to apply_deferred][8726]
* [Rename Command's "write" method to "apply"][8814]
* [Require `#[derive(Event)]` on all Events][7086]
* [Implement WorldQuery for EntityRef][6960]
* [Improve or-with disjoint checks][7085]
* [Add a method to run read-only systems using `&World`][8849]
* [Reduce branching when inserting components][8053]
* [Make `#[system_param(ignore)]` and `#[world_query(ignore)]` unnecessary][8030]
* [Remove `#[system_param(ignore)]` and `#[world_query(ignore)]`][8265]
* [Extend the `WorldQuery` macro to tuple structs][8119]
* [Make state private and only accessible through getter for State resource][8009]
* [implement `Deref` for `State<S>`][8668]
* [Inline more ECS functions][8083]
* [Add a `scope` API for world schedules][8387]
* [Simplify system piping and make it more flexible][8377]
* [Add `any_component_removed` condition][8326]
* [Use `UnsafeWorldCell` to increase code quality for `SystemParam`][8174]
* [Improve safety for the multi-threaded executor using `UnsafeWorldCell`][8292]
* [Migrate the rest of the engine to `UnsafeWorldCell`][8833]
* [Make the `Condition` trait generic][8721]
* [Add or_else combinator to run_conditions.rs][8714]
* [Add iter_many_manual QueryState method][8772]
* [Provide access to world storages via UnsafeWorldCell][8987]
* [Added Has<T> WorldQuery type][8844]
* [Add/fix `track_caller` attribute on panicking entity accessor methods][8951]
* [Increase type safety and clarity for change detection][7905]
* [Make `WorldQuery` meta types unnameable][7964]
* [Add a public constructor for `Mut<T>`][7931]
* [Remove ChangeTrackers][7902]
* [Derive Eq, PartialEq for Tick][9020]
* [Initialize empty schedules when calling `.in_schedule` if they do not already exist][7911]
* [Replace multiple calls to `add_system` with `add_systems`][8001]
* [don't panic on unknown ambiguity][7950]
* [add Clone to common conditions][8060]
* [Make BundleInfo's fields not pub(crate)][8068]
* [Pass query change ticks to `QueryParIter` instead of always using change ticks from `World`.][8029]
* [Remove redundant bounds check in `Entities::get`][8108]
* [Add World::try_run_schedule][8028]
* [change not implemation to custom system struct][8105]
* [Fix name conflicts caused by the `SystemParam` and `WorldQuery` macros][8012]
* [Check for conflicting accesses in `assert_is_system`][8154]
* [Fix field visibility for read-only `WorldQuery` types][8163]
* [`Or<T>` should be a new type of `PhantomData<T>`][8212]
* [Make standard commands more ergonomic (in niche cases)][8249]
* [Remove base set error variants of `ScheduleBuildError`][8269]
* [Replace some unsafe system executor code with safe code][8274]
* [Update `increment_change_tick` to return a strongly-typed `Tick`][8295]
* [Move event traces to detailed_trace!][7732]
* [Only trigger state transitons if `next_state != old_state`][8359]
* [Fix panics and docs when using World schedules][8364]
* [Improve warning for Send resources marked as non_send][8000]
* [Reorganize system modules][8419]
* [Fix boxed labels][8436]
* [Simplify world schedule methods][8403]
* [Just print out name string, not the entire Name struct][8494]
* [Manually implement common traits for `EventId`][8529]
* [Replace remaining uses of `&T, Changed<T>` with `Ref` in UI system queries][8567]
* [Rename `UnsafeWorldCell::read_change_tick`][8588]
* [Improve encapsulation for commands and add docs][8725]
* [Fix all_tuples + added docs.][8743]
* [Add `new` and `map` methods to `Ref`][8797]
* [Allow unsized types as mapped value in `Ref::map`][8817]
* [Implement `Clone` for `CombinatorSystem`][8826]
* [Add get_ref to EntityRef][8818]
* [Make `QueryParIter::for_each_unchecked` private][8848]
* [Simplify the `ComponentIdFor` type][8845]
* [Add last_changed_tick and added_tick to ComponentTicks][8803]
* [Require read-only queries in `QueryState::par_iter`][8832]
* [Fix any_component_removed][8939]
* [Deprecate type aliases for `WorldQuery::Fetch`][8843]
* [bevy_ecs: add untyped methods for inserting components and bundles][7204]
* [Move AppTypeRegistry to bevy_ecs][8901]
* [skip check change tick for apply_deferred systems][8760]
* [Split the bevy_ecs reflect.rs module][8834]
* [Make function pointers of ecs Reflect* public][8687]

### Rendering + Reflection + Scenes

* [fix: register Cascade in the TypeRegistry][8088]

### Tasks

* [Add optional single-threaded feature to bevy_ecs/bevy_tasks][6690]

### Math

* [Re-export glam_assert feature][8232]
* [Fix CubicCurve::iter_samples iteration count][8049]
* [Add integer equivalents for `Rect`][7984]
* [Add `CubicCurve::segment_count` + `iter_samples` adjustment][8711]

### Rendering + Assets + Meta

* [Add depending bevy features for higher level one][7855]

### ECS + Scenes

* [Make scene handling of entity references robust][7335]
* [Rename map_entities and map_specific_entities][7570]

### Util

* [bevy_derive: Add `#[deref]` attribute][8552]

### Input

* [Add gamepad rumble support to bevy_input][8398]
* [Rename keys like `LAlt` to `AltLeft`][8792]
* [Add window entity to mouse and keyboard events][8852]
* [Add get_unclamped to Axis][8871]

### Upgrades

* [Upgrade Taffy requirement to v0.3.5][7959]
* [Update ruzstd and basis universal][8622]
* [Updated to wgpu 0.16.0, wgpu-hal 0.16.0 and naga 0.12.0][8446]
* [Update sysinfo requirement from 0.28.1 to 0.29.0][8650]
* [Update libloading requirement from 0.7 to 0.8][8649]
* [update syn, encase, glam and hexasphere][8573]
* [Update android_log-sys requirement from 0.2.0 to 0.3.0][7925]
* [update bitflags to 2.3][8728]
* [Update ruzstd requirement from 0.3.1 to 0.4.0][8755]
* [Update notify requirement from 5.0.0 to 6.0.0][8757]
* [Bump hashbrown to 0.14][8904]
* [update ahash and hashbrown][8623]
* [Bump accesskit and accesskit_winit][8655]

### Examples

* [new example showcase tool][8561]
* [Adding a bezier curve example][8194]
* [Add low level post process example using a custom render pass][6909]
* [Add example to demonstrate manual generation and UV mapping of 3D mesh (generate_custom_mesh) solve #4922][8909]
* [Add `overflow_debug` example][8198]
* [UI text wrapping and `LineBreakOn` example][7761]
* [Size Constraints Example][7956]
* [UI Display and Visibility Example][7629]

[5370]: https://github.com/bevyengine/bevy/pull/5370
[5703]: https://github.com/bevyengine/bevy/pull/5703
[5928]: https://github.com/bevyengine/bevy/pull/5928
[6529]: https://github.com/bevyengine/bevy/pull/6529
[6697]: https://github.com/bevyengine/bevy/pull/6697
[6815]: https://github.com/bevyengine/bevy/pull/6815
[6846]: https://github.com/bevyengine/bevy/pull/6846
[6909]: https://github.com/bevyengine/bevy/pull/6909
[6960]: https://github.com/bevyengine/bevy/pull/6960
[6971]: https://github.com/bevyengine/bevy/pull/6971
[6974]: https://github.com/bevyengine/bevy/pull/6974
[7085]: https://github.com/bevyengine/bevy/pull/7085
[7086]: https://github.com/bevyengine/bevy/pull/7086
[7112]: https://github.com/bevyengine/bevy/pull/7112
[7163]: https://github.com/bevyengine/bevy/pull/7163
[7184]: https://github.com/bevyengine/bevy/pull/7184
[7204]: https://github.com/bevyengine/bevy/pull/7204
[7264]: https://github.com/bevyengine/bevy/pull/7264
[7291]: https://github.com/bevyengine/bevy/pull/7291
[7335]: https://github.com/bevyengine/bevy/pull/7335
[7402]: https://github.com/bevyengine/bevy/pull/7402
[7407]: https://github.com/bevyengine/bevy/pull/7407
[7422]: https://github.com/bevyengine/bevy/pull/7422
[7454]: https://github.com/bevyengine/bevy/pull/7454
[7485]: https://github.com/bevyengine/bevy/pull/7485
[7570]: https://github.com/bevyengine/bevy/pull/7570
[7614]: https://github.com/bevyengine/bevy/pull/7614
[7629]: https://github.com/bevyengine/bevy/pull/7629
[7656]: https://github.com/bevyengine/bevy/pull/7656
[7676]: https://github.com/bevyengine/bevy/pull/7676
[7706]: https://github.com/bevyengine/bevy/pull/7706
[7732]: https://github.com/bevyengine/bevy/pull/7732
[7761]: https://github.com/bevyengine/bevy/pull/7761
[7772]: https://github.com/bevyengine/bevy/pull/7772
[7779]: https://github.com/bevyengine/bevy/pull/7779
[7795]: https://github.com/bevyengine/bevy/pull/7795
[7809]: https://github.com/bevyengine/bevy/pull/7809
[7817]: https://github.com/bevyengine/bevy/pull/7817
[7819]: https://github.com/bevyengine/bevy/pull/7819
[7855]: https://github.com/bevyengine/bevy/pull/7855
[7867]: https://github.com/bevyengine/bevy/pull/7867
[7885]: https://github.com/bevyengine/bevy/pull/7885
[7902]: https://github.com/bevyengine/bevy/pull/7902
[7905]: https://github.com/bevyengine/bevy/pull/7905
[7911]: https://github.com/bevyengine/bevy/pull/7911
[7925]: https://github.com/bevyengine/bevy/pull/7925
[7930]: https://github.com/bevyengine/bevy/pull/7930
[7931]: https://github.com/bevyengine/bevy/pull/7931
[7936]: https://github.com/bevyengine/bevy/pull/7936
[7938]: https://github.com/bevyengine/bevy/pull/7938
[7948]: https://github.com/bevyengine/bevy/pull/7948
[7950]: https://github.com/bevyengine/bevy/pull/7950
[7951]: https://github.com/bevyengine/bevy/pull/7951
[7956]: https://github.com/bevyengine/bevy/pull/7956
[7959]: https://github.com/bevyengine/bevy/pull/7959
[7964]: https://github.com/bevyengine/bevy/pull/7964
[7966]: https://github.com/bevyengine/bevy/pull/7966
[7977]: https://github.com/bevyengine/bevy/pull/7977
[7984]: https://github.com/bevyengine/bevy/pull/7984
[7988]: https://github.com/bevyengine/bevy/pull/7988
[7993]: https://github.com/bevyengine/bevy/pull/7993
[7996]: https://github.com/bevyengine/bevy/pull/7996
[8000]: https://github.com/bevyengine/bevy/pull/8000
[8001]: https://github.com/bevyengine/bevy/pull/8001
[8007]: https://github.com/bevyengine/bevy/pull/8007
[8009]: https://github.com/bevyengine/bevy/pull/8009
[8012]: https://github.com/bevyengine/bevy/pull/8012
[8014]: https://github.com/bevyengine/bevy/pull/8014
[8019]: https://github.com/bevyengine/bevy/pull/8019
[8026]: https://github.com/bevyengine/bevy/pull/8026
[8028]: https://github.com/bevyengine/bevy/pull/8028
[8029]: https://github.com/bevyengine/bevy/pull/8029
[8030]: https://github.com/bevyengine/bevy/pull/8030
[8040]: https://github.com/bevyengine/bevy/pull/8040
[8041]: https://github.com/bevyengine/bevy/pull/8041
[8042]: https://github.com/bevyengine/bevy/pull/8042
[8049]: https://github.com/bevyengine/bevy/pull/8049
[8053]: https://github.com/bevyengine/bevy/pull/8053
[8060]: https://github.com/bevyengine/bevy/pull/8060
[8065]: https://github.com/bevyengine/bevy/pull/8065
[8068]: https://github.com/bevyengine/bevy/pull/8068
[8070]: https://github.com/bevyengine/bevy/pull/8070
[8079]: https://github.com/bevyengine/bevy/pull/8079
[8083]: https://github.com/bevyengine/bevy/pull/8083
[8088]: https://github.com/bevyengine/bevy/pull/8088
[8090]: https://github.com/bevyengine/bevy/pull/8090
[8095]: https://github.com/bevyengine/bevy/pull/8095
[8097]: https://github.com/bevyengine/bevy/pull/8097
[8103]: https://github.com/bevyengine/bevy/pull/8103
[8105]: https://github.com/bevyengine/bevy/pull/8105
[8108]: https://github.com/bevyengine/bevy/pull/8108
[8109]: https://github.com/bevyengine/bevy/pull/8109
[8118]: https://github.com/bevyengine/bevy/pull/8118
[8119]: https://github.com/bevyengine/bevy/pull/8119
[8121]: https://github.com/bevyengine/bevy/pull/8121
[8122]: https://github.com/bevyengine/bevy/pull/8122
[8137]: https://github.com/bevyengine/bevy/pull/8137
[8145]: https://github.com/bevyengine/bevy/pull/8145
[8151]: https://github.com/bevyengine/bevy/pull/8151
[8154]: https://github.com/bevyengine/bevy/pull/8154
[8158]: https://github.com/bevyengine/bevy/pull/8158
[8163]: https://github.com/bevyengine/bevy/pull/8163
[8174]: https://github.com/bevyengine/bevy/pull/8174
[8180]: https://github.com/bevyengine/bevy/pull/8180
[8184]: https://github.com/bevyengine/bevy/pull/8184
[8194]: https://github.com/bevyengine/bevy/pull/8194
[8195]: https://github.com/bevyengine/bevy/pull/8195
[8197]: https://github.com/bevyengine/bevy/pull/8197
[8198]: https://github.com/bevyengine/bevy/pull/8198
[8199]: https://github.com/bevyengine/bevy/pull/8199
[8212]: https://github.com/bevyengine/bevy/pull/8212
[8220]: https://github.com/bevyengine/bevy/pull/8220
[8223]: https://github.com/bevyengine/bevy/pull/8223
[8231]: https://github.com/bevyengine/bevy/pull/8231
[8232]: https://github.com/bevyengine/bevy/pull/8232
[8249]: https://github.com/bevyengine/bevy/pull/8249
[8260]: https://github.com/bevyengine/bevy/pull/8260
[8264]: https://github.com/bevyengine/bevy/pull/8264
[8265]: https://github.com/bevyengine/bevy/pull/8265
[8269]: https://github.com/bevyengine/bevy/pull/8269
[8272]: https://github.com/bevyengine/bevy/pull/8272
[8274]: https://github.com/bevyengine/bevy/pull/8274
[8275]: https://github.com/bevyengine/bevy/pull/8275
[8283]: https://github.com/bevyengine/bevy/pull/8283
[8292]: https://github.com/bevyengine/bevy/pull/8292
[8294]: https://github.com/bevyengine/bevy/pull/8294
[8295]: https://github.com/bevyengine/bevy/pull/8295
[8298]: https://github.com/bevyengine/bevy/pull/8298
[8299]: https://github.com/bevyengine/bevy/pull/8299
[8301]: https://github.com/bevyengine/bevy/pull/8301
[8306]: https://github.com/bevyengine/bevy/pull/8306
[8316]: https://github.com/bevyengine/bevy/pull/8316
[8323]: https://github.com/bevyengine/bevy/pull/8323
[8326]: https://github.com/bevyengine/bevy/pull/8326
[8330]: https://github.com/bevyengine/bevy/pull/8330
[8336]: https://github.com/bevyengine/bevy/pull/8336
[8346]: https://github.com/bevyengine/bevy/pull/8346
[8359]: https://github.com/bevyengine/bevy/pull/8359
[8362]: https://github.com/bevyengine/bevy/pull/8362
[8364]: https://github.com/bevyengine/bevy/pull/8364
[8377]: https://github.com/bevyengine/bevy/pull/8377
[8380]: https://github.com/bevyengine/bevy/pull/8380
[8387]: https://github.com/bevyengine/bevy/pull/8387
[8398]: https://github.com/bevyengine/bevy/pull/8398
[8402]: https://github.com/bevyengine/bevy/pull/8402
[8403]: https://github.com/bevyengine/bevy/pull/8403
[8408]: https://github.com/bevyengine/bevy/pull/8408
[8412]: https://github.com/bevyengine/bevy/pull/8412
[8419]: https://github.com/bevyengine/bevy/pull/8419
[8422]: https://github.com/bevyengine/bevy/pull/8422
[8425]: https://github.com/bevyengine/bevy/pull/8425
[8427]: https://github.com/bevyengine/bevy/pull/8427
[8428]: https://github.com/bevyengine/bevy/pull/8428
[8434]: https://github.com/bevyengine/bevy/pull/8434
[8436]: https://github.com/bevyengine/bevy/pull/8436
[8437]: https://github.com/bevyengine/bevy/pull/8437
[8444]: https://github.com/bevyengine/bevy/pull/8444
[8445]: https://github.com/bevyengine/bevy/pull/8445
[8446]: https://github.com/bevyengine/bevy/pull/8446
[8448]: https://github.com/bevyengine/bevy/pull/8448
[8455]: https://github.com/bevyengine/bevy/pull/8455
[8456]: https://github.com/bevyengine/bevy/pull/8456
[8460]: https://github.com/bevyengine/bevy/pull/8460
[8466]: https://github.com/bevyengine/bevy/pull/8466
[8467]: https://github.com/bevyengine/bevy/pull/8467
[8468]: https://github.com/bevyengine/bevy/pull/8468
[8470]: https://github.com/bevyengine/bevy/pull/8470
[8476]: https://github.com/bevyengine/bevy/pull/8476
[8485]: https://github.com/bevyengine/bevy/pull/8485
[8491]: https://github.com/bevyengine/bevy/pull/8491
[8494]: https://github.com/bevyengine/bevy/pull/8494
[8495]: https://github.com/bevyengine/bevy/pull/8495
[8496]: https://github.com/bevyengine/bevy/pull/8496
[8497]: https://github.com/bevyengine/bevy/pull/8497
[8503]: https://github.com/bevyengine/bevy/pull/8503
[8512]: https://github.com/bevyengine/bevy/pull/8512
[8514]: https://github.com/bevyengine/bevy/pull/8514
[8521]: https://github.com/bevyengine/bevy/pull/8521
[8522]: https://github.com/bevyengine/bevy/pull/8522
[8529]: https://github.com/bevyengine/bevy/pull/8529
[8531]: https://github.com/bevyengine/bevy/pull/8531
[8545]: https://github.com/bevyengine/bevy/pull/8545
[8548]: https://github.com/bevyengine/bevy/pull/8548
[8549]: https://github.com/bevyengine/bevy/pull/8549
[8551]: https://github.com/bevyengine/bevy/pull/8551
[8552]: https://github.com/bevyengine/bevy/pull/8552
[8561]: https://github.com/bevyengine/bevy/pull/8561
[8564]: https://github.com/bevyengine/bevy/pull/8564
[8567]: https://github.com/bevyengine/bevy/pull/8567
[8573]: https://github.com/bevyengine/bevy/pull/8573
[8575]: https://github.com/bevyengine/bevy/pull/8575
[8585]: https://github.com/bevyengine/bevy/pull/8585
[8588]: https://github.com/bevyengine/bevy/pull/8588
[8593]: https://github.com/bevyengine/bevy/pull/8593
[8601]: https://github.com/bevyengine/bevy/pull/8601
[8622]: https://github.com/bevyengine/bevy/pull/8622
[8623]: https://github.com/bevyengine/bevy/pull/8623
[8627]: https://github.com/bevyengine/bevy/pull/8627
[8631]: https://github.com/bevyengine/bevy/pull/8631
[8642]: https://github.com/bevyengine/bevy/pull/8642
[8643]: https://github.com/bevyengine/bevy/pull/8643
[8649]: https://github.com/bevyengine/bevy/pull/8649
[8650]: https://github.com/bevyengine/bevy/pull/8650
[8668]: https://github.com/bevyengine/bevy/pull/8668
[8677]: https://github.com/bevyengine/bevy/pull/8677
[8685]: https://github.com/bevyengine/bevy/pull/8685
[8687]: https://github.com/bevyengine/bevy/pull/8687
[8691]: https://github.com/bevyengine/bevy/pull/8691
[8701]: https://github.com/bevyengine/bevy/pull/8701
[8704]: https://github.com/bevyengine/bevy/pull/8704
[8711]: https://github.com/bevyengine/bevy/pull/8711
[8714]: https://github.com/bevyengine/bevy/pull/8714
[8721]: https://github.com/bevyengine/bevy/pull/8721
[8722]: https://github.com/bevyengine/bevy/pull/8722
[8723]: https://github.com/bevyengine/bevy/pull/8723
[8725]: https://github.com/bevyengine/bevy/pull/8725
[8726]: https://github.com/bevyengine/bevy/pull/8726
[8728]: https://github.com/bevyengine/bevy/pull/8728
[8732]: https://github.com/bevyengine/bevy/pull/8732
[8740]: https://github.com/bevyengine/bevy/pull/8740
[8743]: https://github.com/bevyengine/bevy/pull/8743
[8744]: https://github.com/bevyengine/bevy/pull/8744
[8753]: https://github.com/bevyengine/bevy/pull/8753
[8755]: https://github.com/bevyengine/bevy/pull/8755
[8757]: https://github.com/bevyengine/bevy/pull/8757
[8760]: https://github.com/bevyengine/bevy/pull/8760
[8761]: https://github.com/bevyengine/bevy/pull/8761
[8764]: https://github.com/bevyengine/bevy/pull/8764
[8771]: https://github.com/bevyengine/bevy/pull/8771
[8772]: https://github.com/bevyengine/bevy/pull/8772
[8776]: https://github.com/bevyengine/bevy/pull/8776
[8791]: https://github.com/bevyengine/bevy/pull/8791
[8792]: https://github.com/bevyengine/bevy/pull/8792
[8793]: https://github.com/bevyengine/bevy/pull/8793
[8795]: https://github.com/bevyengine/bevy/pull/8795
[8797]: https://github.com/bevyengine/bevy/pull/8797
[8801]: https://github.com/bevyengine/bevy/pull/8801
[8802]: https://github.com/bevyengine/bevy/pull/8802
[8803]: https://github.com/bevyengine/bevy/pull/8803
[8804]: https://github.com/bevyengine/bevy/pull/8804
[8814]: https://github.com/bevyengine/bevy/pull/8814
[8817]: https://github.com/bevyengine/bevy/pull/8817
[8818]: https://github.com/bevyengine/bevy/pull/8818
[8822]: https://github.com/bevyengine/bevy/pull/8822
[8826]: https://github.com/bevyengine/bevy/pull/8826
[8832]: https://github.com/bevyengine/bevy/pull/8832
[8833]: https://github.com/bevyengine/bevy/pull/8833
[8834]: https://github.com/bevyengine/bevy/pull/8834
[8843]: https://github.com/bevyengine/bevy/pull/8843
[8844]: https://github.com/bevyengine/bevy/pull/8844
[8845]: https://github.com/bevyengine/bevy/pull/8845
[8848]: https://github.com/bevyengine/bevy/pull/8848
[8849]: https://github.com/bevyengine/bevy/pull/8849
[8852]: https://github.com/bevyengine/bevy/pull/8852
[8866]: https://github.com/bevyengine/bevy/pull/8866
[8868]: https://github.com/bevyengine/bevy/pull/8868
[8871]: https://github.com/bevyengine/bevy/pull/8871
[8877]: https://github.com/bevyengine/bevy/pull/8877
[8878]: https://github.com/bevyengine/bevy/pull/8878
[8886]: https://github.com/bevyengine/bevy/pull/8886
[8890]: https://github.com/bevyengine/bevy/pull/8890
[8891]: https://github.com/bevyengine/bevy/pull/8891
[8901]: https://github.com/bevyengine/bevy/pull/8901
[8903]: https://github.com/bevyengine/bevy/pull/8903
[8904]: https://github.com/bevyengine/bevy/pull/8904
[8905]: https://github.com/bevyengine/bevy/pull/8905
[8907]: https://github.com/bevyengine/bevy/pull/8907
[8909]: https://github.com/bevyengine/bevy/pull/8909
[8910]: https://github.com/bevyengine/bevy/pull/8910
[8920]: https://github.com/bevyengine/bevy/pull/8920
[8928]: https://github.com/bevyengine/bevy/pull/8928
[8933]: https://github.com/bevyengine/bevy/pull/8933
[8939]: https://github.com/bevyengine/bevy/pull/8939
[8947]: https://github.com/bevyengine/bevy/pull/8947
[8951]: https://github.com/bevyengine/bevy/pull/8951
[8960]: https://github.com/bevyengine/bevy/pull/8960
[8957]: https://github.com/bevyengine/bevy/pull/8957
[9054]: https://github.com/bevyengine/bevy/pull/9054
[6690]: https://github.com/bevyengine/bevy/pull/6690
[8424]: https://github.com/bevyengine/bevy/pull/8424
[8655]: https://github.com/bevyengine/bevy/pull/8655
[6793]: https://github.com/bevyengine/bevy/pull/6793
[8720]: https://github.com/bevyengine/bevy/pull/8720
[9024]: https://github.com/bevyengine/bevy/pull/9024
[9027]: https://github.com/bevyengine/bevy/pull/9027
[9016]: https://github.com/bevyengine/bevy/pull/9016
[9023]: https://github.com/bevyengine/bevy/pull/9023
[9020]: https://github.com/bevyengine/bevy/pull/9020
[9030]: https://github.com/bevyengine/bevy/pull/9030
[9013]: https://github.com/bevyengine/bevy/pull/9013
[8926]: https://github.com/bevyengine/bevy/pull/8926
[9003]: https://github.com/bevyengine/bevy/pull/9003
[8993]: https://github.com/bevyengine/bevy/pull/8993
[8508]: https://github.com/bevyengine/bevy/pull/8508
[6056]: https://github.com/bevyengine/bevy/pull/6056
[8987]: https://github.com/bevyengine/bevy/pull/8987
[8952]: https://github.com/bevyengine/bevy/pull/8952
[8961]: https://github.com/bevyengine/bevy/pull/8961
[8978]: https://github.com/bevyengine/bevy/pull/8978
[8982]: https://github.com/bevyengine/bevy/pull/8982
[8977]: https://github.com/bevyengine/bevy/pull/8977
[8931]: https://github.com/bevyengine/bevy/pull/8931
