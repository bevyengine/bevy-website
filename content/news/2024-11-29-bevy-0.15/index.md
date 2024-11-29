+++
title = "Bevy 0.15"
date = 2024-11-29
[extra]
image = "cover.png"
show_image = true
image_subtitle = "A snake statue in volumetric fog illuminated by volumetric lighting"
image_subtitle_link = "https://sketchfab.com/3d-models/snake-statue-794b77a3e4654a669cf259d20dc89ec7"
+++

Thanks to 195 contributors, 1203 pull requests, community reviewers, and our [**generous donors**](/donate), we're happy to announce the **Bevy 0.15** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/quick-start) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.15**, check out our [0.14 to 0.15 Migration Guide](/learn/migration-guides/0-14-to-0-15/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

- **Required Components**: A rethink of how spawning entities works that significantly improves the Bevy user experience
- **Entity Picking / Selection**: A modular system for selecting entities across contexts
- **Animation Improvements**: generalized entity animation, animation masks, additive blending, and animation events
- **Curves**: a new `Curve` trait, cyclic splines, common easing functions, color gradient curves
- **Reflection Improvements**: Function reflection, unique reflect, remote type reflection
- **Bevy Remote Protocol (BRP)**: A new protocol that allows external clients (such as editors) to interact with running Bevy games
- **Visibility Bitmask Ambient Occlusion (VBAO)**: An improved GTAO algorithm that improves ambient occlusion quality
- **Chromatic Aberration**: A new post processing effect that simulates lenses that fail to focus light to a single point
- **Volumetric Fog Improvements**: "Fog volumes" that define where volumetric fog is rendered (and what form it takes), along with Point Lights and Spotlight compatibility
- **Order Independent Transparency**: A new opt-in transparency algorithm that improves the stability / quality of transparent objects as their distance from the camera changes
- **Improved Text Rendering**: We've switched to Cosmic Text for our text rendering, which significantly improves our ability to render text, especially for non-Latin-based languages that require font shaping and bidirectional text
- **Gamepads as Entities**: Gamepads are now represented as entities, making them much easier to interact with
- **UI Box Shadows**: Bevy UI nodes can now render configurable box shadows

Bevy 0.15 was prepared using our new **release candidate** process to help ensure that you can upgrade right away with peace of mind. We worked closely with both plugin authors and ordinary users to catch critical bugs, polish new features, and refine the migration guide. For each release candidate, we prepared fixes, [shipped a new release candidate on crates.io](https://crates.io/crates/bevy/versions?sort=date), let core ecosystem crates update, and listened closely for show-stopping problems. A huge thanks to [everyone who helped out](https://discord.com/channels/691052431525675048/1295069829740499015)! These efforts are a vital step towards making Bevy something that teams large and small can trust to work reliably.

<!-- more -->

{{ release_notes(version="0.15") }}

## What's Next?

The features above may be great, but what else does Bevy have in flight?
Peering deep into the mists of time (predictions are _extra_ hard when your team is almost all volunteers!), we can see some exciting work taking shape:

- **Bevy Scene Notation:** Required components mark the first step on Cart's [master plan](https://github.com/bevyengine/bevy/discussions/14437) for BSN. Over the next few months, he's going to be heads-down developing a Bevy-specific file format (complete with matching macro and IDE support), the `Construct` trait (to easily include asset data in scenes), patches (to layer modifications to scenes) and experimenting with approaches to reactivity for UI.
- **Better font support:** While `cosmic_text` is a huge leap forward for text shaping and rendering, our approach to handling fonts and type-faces is still quite crude. Bidirectional text, working with system fonts, a convenient Markdown-style "bold this section of the text" API, font fallback and more are planned.
- **Picking-Powered UI Interaction:** `bevy_picking` introduces a much more powerful and expressive way to handle pointer interactions, but we're [not leveraging its full power](https://github.com/bevyengine/bevy/issues/15550) within `bevy_ui` itself. While picking events are great, a single source of truth for "what's the user doing with this button" is vital for responsive widget styling.
- **`bevy_lint`:** Try as we might, it _is_ possible to misuse Bevy's API! As part of a broader [`bevy_cli`](https://github.com/theBevyFlock/bevy_cli) project, the Bevy community has developed a Bevy-specific linter to catch common mistakes or hazards and are looking for early adopters to try it out!
- **Focus abstraction:** Keeping track of which UI element is focused is vital to allow users of screen readers, gamepads and keyboards to comfortably navigate the UI. We're planning to build on our success with `bevy_picking` and develop a complementary [focus-tracking solution](https://github.com/bevyengine/bevy/issues/15378), along with a few simple backends to opt-in to keyboard or gamepad-based UI navigation.
- **Immutable components:** Component hooks and observers are really powerful for responding to changes and upholding invariants, but they're easily bypassed by simply mutating the component. The mad science crew has been [experimenting with](https://github.com/bevyengine/bevy/issues/16208) a way to opt-out of direct mutation, opening the door to more robust hierarchies, complex observer-powered reactions and a first-party component indexing solution.
- **Actually Retained Rendering:** While the render world is _technically_ retained in Bevy 0.15, most of our existing code still spawns and despawns entities every frame to reduce the risk of introducing bugs during the migration. We're looking forward to gradually changing this and profiling the performance impact!
- **`no_std` Bevy:** To better support weird platforms (like the [Playdate](https://play.date/)!) and make life easier for devs experimenting with Bevy on modern consoles, we've been [working towards](https://github.com/bevyengine/bevy/issues/15460) ensuring that (much of) Bevy can compile and run without Rust's standard library.

{{ support_bevy() }}

## Contributors

A huge thanks to the 195 contributors that made this release (and associated docs) possible! In random order:

- @Elabajaba
- @Kees-van-Beilen
- @rosefromthedead
- @Waridley
- @Torstein Grindvik
- @james-j-obrien
- @NiklasEi
- @hxYuki
- @softmoth
- @MarkusTheOrt
- @RobWalt
- @Natalie Baker
- @Davier
- @daxpedda
- @devnev
- @rparrett
- @Jondolf
- @Aztro-dev
- @NoahShomette
- @kettle11
- @Malax
- @Architector4
- @maniwani
- @dubrowgn
- @mamekoro
- @Leinnan
- @DasLixou
- @IceSentry
- @MrGVSV
- @nvdaz
- @SkiFire13
- @BD103
- @cbournhonesque-sc
- @wackbyte
- @atornity
- @Dig-Doug
- @kirusfg
- @hank
- @Alice Cecile
- @BorisBoutillier
- @tripokey
- @ekropotin
- @Eduardo Canellas de Oliveira
- @alice-i-cecile
- @kidrigger
- @tjamaan
- @solis-lumine-vorago
- @NathanSWard
- @mockersf
- @ebola
- @bardt
- @GitGhillie
- @mintlu8
- @SpecificProtagonist
- @nelsontkq
- @TheTacBanana
- @JMS55
- @AxiomaticSemantics
- @ameknite
- @kristoff3r
- @nelson
- @james7132
- @fantasyRqg
- @mnmaita
- @thebluefish
- @aevyrie
- @Ato2207
- @valentinegb
- @killercup
- @pablo-lua
- @janhohenheim
- @rafalh
- @Testare
- @doonv
- @wgxer
- @afonsolage
- @hecksmosis
- @HeyZoos
- @SIGSTACKFAULT
- @irate-devil
- @Vrixyz
- @tygyh
- @bogdiw
- @notverymoe
- @richardhozak
- @HugoPeters1024
- @Trashtalk217
- @nfagerlund
- @matiqo15
- @nxsaken
- @lkolbly
- @jakobhellermann
- @DGriffin91
- @AlexOkafor
- @shanecelis
- @hankjordan
- @lee-orr
- @bonsairobo
- @NthTensor
- @Bluefinger
- @thepackett
- @st0rmbtw
- @rqg
- @maueroats
- @scottmcm
- @Olle-Lukowski
- @wainwrightmark
- @xNapha
- @extrawurst
- @thmsgntz
- @brianreavis
- @garychia
- @orph3usLyre
- @TrialDragon
- @ickshonpe
- @Braymatter
- @johanhelsing
- @capt-glorypants
- @andriyDev
- @Ixentus
- @CorneliusCornbread
- @LeshaInc
- @DavJCosby
- @Pixelstormer
- @nothendev
- @tguichaoua
- @Shatur
- @torsteingrindvik
- @stepancheg
- @ArthurBrussee
- @RyanSpaker
- @dmlary
- @Gadzev
- @johnbchron
- @coreh
- @IQuick143
- @davidasberg
- @viridia
- @andristarr
- @Kanabenki
- @tychedelia
- @JoJoJet
- @kayhhh
- @ickk
- @josfeenstra
- @cart
- @Nathan-Fenner
- @antoniacobaeus
- @UkoeHB
- @Adamkob12
- @asuratos
- @matthew-gries
- @KirmesBude
- @13ros27
- @benfrankel
- @taizu-jin
- @eltociear
- @ibotha
- @Friz64
- @SludgePhD
- @rodolphito
- @tbillington
- @ManevilleF
- @GuillaumeGomez
- @bushrat011899
- @MinerSebas
- @akimakinai
- @laund
- @soqb
- @hymm
- @cBournhonesque
- @VitalyAnkh
- @komadori
- @fuchsnj
- @simbleau
- @robtfm
- @asafigan
- @Nilirad
- @MiniaczQ
- @superdump
- @jeliag
- @esensar
- @porkbrain
- @anarelion
- @seabassjh
- @TimJentzsch
- @laundmo
- @NiseVoid
- @nicopap
- @TheBlckbird
- @re0312
- @SET001
- @pcwalton
- @ItsDoot
- @Aceeri

## Full Changelog

## A-Rendering + A-Windowing

- [Allow prepare_windows to run off main thread.][11660]
- [Allow prepare_windows to run off main thread on all platforms][11672]
- [don't run `create_surfaces` system if not needed][11720]
- [fix create_surfaces system ordering][11747]

## A-Animation + A-Reflection

- [Add type registrations for animation types][11889]

## A-Assets

- [Don't `.unwrap()` in `AssetPath::try_parse`][10452]
- [feat: `Debug` implemented for `AssetMode`][10494]
- [Remove rogue : from embedded_asset! docs][10516]
- [use `tree` syntax to explain bevy_rock file structure][10523]
- [Make AssetLoader/Saver Error type bounds compatible with anyhow::Error][10493]
- [Fix untyped labeled asset loading][10514]
- [Add `load_untyped` to LoadContext][10526]
- [fix example custom_asset_reader on wasm][10574]
- [`ReadAssetBytesError::Io`  exposes failing path][10450]
- [Added Method to Allow Pipelined Asset Loading][10565]
- [Add missing asset load error logs for load_folder and load_untyped][10578]
- [Fix wasm builds with file_watcher enabled][10589]
- [Do not panic when failing to create assets folder (#10613)][10614]
- [Use handles for queued scenes in SceneSpawner][10619]
- [Fix file_watcher feature hanging indefinitely][10585]
- [derive asset for enums][10410]
- [Ensure consistency between Un/Typed `AssetId` and `Handle`][10628]
- [Fix Asset Loading Bug][10698]
- [remove double-hasing of typeid for handle][10699]
- [AssetMetaMode][10623]
- [Fix GLTF scene dependencies and make full scene renders predictable][10745]
- [Print precise and correct watch warnings (and only when necessary)][10787]
- [Allow removing and reloading assets with live handles][10785]
- [Add GltfLoaderSettings][10804]
- [Refactor `process_handle_drop_internal()` in bevy_asset][10920]
- [fix base64 padding when loading a gltf file][11053]
- [assets should be kept on CPU by default][11212]
- [Don't auto create assets folder][11218]
- [Use `impl Into<A>` for `Assets::add`][10878]
- [Add `reserve_handle` to `Assets`.][10939]
- [Better error message on incorrect asset label][11254]
- [GLTF extension support][11138]
- [Fix embedded watcher to work with external crates][11370]
- [Added AssetLoadFailedEvent, UntypedAssetLoadFailedEvent][11369]
- [auto create imported asset folder if needed][11284]
- [Fix minor typo][11491]
- [Include asset path in get_meta_path panic message][11504]
- [Fix documentation for `AssetReader::is_directory` function][11538]
- [AssetSaver and AssetTransformer split][11260]
- [AssetPath source parse fix][11543]
- [Allow TextureAtlasBuilder in AssetLoader][11548]
- [Add a getter for asset watching status on `AssetServer`][11578]
- [Make SavedAsset::get_labeled accept &str as label][11612]
- [Added Support for Extension-less Assets][10153]
- [Fix embedded asset path manipulation][10383]
- [Fix AssetTransformer breaking LabeledAssets][11626]
- [Put asset_events behind a run condition][11800]
- [Use Asset Path Extension for `AssetLoader` Disambiguation][11644]

## A-Core + A-App

- [Add Accessibility plugin to default plugins docs][11512]

## A-Accessibility

- [Add html tags required for accessibility][10989]
- [missed negation during accessibility refactor][11206]

## A-Transform

- [Add `Transform::is_finite`][10592]

## A-ECS + A-Hierarchy

- [Add a doc note about despawn footgun][10889]

## A-Text

- [Rename `TextAlignment` to `JustifyText`.][10854]
- [Subtract 1 from text positions to account for glyph texture padding.][11662]

## A-Assets + A-UI

- [UI and unloaded assets: don't filter out nodes with an unloaded image][11205]

## A-Utils + A-Time

- [Make SystemTime available in both native and wasm][10980]

## A-Rendering + A-Assets

- [Fix shader import hot reloading on windows][10502]
- [Unload render assets from RAM][10520]
- [mipmap levels can be 0 and they should be interpreted as 1][11767]

## A-Physics

- [refactor collide code (Adopted)][11106]
- [Use `IntersectsVolume` for breakout example collisions][11500]

## A-ECS + A-Editor + A-App + A-Diagnostics

- [System Stepping implemented as Resource][8453]

## A-Reflection + A-Scenes

- [Implement and register Reflect (value) for CameraRenderGraph and CameraMainTextureUsages][11878]

## A-Audio + A-Windowing

- [Winit update: fix suspend on Android][11403]

## A-Build-System + A-Meta

- [Standardize toml format with taplo][10594]

## A-ECS + A-Time

- [Wait until `FixedUpdate` can see events before dropping them][10077]
- [Add First/Pre/Post/Last schedules to the Fixed timestep][10977]
- [Add run conditions for executing a system after a delay][11095]
- [Add paused run condition][11313]

## A-Meta

- [Add "update screenshots" to release checklist][10369]
- [Remove references to specific projects from the readme][10836]
- [Fix broken link between files][10962]
- [[doc] Fix typo in CONTRIBUTING.md][10971]
- [Remove unused namespace declarations][10965]
- [Add docs link to root `Cargo.toml`][10998]
- [Migrate third party plugins guidelines to the book][11242]
- [Run markdownlint][11386]
- [Improve `config_fast_builds.toml`][11529]
- [Use `-Z threads=0` option in `config_fast_builds.toml`][11541]
- [CONTRIBUTING.md: Mention splitting complex PRs][11703]

## A-Time

- [docs: use `read` instead of deprecated `iter`][10376]
- [Rename `Time::<Fixed>::overstep_percentage()` and `Time::<Fixed>::overstep_percentage_f64()`][10448]
- [Rename `Timer::{percent,percent_left}` to `Timer::{fraction,fraction_remaining}`][10442]
- [Document how to configure FixedUpdate][10564]
- [Add discard_overstep function to Time<Fixed>][10453]

## A-Assets + A-Reflection

- [Register `AssetPath` as type for reflection][11483]

## A-Diagnostics + A-Utils

- [move once from bevy_log to bevy_utils, to allow for it's use in bevy_ecs][11419]

## A-Windowing + A-App

- [Revert `App::run()` behavior/Remove `winit` specific code from `bevy_app`][10389]

## A-ECS + A-Scenes

- [Make the MapEntities trait generic over Mappers, and add a simpler EntityMapper][11428]

## A-Hierarchy

- [bevy_hierarchy: add some docs][10598]
- [Make bevy_app and reflect opt-out for bevy_hierarchy.][10721]
- [Add `bevy_hierarchy` Crate and plugin documentation][10951]
- [Rename "AddChild" to "PushChild"][11194]
- [Inline trivial methods in bevy_hierarchy][11332]

## A-ECS + A-App

- [Add custom schedule example][11527]

## A-Transform + A-Math

- [return Direction3d from Transform::up and friends][11604]

## A-UI + A-Text

- [Improved Text Rendering][10537]
- [Feature-gate all references to `bevy_text` in `bevy_ui`][11391]

## A-Input

- [Make ButtonSettings.is_pressed/released public][10534]
- [Rename `Input` to `ButtonInput`][10859]
- [Add method to check if all inputs are pressed][11010]
- [Add window entity to TouchInput events][11128]
- [Extend `Touches` with clear and reset methods][10930]
- [Add logical key data to KeyboardInput][11400]
- [Derive Ord for GamepadButtonType.][11791]
- [Add delta to CursorMoved event][11710]

## A-Rendering + A-Diagnostics

- [Use `warn_once` where relevant instead of manually implementing a single warn check][11693]

## A-Rendering

- [Fix bevy_pbr shader function name][10423]
- [Implement Clone for VisibilityBundle and SpatialBundle][10394]
- [Reexport `wgpu::Maintain`][10461]
- [Use a consistent scale factor and resolution in stress tests][10474]
- [Ignore inactive cameras][10543]
- [Add shader_material_2d example][10542]
- [More inactive camera checks][10555]
- [Fix post processing example to only run effect on camera with settings component][10560]
- [Make sure added image assets are checked in camera_system][10556]
- [Ensure ExtendedMaterial works with reflection (to enable bevy_egui_inspector integration)][10548]
- [Explicit color conversion methods][10321]
- [Re-export wgpu BufferAsyncError][10611]
- [Improve shader_material example][10547]
- [Non uniform transmission samples][10674]
- [Explain how `AmbientLight` is inserted and configured][10712]
- [Add wgpu_pass method to TrackedRenderPass][10722]
- [Add a `depth_bias` to `Material2d`][10683]
- [Use as_image_copy where possible][10733]
- [impl From<Color> for ClearColorConfig][10734]
- [Ensure instance_index push constant is always used in prepass.wgsl][10706]
- [Bind group layout entries][10224]
- [prepass vertex shader always outputs world position][10657]
- [Swap material and mesh bind groups][10485]
- [try_insert Aabbs][10801]
- [Fix prepass binding issues causing crashes when not all prepass bindings are used][10788]
- [Fix binding group in custom_material_2d.wgsl][10841]
- [Normalize only nonzero normals for mikktspace normal maps][10905]
- [light renderlayers][10742]
- [Explain how RegularPolygon mesh is generated][10927]
- [Fix Mesh2d normals on webgl][10967]
- [Update to wgpu 0.18][10266]
- [Fix typo in docs for `ViewVisibility`][10979]
- [Add docs to bevy_sprite a little][10947]
- [Fix BindingType import warning][10818]
- [Update texture_atlas example with different padding and sampling][10073]
- [Update AABB when Sprite component changes in calculate_bounds_2d()][11016]
- [OrthographicProjection.scaling_mode is not just for resize][11024]
- [Derive `Debug` for `BloomCompositeMode`][11041]
- [Document None conditions on compute_aabb][11051]
- [Replace calculation with function call][11077]
- [Register Camera types.][11069]
- [Add example for pixel-perfect grid snapping in 2D][8112]
- [Misc cleanup][11134]
- [Keep track of when a texture is first cleared][10325]
- [Fix Mesh::ATTRIBUTE_UV_0 documentation][11110]
- [Do not load prepass normals for transmissive materials][11140]
- [Export tonemapping_pipeline_key (2d), alpha_mode_pipeline_key][11166]
- [Simplify examples/3d/orthographic][11045]
- [Implement lightmaps.][10231]
- [Bump the vertex attribute index for prepass joints.][11191]
- [Fix: Gizmos crash due to the persistence policy being set to `Unload`. Change it to `Keep`][11192]
- [Usability methods for RenderTargets and image handles][10736]
- [Explain Camera physical size is in pixel][11189]
- [update Outdated comment][11243]
- [Revert "Implement minimal reflection probes. (#10057)"][11307]
- [Explain OrthographicProjection.scale][11023]
- [Mul<f32> for ScalingMode][11030]
- [Rustdoc examples for OrthographicProjection][11031]
- [Option to enable deterministic rendering][11248]
- [Fix ssao only sampling mip 0][11292]
- [Revert "Implement minimal reflection probes. (#10057)"][11307]
- [Sprite slicing and tiling][10588]
- [Approximate indirect specular occlusion][11152]
- [Texture Atlas rework][5103]
- [Exposure settings (adopted)][11347]
- [Remove Vec from GpuArrayBuffer][11368]
- [Make `DynamicUniformBuffer::push` accept an `&T` instead of `T`][11373]
- [Restore brightness in the remaining three examples after exposure PR][11389]
- [Customizable camera main texture usage][11412]
- [Cleanup deterministic example][11416]
- [Implement minimal reflection probes (fixed macOS, iOS, and Android).][11366]
- [optimize  batch_and_prepare_render_phase][11323]
- [add `storage_texture` option to as_bind_group macro][9943]
- [Revert rendering-related associated type name changes][11027]
- [Meshlet prep][11442]
- [Reuse sampler when creating cached bind groups][10610]
- [Add Animated Material example][11524]
- [Update to wgpu 0.19 and raw-window-handle 0.6][11280]
- [Fix bug where Sprite::rect was ignored][11480]
- [Added documentation explaining the difference between lumens and luxes][11551]
- [Fix infinite asset preparation due to undrained AssetEvent events][11383]
- [Workaround for ICE in the DXC shader compiler in debug builds with an `EnvironmentMapLight`][11487]
- [Refactor tonemapping example's image viewer update into two systems][11519]
- [Add `Mesh` transformation][11454]
- [Fix specular envmap in deferred][11534]
- [Add `Meshable` trait and implement meshing for 2D primitives][11431]
- [Optimize extract_clusters and prepare_clusters systems][10633]
- [RenderAssetPersistencePolicy → RenderAssetUsages][11399]
- [RenderGraph Labelization][10644]
- [Gate diffuse and specular transmission behind shader defs][11627]
- [Add helpers for translate, rotate, and scale operations - Mesh][11675]
- [CameraProjection::compute_frustum][11139]
- [Added formats to `MeshVertexAttribute` constant's docstrings][11705]
- [Async pipeline compilation][10812]
- [sort by pipeline then mesh for non transparent passes for massively better batching][11671]
- [Added remove_indices to Mesh][11733]
- [Implement irradiance volumes.][10268]
- [Mesh insert indices][11745]
- [Don't try to create a uniform buffer for light probes if there are no views.][11751]
- [Properly check for result when getting pipeline in Msaa][11758]
- [wait for render app when main world is dropped][11737]
- [Deprecate shapes in `bevy_render::mesh::shape`][11773]
- [Cache the QueryState used to drop swapchain TextureViews][11781]
- [Multithreaded render command encoding][9172]
- [Fix `Quad` deprecation message mentioning a type that doesn't exist][11798]
- [Stop extracting mesh entities to the render world.][11803]
- [Stop copying the light probe array to the stack in the shader.][11805]
- [Add `Mesh::merge`][11456]
- [Call a TextureAtlasLayout a layout and not an atlas][11783]
- [fix shadow batching][11645]
- [Change light defaults & fix light examples][11581]
- [New Exposure and Lighting Defaults (and calibrate examples)][11868]
- [Change MeshUniform::new() to be public.][11880]
- [Rename Core Render Graph Labels][11882]
- [Support optional clear color in ColorAttachment.][11884]
- [irradiance: use textureSampleLevel for WebGPU support][11893]
- [Add configuration for async pipeline creation on RenderPlugin][11847]
- [Derive Reflect for Exposure][11907]
- [Add `MeshPipelineKey::LIGHTMAPPED` as applicable during the shadow map pass.][11910]
- [Irradiance volume example tweaks][11911]
- [Disable irradiance volumes on WebGL and WebGPU.][11909]
- [Remove `naga_oil` dependency from `bevy_pbr`][11914]

## A-Scenes

- [Re-export `ron` in `bevy_scene`][10529]
- [Fix load scene example to use proper serialization format for rotation field][10638]
- [Mention DynamicSceneBuilder in doc comment][10780]
- [Mention DynamicSceneBuilder in scene example][10441]
- [Implement Std traits for `SceneInstanceReady`][11003]
- [Change SceneSpawner::spawn_dynamic_sync to return InstanceID][11239]
- [Fix scene example][11289]
- [Send `SceneInstanceReady` only once per scene][11002]

## A-Utils

- [bevy_utils: Export `generate_composite_uuid` utility function][10496]
- [Save an instruction in `EntityHasher`][10648]
- [Add SystemTime to bevy_utils][11054]
- [Re-export smallvec crate from bevy_utils][11006]
- [Enable cloning EntityHashMap and PreHashMap][11178]
- [impl `Borrow` and `AsRef` for `CowArc`][11616]
- [Hash stability guarantees][11690]
- [Deprecating hashbrown reexports][11721]
- [Update ahash to 0.8.7][11785]

## A-UI

- [ui material: fix right border width][10421]
- [Add PartialEq to Anchor][10424]
- [UI Material: each material should have its own buffer][10422]
- [UI Materials: ignore entities with a `BackgroundColor` component][10434]
- [Fix panic when using image in UiMaterial][10591]
- [Make clipped areas of UI nodes non-interactive][10454]
- [Fix typo in resolve_outlines_system][10730]
- [Clip outlines by the node's own clipping rect, not the parent's.][10922]
- [Give UI nodes with `Display::None` an empty clipping rect][10942]
- [Create serialize feature for bevy_ui][11188]
- [Made the remaining types from bevy_ui to reflect the Default trait if…][11199]
- [Camera-driven UI][10559]
- [fix occasional crash moving ui root nodes][11371]
- [Fix panic on Text UI without Cameras][11405]
- [Allow user to choose default ui camera][11436]
- [Rustdoc links in bevy_ui][11555]
- [Avoid unconditionally unwrapping the Result - UI Stack System][11575]

## A-Assets + A-Diagnostics

- [Fix asset loader registration warning][11870]

## A-Audio + A-Reflection

- [Reflect and register audio-related types][10484]

## A-Audio

- [Add `VolumeLevel::ZERO`][10608]
- [Deduplicate systems in bevy_audio][10906]
- [Non-Intrusive refactor of `play_queued_audio_system()`][10910]
- [docs: AnimationPlayer::play doesn't have transition_duration arg][10970]
- [Remove the ability to ignore global volume][11092]
- [Optional override for global spatial scale][10419]

## A-Tasks

- [Make FakeTask public on singlethreaded context][10517]
- [Re-export `futures_lite` in `bevy_tasks`][10670]
- [bump bevy_tasks futures-lite to 2.0.1][10675]
- [Fix wrong transmuted type in `TaskPool::scope_with_executor_inner`][11455]
- [Use `std::thread::sleep` instead of spin-waiting in the async_compute example][11856]

## A-ECS

- [Use `EntityHashMap` for `EntityMapper`][10415]
- [Allow registering boxed systems][10378]
- [Remove unnecessary if statement in scheduler][10446]
- [Optimize `Entity::eq`][10519]
- [Add 'World::run_system_with_input' function + allow `World::run_system` to get system output][10380]
- [Update `Event` send methods to return `EventId`][10551]
- [Some docs for IntoSystemSet][10563]
- [Link to `In` in `pipe` documentation][10596]
- [Optimise `Entity` with repr align & manual `PartialOrd`/`Ord`][10558]
- [Allow #[derive(Bundle)] on tuple structs (take 3)][10561]
- [Add an `Entry` api to `EntityWorldMut`.][10650]
- [Make impl block for RemovedSystem generic][10651]
- [Append commands][10400]
- [Rustdoc example for Ref][10682]
- [Link to `Main` schedule docs from other schedules][10691]
- [Warn that Added/Changed filters do not see deferred changes][10681]
- [Fix non-functional nondeterministic_system_order example][10719]
- [Copy over docs for `Condition` trait from PR #10718][10748]
- [Implement `Drop` for `CommandQueue`][10746]
- [Split WorldQuery into WorldQueryData and WorldQueryFilter][9918]
- [Make IntoSystemConfigs::into_configs public API (visible in docs)][10624]
- [Override QueryIter::fold to port Query::for_each perf gains to select Iterator combinators][6773]
- [Deprecate QueryState::for_each_unchecked][10815]
- [Clarifying Commands' purpose][10837]
- [Make ComponentId typed in Components][10770]
- [Reduced `TableRow` `as` Casting][10811]
- [Add `EntityCommands.retain` and `EntityWorldMut.retain`][10873]
- [Remove unnecessary ResMut in examples][10879]
- [Add a couple assertions for system types][10893]
- [Remove reference to default schedule][10918]
- [Improve `EntityWorldMut.remove`, `retain` and `despawn` docs by linking to more detail][10943]
- [Reorder fields in SystemSchedule][10764]
- [Rename `WorldQueryData` & `WorldQueryFilter` to `QueryData` & `QueryFilter`][10779]
- [Fix soundness of `UnsafeWorldCell` usage example][10941]
- [Actually check alignment in BlobVec test aligned_zst][10885]
- [Rename `Q` type parameter to `D` when referring to `WorldQueryData`][10782]
- [Allow the editing of startup schedules][10969]
- [Auto insert sync points][9822]
- [Simplify lifetimes in `QueryState` methods][10937]
- [Add is_resource_changed_by_id + is_resource_added_by_id][11012]
- [Rename some lifetimes (ResMut etc) for clarity][11021]
- [Add non-existent entity behavior to Has doc][11025]
- [Fix typo in docs for Has][11028]
- [Add insert_state to App.][11043]
- [Explain Changed, Added are not archetype filters][11049]
- [Add missing colon in `States` documentation][11064]
- [Explain EventWriter limits concurrency][11063]
- [Better doc for SystemName][11084]
- [impl ExclusiveSystemParam for WorldId][11164]
- [impl ExclusiveSystemParam for PhantomData][11153]
- [Remove little warn on bevy_ecs][11149]
- [Rename `ArchetypeEntity::entity` into `ArchetypeEntity::id`][11118]
- [Fixed Typo in the description of EntityMut][11103]
- [Implement Deref and DerefMut for In][11104]
- [impl ExclusiveSystemParam for SystemName][11163]
- [Print a warning for un-applied commands being dropped from a CommandQueue][11146]
- [Implement TypePath for EntityHash][11195]
- [Fix integer overflow in BlobVec::push for ZST][10799]
- [Fix integer overflow in BlobVec::reserve_exact][11234]
- [StateTransitionEvent][11089]
- [Restore support for running `fn` `EntityCommands` on entities that might be despawned][11107]
- [Remove apply_deferred example][11142]
- [Minimize small allocations by dropping the tick Vecs from Resources][11226]
- [Change Entity::generation from u32 to NonZeroU32 for niche optimization][9907]
- [fix B0003 example and update logs][11162]
- [Unified identifer for entities & relations][9797]
- [Simplify conditions][11316]
- [Add example using `State` in docs][11319]
- [Skip rehashing TypeIds][11268]
- [Make `TypeId::hash` more robust in case of upstream rustc changes][11334]
- [Fix doc of [`Schedules`] to mention exclusion of current schedule.][11360]
- [Dynamic queries and builder API][9774]
- [Remove duplicate `#[automatically_derived]` in ECS macro][11388]
- [Get Change Tick methods for Resources][11404]
- [Optional state][11417]
- [Double the capacity when BlobVec is full][11167]
- [document which lifetime is needed for systemparam derive][11321]
- [refactor: Simplify lifetimes for `Commands` and related types][11445]
- [Implement `Debug` for `CommandQueue`][11444]
- [Fix typo in comment][11486]
- [Rename Schedule::name to Schedule::label][11531]
- [Exclusive systems can now be used for one-shot systems][11560]
- [added ability to get `Res<T>` from `World` with `World::get_resource_ref`][11561]
- [bevy_ecs: Add doc example for par_iter_mut (#11311)][11499]
- [Add an example demonstrating how to send and receive events in the same system][11574]
- [Add a doctest example for EntityMapper][11583]
- [Rephrase comment about Local<T> for clarity. (Adopted)][11129]
- [Use batch spawn in benchmarks][11611]
- [Fix bug where events are not being dropped][11528]
- [Make Archetypes.archetype_component_count private][10774]
- [Deprecated Various Component Methods from `Query` and `QueryState`][9920]
- [`System::type_id` Consistency][11728]
- [Typo in [`ScheduleLabel`] derive macro][11764]
- [Mention Resource where missing from component/resource related type docs][11769]
- [Expose query accesses][11700]
- [Add a method for detecting changes within a certain scope][11687]
- [Fix double indirection when applying command queues][11822]
- [Immediately poll the executor once before spawning it as a task][11801]
- [Fix small docs misformat in `BundleInfo::new`][11855]
- [`FilteredEntityRef` conversions][11838]

## A-Rendering + A-Animation

- [TextureAtlasBuilder now respects insertion order][11474]
- [normalize joint weights][10539]

## A-ECS + A-Meta

- [resolve all internal ambiguities][10411]

## A-Rendering + A-UI

- [Provide GlobalsUniform in UiMaterial shaders][10739]
- [Include UI node size in the vertex inputs for UiMaterial.][11722]
- [UI Texture 9 slice][11600]
- [Optional ImageScaleMode][11780]

## A-Math

- [Define a basic set of Primitives][10466]
- [Add and impl Primitives][10580]
- [Add winding order for `Triangle2d`][10620]
- [Use minor and major radii for `Torus` primitive shape][10643]
- [Remove `From` implementations from the direction types][10857]
- [Impl `TryFrom` vector for directions and add `InvalidDirectionError`][10884]
- [Add `Direction2d::from_xy` and `Direction3d::from_xyz`][10882]
- [Implement `Neg` for `Direction2d` and `Direction3d`][11179]
- [Add constants for `Direction2d` and `Direction3d`][11180]
- [Add `approx` feature to `bevy_math`][11176]
- [Add `libm` feature to `bevy_math`][11238]
- [Add `new_and_length` method to `Direction2d` and `Direction3d`][11172]
- [Update `glam`, `encase` and `hexasphere`][11082]
- [Implement bounding volume types][10946]
- [Remove `Default` impl for `CubicCurve`][11335]
- [Implement bounding volumes for primitive shapes][11336]
- [Improve `Rectangle` and `Cuboid` consistency][11434]
- [Change `Ellipse` representation and improve helpers][11435]
- [Add `Aabb2d::new` and `Aabb3d::new` constructors][11433]
- [Add geometric primitives to `bevy_math::prelude`][11432]
- [Direction: Rename `from_normalized` to `new_unchecked`][11425]
- [Implement bounding volume intersections][11439]
- [Add `new` constructors for `Circle` and `Sphere`][11526]
- [Derive PartialEq, Serialize, Deserialize and Reflect on primitives][11514]
- [Document RegularPolygon][11017]
- [Add RayTest2d and RayTest3d][11310]
- [Add more constructors and math helpers for primitive shapes][10632]
- [Add `Capsule2d` primitive][11585]
- [Add volume cast intersection tests][11586]
- [Add Clone to intersection test types][11640]
- [Implement `approx` traits for direction types][11650]
- [Support rotating `Direction3d` by `Quat`][11649]
- [Rename RayTest to RayCast][11635]
- [Add example for bounding volumes and intersection tests][11666]
- [Dedicated primitive example][11697]
- [Un-hardcode positions and colors in `2d_shapes` example][11867]

## A-Build-System

- [check for all-features with cargo-deny][10544]
- [Bump actions/github-script from 6 to 7][10653]
- [Add doc_markdown clippy linting config to cargo workspace][10640]
- [Enable `clippy::undocumented_unsafe_blocks` warning across the workspace][10646]
- [Remove trailing whitespace][10723]
- [Move remaining clippy lint definitions to Cargo.toml][10672]
- [Add `clippy::manual_let_else` at warn level to lints][10684]
- [Remove unused import][10963]
- [Rename functions and variables to follow code style][10961]
- [Remove unused variable][10966]
- [add libxkbcommon-x11-0 to the default linux dependencies][11060]
- [fix patches for example showcase after winit update][11058]
- [finish cleaning up dependency bans job][11059]
- [Bump actions/upload-artifact from 2 to 4][11014]
- [Publish dev-docs with Github Pages artifacts (2nd attempt)][10892]
- [example showcase patches: use default instead of game mode for desktop][11250]
- [Bump toml_edit in build-template-pages tool][11342]
- [Miri is failing on latest nightly: pin nightly to last known working version][11421]
- [Bump dev-docs pages actions][11418]
- [Unpin nightly for miri][11462]
- [documentation in CI: remove lock file][11507]
- [Bump actions/cache from 3 to 4][11469]
- [simplify animated_material example][11576]
- [example showcase: fix window resized patch][11596]
- [run examples on macOS to validate PRs][11630]
- [Inverse `missing_docs` logic][11676]
- [Bump peter-evans/create-pull-request from 5 to 6][11712]

## A-Gizmos

- [Fix float precision issue in the gizmo shader][10408]
- [Gizmo Arrows][10550]
- [Move Circle Gizmos to Their Own File][10631]
- [move gizmo arcs to their own file][10660]
- [Warn when bevy_sprite and bevy_pbr are not enabled with bevy_gizmos][11296]
- [Multiple Configurations for Gizmos][10342]
- [Fix gizmos app new panic][11420]
- [Use Direction3d for gizmos.circle normal][11422]
- [Implement Arc3D for Gizmos][11540]
- [Insert Gizmos config instead of Init][11580]
- [Drawing Primitives with Gizmos][11072]
- [fix(primitives): fix polygon gizmo rendering bug][11699]
- [Fix global wireframe behavior not being applied on new meshes][11792]
- [Overwrite gizmo group in `insert_gizmo_group`][11860]

## A-Rendering + A-Math

- [Split `Ray` into `Ray2d` and `Ray3d` and simplify plane construction][10856]
- [Introduce AspectRatio struct][10368]
- [Implement meshing for `Capsule2d`][11639]
- [Implement `Meshable` for some 3D primitives][11688]

## A-Core

- [Derive `Debug` for `Framecount`][11573]
- [Don't unconditionally enable bevy_render or bevy_assets if mutli-threaded feature is enabled][11726]

## A-Windowing

- [Some explanations for Window component][10714]
- [don't run update before window creation in winit][10741]
- [add new event `WindowOccluded` from winit][10735]
- [Add comment about scale factor in `WindowMode`][10872]
- [Refactor function `update_accessibility_nodes`][10911]
- [Change `Window` scale factor to f32 (adopted)][10897]
- [Reexport winit::platform::android::activity::* in bevy_winit][11011]
- [Update winit dependency to 0.29][10702]
- [Remove CanvasParentResizePlugin][11057]
- [Use `WindowBuilder::with_append()` to append canvas][11065]
- [Fix perf degradation on web builds][11227]
- [mobile and webgpu: trigger redraw request when needed and improve window creation][11245]
- [Remove unnecessary unsafe impls for WinitWindows on Wasm][11270]
- [Fix Reactive and ReactiveLowPower update modes][11325]
- [Change `WinitPlugin` defaults to limit game update rate when window is not visible (for real this time)][11305]
- [Cleanup bevy winit][11489]
- [Add `name` to `bevy::window::Window`][7650]
- [Avoid unwraps in winit fullscreen handling code][11735]

## A-UI + A-Transform + A-Text

- [UI text rotation and scaling fix][11326]

## A-Animation

- [Fix animations resetting after repeat count][10540]
- [Add Debug, PartialEq and Eq derives to bevy_animation.][10562]
- [support all types of animation interpolation from gltf][10755]
- [Clean up code to find the current keyframe][11306]
- [Skip alloc when updating animation path cache][11330]
- [Replace the `cubic_spline_interpolation` macro with a generic function][11605]
- [Animatable trait for interpolation and blending][4482]

## A-ECS + A-Pointers

- [Replace pointer castings (`as`) by their API equivalent][11818]

## A-ECS + A-Utils

- [Add helper macro's for logging only once][10808]
- [Move `EntityHash` related types into `bevy_ecs`][11498]

## A-Reflection

- [Fix issue with `Option` serialization][10705]
- [fix `insert_reflect` panic caused by `clone_value`][10627]
- [Remove pointless trait implementation exports in `bevy_reflect`][10771]
- [Fix nested generics in Reflect derive][10791]
- [Fix debug printing for dynamic types][10740]
- [reflect: maximally relax `TypePath` bounds][11037]
- [Use `static_assertions` to check for trait impls][11407]
- [Add `ReflectFromWorld` and replace the `FromWorld` requirement on `ReflectComponent` and `ReflectBundle` with `FromReflect`][9623]
- [Fix reflected serialization/deserialization on `Name` component][11447]
- [Add Reflection for Wrapping/Saturating types][11397]
- [Remove TypeUuid][11497]
- [Fix warnings in bevy_reflect][11556]
- [bevy_reflect: Type parameter bounds][9046]
- [bevy_reflect: Split `#[reflect(where)]`][11597]
- [reflection: replace `impl_reflect_struct` with `impl_reflect`][11437]
- [Add the ability to manually create ParsedPaths (+ cleanup)][11029]
- [bevy_reflect: Reflect `&'static str`][11686]
- [Improve DynamicStruct::insert][11068]
- [Missing registrations][11736]
- [Add `ReflectKind`][11664]
- [doc(bevy_reflect): add note about trait bounds on `impl_type_path`][11810]
- [bevy_reflect_derive: Clean up attribute logic][11777]

## A-ECS + A-Tasks

- [Async channel v2][10692]

## A-Pointers

- [Remove a ptr-to-int cast in `CommandQueue::apply`][10475]
- [Fix memory leak in dynamic ECS example][11461]
- [bevy_ptr: fix `unsafe_op_in_unsafe_fn` lint][11610]

## A-ECS + A-Reflection

- [Adding derive Reflect for tick structs][11641]

## A-Reflection + A-Gizmos

- [`#[derive(Reflect)]` on `GizmoConfig`][10483]
- [Register `WireframeColor`][10486]

## No area label

- [Fix intra-doc link warnings][10445]
- [Fix minor issues with custom_asset example][10337]
- [Prepend `root_path` to meta path in HttpWasmAssetReader][10527]
- [support required features in wasm examples showcase][10577]
- [examples showcase: use patches instead of sed for wasm hacks][10601]
- [Add [lints] table, fix  adding `#![allow(clippy::type_complexity)]` everywhere][10011]
- [Bumps async crates requirements to latest major version][10370]
- [delete methods deprecated in 0.12][10693]
- [Ran `cargo fmt` on `benches` crate][10758]
- [Remove unnecessary path prefixes][10749]
- [Fix typos in safety comment][10827]
- [Substitute `get(0)` with `first()`][10847]
- [Remove identity `map` calls][10848]
- [Renamed Accessibility plugin to AccessKitPlugin in bevy_winit][10914]
- [Reorder impl to be the same as the trait][11076]
- [Replace deprecated elements][10999]
- [Remove unnecessary parentheses][10990]
- [Replace deprecated elements][10999]
- [Simplify equality assertions][10988]
- [Add Solus package requrements to linux_dependencies.md][10996]
- [Update base64 requirement from 0.13.0 to 0.21.5][10336]
- [Update sysinfo version to 0.30.0][11071]
- [Remove unnecessary parens][11075]
- [Reorder impl to be the same as the trait][11076]
- [Fix ci xvfb][11143]
- [Replace or document ignored doctests][11040]
- [Add static assertions to bevy_utils for compile-time checks][11182]
- [Fix missed explicit conversions in examples][11261]
- [Remove unused event-listener dependency][11269]
- [Fixed typo in generate_custom_mesh.rs example][11293]
- [Extract examples `CameraController` into a module][11338]
- [Use EntityHashMap whenever possible][11353]
- [Fix link to plugin guidelines][11379]
- [[doc] Fix typo and formatting in CONTRIBUTING.md][11381]
- [add a required feature for shader_material_glsl][11440]
- [Update ruzstd requirement from 0.4.0 to 0.5.0][11467]
- [Tweak gamepad viewer example style][11484]
- [Add `.toml` extension to `.cargo/config_fast_builds`][11506]
- [Add README to benches][11508]
- [Fix panic in examples using argh on the web][11513]
- [Fix cyclic dep][11523]
- [Enable the `unsafe_op_in_unsafe_fn` lint][11591]
- [Update erased-serde requirement from 0.3 to 0.4][11599]
- [Fix example send_and_receive_events][11615]
- [Update cursor.rs][11617]
- [Use the `Continuous` update mode in stress tests when unfocused][11652]
- [Don't auto insert on the extract schedule][11669]
- [Update tracing-tracy requirement from 0.10.4 to 0.11.0 and tracy-client requirement from 0.16.4 to 0.17.0][11678]
- [Use TypeIdMap whenever possible][11684]
- [Fix a few typos in error docs][11709]
- [bevy_render: use the non-send marker from bevy_core][11725]
- [Ignore screenshots generated by `screenshot` example][11797]
- [Docs reflect that `RemovalDetection` also yields despawned entities][11795]
- [bevy_dynamic_plugin: fix `unsafe_op_in_unsafe_fn` lint][11622]
- [Replace `crossbeam::scope` reference with `thread::scope` in docs][11832]
- [Use question mark operator when possible][11865]
- [Fix a few Clippy lints][11866]
- [WebGPU: fix web-sys version][11894]
- [Remove map_flatten from linting rules][11913]
- [Fix duplicate `encase_derive_impl` dependency][11915]

## A-App

- [add regression test for #10385/#10389][10609]
- [Fix typos plugin.rs][11193]
- [Expressively define plugins using functions][11080]
- [Mark `DynamicPluginLoadError` internal error types as source][11618]

## A-Diagnostics

- [Fix Line for tracy version][10663]
- [Some doc to bevy_diagnostic][11020]
- [Print to stderr from panic handler in LogPlugin][11170]
- [Add ability to panic to logs example][11171]
- [Make sure tracy deps conform to compatibility table][11331]
- [Describe purpose of bevy_diagnostic][11327]
- [Add support for updating the tracing subscriber in LogPlugin][10822]
- [Replace `DiagnosticId` by `DiagnosticPath`][9266]
- [fix link to tracy][11521]
- [Fix sysinfo CPU brand output][11850]

## A-Rendering + A-ECS

- [Explain where rendering is][11018]

## A-Assets + A-Math

- [Use glam for computing gLTF node transform][11361]

[4482]: https://github.com/bevyengine/bevy/pull/4482
[5103]: https://github.com/bevyengine/bevy/pull/5103
[6773]: https://github.com/bevyengine/bevy/pull/6773
[7650]: https://github.com/bevyengine/bevy/pull/7650
[8112]: https://github.com/bevyengine/bevy/pull/8112
[8453]: https://github.com/bevyengine/bevy/pull/8453
[9046]: https://github.com/bevyengine/bevy/pull/9046
[9172]: https://github.com/bevyengine/bevy/pull/9172
[9266]: https://github.com/bevyengine/bevy/pull/9266
[9623]: https://github.com/bevyengine/bevy/pull/9623
[9774]: https://github.com/bevyengine/bevy/pull/9774
[9797]: https://github.com/bevyengine/bevy/pull/9797
[9822]: https://github.com/bevyengine/bevy/pull/9822
[9907]: https://github.com/bevyengine/bevy/pull/9907
[9918]: https://github.com/bevyengine/bevy/pull/9918
[9920]: https://github.com/bevyengine/bevy/pull/9920
[9943]: https://github.com/bevyengine/bevy/pull/9943
[10011]: https://github.com/bevyengine/bevy/pull/10011
[10073]: https://github.com/bevyengine/bevy/pull/10073
[10077]: https://github.com/bevyengine/bevy/pull/10077
[10153]: https://github.com/bevyengine/bevy/pull/10153
[10224]: https://github.com/bevyengine/bevy/pull/10224
[10231]: https://github.com/bevyengine/bevy/pull/10231
[10266]: https://github.com/bevyengine/bevy/pull/10266
[10268]: https://github.com/bevyengine/bevy/pull/10268
[10321]: https://github.com/bevyengine/bevy/pull/10321
[10325]: https://github.com/bevyengine/bevy/pull/10325
[10336]: https://github.com/bevyengine/bevy/pull/10336
[10337]: https://github.com/bevyengine/bevy/pull/10337
[10342]: https://github.com/bevyengine/bevy/pull/10342
[10368]: https://github.com/bevyengine/bevy/pull/10368
[10369]: https://github.com/bevyengine/bevy/pull/10369
[10370]: https://github.com/bevyengine/bevy/pull/10370
[10376]: https://github.com/bevyengine/bevy/pull/10376
[10378]: https://github.com/bevyengine/bevy/pull/10378
[10380]: https://github.com/bevyengine/bevy/pull/10380
[10383]: https://github.com/bevyengine/bevy/pull/10383
[10389]: https://github.com/bevyengine/bevy/pull/10389
[10394]: https://github.com/bevyengine/bevy/pull/10394
[10400]: https://github.com/bevyengine/bevy/pull/10400
[10408]: https://github.com/bevyengine/bevy/pull/10408
[10410]: https://github.com/bevyengine/bevy/pull/10410
[10411]: https://github.com/bevyengine/bevy/pull/10411
[10415]: https://github.com/bevyengine/bevy/pull/10415
[10419]: https://github.com/bevyengine/bevy/pull/10419
[10421]: https://github.com/bevyengine/bevy/pull/10421
[10422]: https://github.com/bevyengine/bevy/pull/10422
[10423]: https://github.com/bevyengine/bevy/pull/10423
[10424]: https://github.com/bevyengine/bevy/pull/10424
[10434]: https://github.com/bevyengine/bevy/pull/10434
[10441]: https://github.com/bevyengine/bevy/pull/10441
[10442]: https://github.com/bevyengine/bevy/pull/10442
[10445]: https://github.com/bevyengine/bevy/pull/10445
[10446]: https://github.com/bevyengine/bevy/pull/10446
[10448]: https://github.com/bevyengine/bevy/pull/10448
[10450]: https://github.com/bevyengine/bevy/pull/10450
[10452]: https://github.com/bevyengine/bevy/pull/10452
[10453]: https://github.com/bevyengine/bevy/pull/10453
[10454]: https://github.com/bevyengine/bevy/pull/10454
[10461]: https://github.com/bevyengine/bevy/pull/10461
[10466]: https://github.com/bevyengine/bevy/pull/10466
[10474]: https://github.com/bevyengine/bevy/pull/10474
[10475]: https://github.com/bevyengine/bevy/pull/10475
[10483]: https://github.com/bevyengine/bevy/pull/10483
[10484]: https://github.com/bevyengine/bevy/pull/10484
[10485]: https://github.com/bevyengine/bevy/pull/10485
[10486]: https://github.com/bevyengine/bevy/pull/10486
[10493]: https://github.com/bevyengine/bevy/pull/10493
[10494]: https://github.com/bevyengine/bevy/pull/10494
[10496]: https://github.com/bevyengine/bevy/pull/10496
[10502]: https://github.com/bevyengine/bevy/pull/10502
[10514]: https://github.com/bevyengine/bevy/pull/10514
[10516]: https://github.com/bevyengine/bevy/pull/10516
[10517]: https://github.com/bevyengine/bevy/pull/10517
[10519]: https://github.com/bevyengine/bevy/pull/10519
[10520]: https://github.com/bevyengine/bevy/pull/10520
[10523]: https://github.com/bevyengine/bevy/pull/10523
[10526]: https://github.com/bevyengine/bevy/pull/10526
[10527]: https://github.com/bevyengine/bevy/pull/10527
[10529]: https://github.com/bevyengine/bevy/pull/10529
[10534]: https://github.com/bevyengine/bevy/pull/10534
[10537]: https://github.com/bevyengine/bevy/pull/10537
[10539]: https://github.com/bevyengine/bevy/pull/10539
[10540]: https://github.com/bevyengine/bevy/pull/10540
[10542]: https://github.com/bevyengine/bevy/pull/10542
[10543]: https://github.com/bevyengine/bevy/pull/10543
[10544]: https://github.com/bevyengine/bevy/pull/10544
[10547]: https://github.com/bevyengine/bevy/pull/10547
[10548]: https://github.com/bevyengine/bevy/pull/10548
[10550]: https://github.com/bevyengine/bevy/pull/10550
[10551]: https://github.com/bevyengine/bevy/pull/10551
[10555]: https://github.com/bevyengine/bevy/pull/10555
[10556]: https://github.com/bevyengine/bevy/pull/10556
[10558]: https://github.com/bevyengine/bevy/pull/10558
[10559]: https://github.com/bevyengine/bevy/pull/10559
[10560]: https://github.com/bevyengine/bevy/pull/10560
[10561]: https://github.com/bevyengine/bevy/pull/10561
[10562]: https://github.com/bevyengine/bevy/pull/10562
[10563]: https://github.com/bevyengine/bevy/pull/10563
[10564]: https://github.com/bevyengine/bevy/pull/10564
[10565]: https://github.com/bevyengine/bevy/pull/10565
[10574]: https://github.com/bevyengine/bevy/pull/10574
[10577]: https://github.com/bevyengine/bevy/pull/10577
[10578]: https://github.com/bevyengine/bevy/pull/10578
[10580]: https://github.com/bevyengine/bevy/pull/10580
[10585]: https://github.com/bevyengine/bevy/pull/10585
[10588]: https://github.com/bevyengine/bevy/pull/10588
[10589]: https://github.com/bevyengine/bevy/pull/10589
[10591]: https://github.com/bevyengine/bevy/pull/10591
[10592]: https://github.com/bevyengine/bevy/pull/10592
[10594]: https://github.com/bevyengine/bevy/pull/10594
[10596]: https://github.com/bevyengine/bevy/pull/10596
[10598]: https://github.com/bevyengine/bevy/pull/10598
[10601]: https://github.com/bevyengine/bevy/pull/10601
[10608]: https://github.com/bevyengine/bevy/pull/10608
[10609]: https://github.com/bevyengine/bevy/pull/10609
[10610]: https://github.com/bevyengine/bevy/pull/10610
[10611]: https://github.com/bevyengine/bevy/pull/10611
[10614]: https://github.com/bevyengine/bevy/pull/10614
[10619]: https://github.com/bevyengine/bevy/pull/10619
[10620]: https://github.com/bevyengine/bevy/pull/10620
[10623]: https://github.com/bevyengine/bevy/pull/10623
[10624]: https://github.com/bevyengine/bevy/pull/10624
[10627]: https://github.com/bevyengine/bevy/pull/10627
[10628]: https://github.com/bevyengine/bevy/pull/10628
[10631]: https://github.com/bevyengine/bevy/pull/10631
[10632]: https://github.com/bevyengine/bevy/pull/10632
[10633]: https://github.com/bevyengine/bevy/pull/10633
[10638]: https://github.com/bevyengine/bevy/pull/10638
[10640]: https://github.com/bevyengine/bevy/pull/10640
[10643]: https://github.com/bevyengine/bevy/pull/10643
[10644]: https://github.com/bevyengine/bevy/pull/10644
[10646]: https://github.com/bevyengine/bevy/pull/10646
[10648]: https://github.com/bevyengine/bevy/pull/10648
[10650]: https://github.com/bevyengine/bevy/pull/10650
[10651]: https://github.com/bevyengine/bevy/pull/10651
[10653]: https://github.com/bevyengine/bevy/pull/10653
[10657]: https://github.com/bevyengine/bevy/pull/10657
[10660]: https://github.com/bevyengine/bevy/pull/10660
[10663]: https://github.com/bevyengine/bevy/pull/10663
[10670]: https://github.com/bevyengine/bevy/pull/10670
[10672]: https://github.com/bevyengine/bevy/pull/10672
[10674]: https://github.com/bevyengine/bevy/pull/10674
[10675]: https://github.com/bevyengine/bevy/pull/10675
[10681]: https://github.com/bevyengine/bevy/pull/10681
[10682]: https://github.com/bevyengine/bevy/pull/10682
[10683]: https://github.com/bevyengine/bevy/pull/10683
[10684]: https://github.com/bevyengine/bevy/pull/10684
[10691]: https://github.com/bevyengine/bevy/pull/10691
[10692]: https://github.com/bevyengine/bevy/pull/10692
[10693]: https://github.com/bevyengine/bevy/pull/10693
[10698]: https://github.com/bevyengine/bevy/pull/10698
[10699]: https://github.com/bevyengine/bevy/pull/10699
[10702]: https://github.com/bevyengine/bevy/pull/10702
[10705]: https://github.com/bevyengine/bevy/pull/10705
[10706]: https://github.com/bevyengine/bevy/pull/10706
[10712]: https://github.com/bevyengine/bevy/pull/10712
[10714]: https://github.com/bevyengine/bevy/pull/10714
[10719]: https://github.com/bevyengine/bevy/pull/10719
[10721]: https://github.com/bevyengine/bevy/pull/10721
[10722]: https://github.com/bevyengine/bevy/pull/10722
[10723]: https://github.com/bevyengine/bevy/pull/10723
[10730]: https://github.com/bevyengine/bevy/pull/10730
[10733]: https://github.com/bevyengine/bevy/pull/10733
[10734]: https://github.com/bevyengine/bevy/pull/10734
[10735]: https://github.com/bevyengine/bevy/pull/10735
[10736]: https://github.com/bevyengine/bevy/pull/10736
[10739]: https://github.com/bevyengine/bevy/pull/10739
[10740]: https://github.com/bevyengine/bevy/pull/10740
[10741]: https://github.com/bevyengine/bevy/pull/10741
[10742]: https://github.com/bevyengine/bevy/pull/10742
[10745]: https://github.com/bevyengine/bevy/pull/10745
[10746]: https://github.com/bevyengine/bevy/pull/10746
[10748]: https://github.com/bevyengine/bevy/pull/10748
[10749]: https://github.com/bevyengine/bevy/pull/10749
[10755]: https://github.com/bevyengine/bevy/pull/10755
[10758]: https://github.com/bevyengine/bevy/pull/10758
[10764]: https://github.com/bevyengine/bevy/pull/10764
[10770]: https://github.com/bevyengine/bevy/pull/10770
[10771]: https://github.com/bevyengine/bevy/pull/10771
[10774]: https://github.com/bevyengine/bevy/pull/10774
[10779]: https://github.com/bevyengine/bevy/pull/10779
[10780]: https://github.com/bevyengine/bevy/pull/10780
[10782]: https://github.com/bevyengine/bevy/pull/10782
[10785]: https://github.com/bevyengine/bevy/pull/10785
[10787]: https://github.com/bevyengine/bevy/pull/10787
[10788]: https://github.com/bevyengine/bevy/pull/10788
[10791]: https://github.com/bevyengine/bevy/pull/10791
[10799]: https://github.com/bevyengine/bevy/pull/10799
[10801]: https://github.com/bevyengine/bevy/pull/10801
[10804]: https://github.com/bevyengine/bevy/pull/10804
[10808]: https://github.com/bevyengine/bevy/pull/10808
[10811]: https://github.com/bevyengine/bevy/pull/10811
[10812]: https://github.com/bevyengine/bevy/pull/10812
[10815]: https://github.com/bevyengine/bevy/pull/10815
[10818]: https://github.com/bevyengine/bevy/pull/10818
[10822]: https://github.com/bevyengine/bevy/pull/10822
[10827]: https://github.com/bevyengine/bevy/pull/10827
[10836]: https://github.com/bevyengine/bevy/pull/10836
[10837]: https://github.com/bevyengine/bevy/pull/10837
[10841]: https://github.com/bevyengine/bevy/pull/10841
[10847]: https://github.com/bevyengine/bevy/pull/10847
[10848]: https://github.com/bevyengine/bevy/pull/10848
[10854]: https://github.com/bevyengine/bevy/pull/10854
[10856]: https://github.com/bevyengine/bevy/pull/10856
[10857]: https://github.com/bevyengine/bevy/pull/10857
[10859]: https://github.com/bevyengine/bevy/pull/10859
[10872]: https://github.com/bevyengine/bevy/pull/10872
[10873]: https://github.com/bevyengine/bevy/pull/10873
[10878]: https://github.com/bevyengine/bevy/pull/10878
[10879]: https://github.com/bevyengine/bevy/pull/10879
[10882]: https://github.com/bevyengine/bevy/pull/10882
[10884]: https://github.com/bevyengine/bevy/pull/10884
[10885]: https://github.com/bevyengine/bevy/pull/10885
[10889]: https://github.com/bevyengine/bevy/pull/10889
[10892]: https://github.com/bevyengine/bevy/pull/10892
[10893]: https://github.com/bevyengine/bevy/pull/10893
[10897]: https://github.com/bevyengine/bevy/pull/10897
[10905]: https://github.com/bevyengine/bevy/pull/10905
[10906]: https://github.com/bevyengine/bevy/pull/10906
[10910]: https://github.com/bevyengine/bevy/pull/10910
[10911]: https://github.com/bevyengine/bevy/pull/10911
[10914]: https://github.com/bevyengine/bevy/pull/10914
[10918]: https://github.com/bevyengine/bevy/pull/10918
[10920]: https://github.com/bevyengine/bevy/pull/10920
[10922]: https://github.com/bevyengine/bevy/pull/10922
[10927]: https://github.com/bevyengine/bevy/pull/10927
[10930]: https://github.com/bevyengine/bevy/pull/10930
[10937]: https://github.com/bevyengine/bevy/pull/10937
[10939]: https://github.com/bevyengine/bevy/pull/10939
[10941]: https://github.com/bevyengine/bevy/pull/10941
[10942]: https://github.com/bevyengine/bevy/pull/10942
[10943]: https://github.com/bevyengine/bevy/pull/10943
[10946]: https://github.com/bevyengine/bevy/pull/10946
[10947]: https://github.com/bevyengine/bevy/pull/10947
[10951]: https://github.com/bevyengine/bevy/pull/10951
[10961]: https://github.com/bevyengine/bevy/pull/10961
[10962]: https://github.com/bevyengine/bevy/pull/10962
[10963]: https://github.com/bevyengine/bevy/pull/10963
[10965]: https://github.com/bevyengine/bevy/pull/10965
[10966]: https://github.com/bevyengine/bevy/pull/10966
[10967]: https://github.com/bevyengine/bevy/pull/10967
[10969]: https://github.com/bevyengine/bevy/pull/10969
[10970]: https://github.com/bevyengine/bevy/pull/10970
[10971]: https://github.com/bevyengine/bevy/pull/10971
[10977]: https://github.com/bevyengine/bevy/pull/10977
[10979]: https://github.com/bevyengine/bevy/pull/10979
[10980]: https://github.com/bevyengine/bevy/pull/10980
[10988]: https://github.com/bevyengine/bevy/pull/10988
[10989]: https://github.com/bevyengine/bevy/pull/10989
[10990]: https://github.com/bevyengine/bevy/pull/10990
[10996]: https://github.com/bevyengine/bevy/pull/10996
[10998]: https://github.com/bevyengine/bevy/pull/10998
[10999]: https://github.com/bevyengine/bevy/pull/10999
[11002]: https://github.com/bevyengine/bevy/pull/11002
[11003]: https://github.com/bevyengine/bevy/pull/11003
[11006]: https://github.com/bevyengine/bevy/pull/11006
[11010]: https://github.com/bevyengine/bevy/pull/11010
[11011]: https://github.com/bevyengine/bevy/pull/11011
[11012]: https://github.com/bevyengine/bevy/pull/11012
[11014]: https://github.com/bevyengine/bevy/pull/11014
[11016]: https://github.com/bevyengine/bevy/pull/11016
[11017]: https://github.com/bevyengine/bevy/pull/11017
[11018]: https://github.com/bevyengine/bevy/pull/11018
[11020]: https://github.com/bevyengine/bevy/pull/11020
[11021]: https://github.com/bevyengine/bevy/pull/11021
[11023]: https://github.com/bevyengine/bevy/pull/11023
[11024]: https://github.com/bevyengine/bevy/pull/11024
[11025]: https://github.com/bevyengine/bevy/pull/11025
[11027]: https://github.com/bevyengine/bevy/pull/11027
[11028]: https://github.com/bevyengine/bevy/pull/11028
[11029]: https://github.com/bevyengine/bevy/pull/11029
[11030]: https://github.com/bevyengine/bevy/pull/11030
[11031]: https://github.com/bevyengine/bevy/pull/11031
[11037]: https://github.com/bevyengine/bevy/pull/11037
[11040]: https://github.com/bevyengine/bevy/pull/11040
[11041]: https://github.com/bevyengine/bevy/pull/11041
[11043]: https://github.com/bevyengine/bevy/pull/11043
[11045]: https://github.com/bevyengine/bevy/pull/11045
[11049]: https://github.com/bevyengine/bevy/pull/11049
[11051]: https://github.com/bevyengine/bevy/pull/11051
[11053]: https://github.com/bevyengine/bevy/pull/11053
[11054]: https://github.com/bevyengine/bevy/pull/11054
[11057]: https://github.com/bevyengine/bevy/pull/11057
[11058]: https://github.com/bevyengine/bevy/pull/11058
[11059]: https://github.com/bevyengine/bevy/pull/11059
[11060]: https://github.com/bevyengine/bevy/pull/11060
[11063]: https://github.com/bevyengine/bevy/pull/11063
[11064]: https://github.com/bevyengine/bevy/pull/11064
[11065]: https://github.com/bevyengine/bevy/pull/11065
[11068]: https://github.com/bevyengine/bevy/pull/11068
[11069]: https://github.com/bevyengine/bevy/pull/11069
[11071]: https://github.com/bevyengine/bevy/pull/11071
[11072]: https://github.com/bevyengine/bevy/pull/11072
[11075]: https://github.com/bevyengine/bevy/pull/11075
[11076]: https://github.com/bevyengine/bevy/pull/11076
[11077]: https://github.com/bevyengine/bevy/pull/11077
[11080]: https://github.com/bevyengine/bevy/pull/11080
[11082]: https://github.com/bevyengine/bevy/pull/11082
[11084]: https://github.com/bevyengine/bevy/pull/11084
[11089]: https://github.com/bevyengine/bevy/pull/11089
[11092]: https://github.com/bevyengine/bevy/pull/11092
[11095]: https://github.com/bevyengine/bevy/pull/11095
[11103]: https://github.com/bevyengine/bevy/pull/11103
[11104]: https://github.com/bevyengine/bevy/pull/11104
[11106]: https://github.com/bevyengine/bevy/pull/11106
[11107]: https://github.com/bevyengine/bevy/pull/11107
[11110]: https://github.com/bevyengine/bevy/pull/11110
[11118]: https://github.com/bevyengine/bevy/pull/11118
[11128]: https://github.com/bevyengine/bevy/pull/11128
[11129]: https://github.com/bevyengine/bevy/pull/11129
[11134]: https://github.com/bevyengine/bevy/pull/11134
[11138]: https://github.com/bevyengine/bevy/pull/11138
[11139]: https://github.com/bevyengine/bevy/pull/11139
[11140]: https://github.com/bevyengine/bevy/pull/11140
[11142]: https://github.com/bevyengine/bevy/pull/11142
[11143]: https://github.com/bevyengine/bevy/pull/11143
[11146]: https://github.com/bevyengine/bevy/pull/11146
[11149]: https://github.com/bevyengine/bevy/pull/11149
[11152]: https://github.com/bevyengine/bevy/pull/11152
[11153]: https://github.com/bevyengine/bevy/pull/11153
[11162]: https://github.com/bevyengine/bevy/pull/11162
[11163]: https://github.com/bevyengine/bevy/pull/11163
[11164]: https://github.com/bevyengine/bevy/pull/11164
[11166]: https://github.com/bevyengine/bevy/pull/11166
[11167]: https://github.com/bevyengine/bevy/pull/11167
[11170]: https://github.com/bevyengine/bevy/pull/11170
[11171]: https://github.com/bevyengine/bevy/pull/11171
[11172]: https://github.com/bevyengine/bevy/pull/11172
[11176]: https://github.com/bevyengine/bevy/pull/11176
[11178]: https://github.com/bevyengine/bevy/pull/11178
[11179]: https://github.com/bevyengine/bevy/pull/11179
[11180]: https://github.com/bevyengine/bevy/pull/11180
[11182]: https://github.com/bevyengine/bevy/pull/11182
[11188]: https://github.com/bevyengine/bevy/pull/11188
[11189]: https://github.com/bevyengine/bevy/pull/11189
[11191]: https://github.com/bevyengine/bevy/pull/11191
[11192]: https://github.com/bevyengine/bevy/pull/11192
[11193]: https://github.com/bevyengine/bevy/pull/11193
[11194]: https://github.com/bevyengine/bevy/pull/11194
[11195]: https://github.com/bevyengine/bevy/pull/11195
[11199]: https://github.com/bevyengine/bevy/pull/11199
[11205]: https://github.com/bevyengine/bevy/pull/11205
[11206]: https://github.com/bevyengine/bevy/pull/11206
[11212]: https://github.com/bevyengine/bevy/pull/11212
[11218]: https://github.com/bevyengine/bevy/pull/11218
[11226]: https://github.com/bevyengine/bevy/pull/11226
[11227]: https://github.com/bevyengine/bevy/pull/11227
[11234]: https://github.com/bevyengine/bevy/pull/11234
[11238]: https://github.com/bevyengine/bevy/pull/11238
[11239]: https://github.com/bevyengine/bevy/pull/11239
[11242]: https://github.com/bevyengine/bevy/pull/11242
[11243]: https://github.com/bevyengine/bevy/pull/11243
[11245]: https://github.com/bevyengine/bevy/pull/11245
[11248]: https://github.com/bevyengine/bevy/pull/11248
[11250]: https://github.com/bevyengine/bevy/pull/11250
[11254]: https://github.com/bevyengine/bevy/pull/11254
[11260]: https://github.com/bevyengine/bevy/pull/11260
[11261]: https://github.com/bevyengine/bevy/pull/11261
[11268]: https://github.com/bevyengine/bevy/pull/11268
[11269]: https://github.com/bevyengine/bevy/pull/11269
[11270]: https://github.com/bevyengine/bevy/pull/11270
[11280]: https://github.com/bevyengine/bevy/pull/11280
[11284]: https://github.com/bevyengine/bevy/pull/11284
[11289]: https://github.com/bevyengine/bevy/pull/11289
[11292]: https://github.com/bevyengine/bevy/pull/11292
[11293]: https://github.com/bevyengine/bevy/pull/11293
[11296]: https://github.com/bevyengine/bevy/pull/11296
[11305]: https://github.com/bevyengine/bevy/pull/11305
[11306]: https://github.com/bevyengine/bevy/pull/11306
[11307]: https://github.com/bevyengine/bevy/pull/11307
[11310]: https://github.com/bevyengine/bevy/pull/11310
[11313]: https://github.com/bevyengine/bevy/pull/11313
[11316]: https://github.com/bevyengine/bevy/pull/11316
[11319]: https://github.com/bevyengine/bevy/pull/11319
[11321]: https://github.com/bevyengine/bevy/pull/11321
[11323]: https://github.com/bevyengine/bevy/pull/11323
[11325]: https://github.com/bevyengine/bevy/pull/11325
[11326]: https://github.com/bevyengine/bevy/pull/11326
[11327]: https://github.com/bevyengine/bevy/pull/11327
[11330]: https://github.com/bevyengine/bevy/pull/11330
[11331]: https://github.com/bevyengine/bevy/pull/11331
[11332]: https://github.com/bevyengine/bevy/pull/11332
[11334]: https://github.com/bevyengine/bevy/pull/11334
[11335]: https://github.com/bevyengine/bevy/pull/11335
[11336]: https://github.com/bevyengine/bevy/pull/11336
[11338]: https://github.com/bevyengine/bevy/pull/11338
[11342]: https://github.com/bevyengine/bevy/pull/11342
[11347]: https://github.com/bevyengine/bevy/pull/11347
[11353]: https://github.com/bevyengine/bevy/pull/11353
[11360]: https://github.com/bevyengine/bevy/pull/11360
[11361]: https://github.com/bevyengine/bevy/pull/11361
[11366]: https://github.com/bevyengine/bevy/pull/11366
[11368]: https://github.com/bevyengine/bevy/pull/11368
[11369]: https://github.com/bevyengine/bevy/pull/11369
[11370]: https://github.com/bevyengine/bevy/pull/11370
[11371]: https://github.com/bevyengine/bevy/pull/11371
[11373]: https://github.com/bevyengine/bevy/pull/11373
[11379]: https://github.com/bevyengine/bevy/pull/11379
[11381]: https://github.com/bevyengine/bevy/pull/11381
[11383]: https://github.com/bevyengine/bevy/pull/11383
[11386]: https://github.com/bevyengine/bevy/pull/11386
[11388]: https://github.com/bevyengine/bevy/pull/11388
[11389]: https://github.com/bevyengine/bevy/pull/11389
[11391]: https://github.com/bevyengine/bevy/pull/11391
[11397]: https://github.com/bevyengine/bevy/pull/11397
[11399]: https://github.com/bevyengine/bevy/pull/11399
[11400]: https://github.com/bevyengine/bevy/pull/11400
[11403]: https://github.com/bevyengine/bevy/pull/11403
[11404]: https://github.com/bevyengine/bevy/pull/11404
[11405]: https://github.com/bevyengine/bevy/pull/11405
[11407]: https://github.com/bevyengine/bevy/pull/11407
[11412]: https://github.com/bevyengine/bevy/pull/11412
[11416]: https://github.com/bevyengine/bevy/pull/11416
[11417]: https://github.com/bevyengine/bevy/pull/11417
[11418]: https://github.com/bevyengine/bevy/pull/11418
[11419]: https://github.com/bevyengine/bevy/pull/11419
[11420]: https://github.com/bevyengine/bevy/pull/11420
[11421]: https://github.com/bevyengine/bevy/pull/11421
[11422]: https://github.com/bevyengine/bevy/pull/11422
[11425]: https://github.com/bevyengine/bevy/pull/11425
[11428]: https://github.com/bevyengine/bevy/pull/11428
[11431]: https://github.com/bevyengine/bevy/pull/11431
[11432]: https://github.com/bevyengine/bevy/pull/11432
[11433]: https://github.com/bevyengine/bevy/pull/11433
[11434]: https://github.com/bevyengine/bevy/pull/11434
[11435]: https://github.com/bevyengine/bevy/pull/11435
[11436]: https://github.com/bevyengine/bevy/pull/11436
[11437]: https://github.com/bevyengine/bevy/pull/11437
[11439]: https://github.com/bevyengine/bevy/pull/11439
[11440]: https://github.com/bevyengine/bevy/pull/11440
[11442]: https://github.com/bevyengine/bevy/pull/11442
[11444]: https://github.com/bevyengine/bevy/pull/11444
[11445]: https://github.com/bevyengine/bevy/pull/11445
[11447]: https://github.com/bevyengine/bevy/pull/11447
[11454]: https://github.com/bevyengine/bevy/pull/11454
[11455]: https://github.com/bevyengine/bevy/pull/11455
[11456]: https://github.com/bevyengine/bevy/pull/11456
[11461]: https://github.com/bevyengine/bevy/pull/11461
[11462]: https://github.com/bevyengine/bevy/pull/11462
[11467]: https://github.com/bevyengine/bevy/pull/11467
[11469]: https://github.com/bevyengine/bevy/pull/11469
[11474]: https://github.com/bevyengine/bevy/pull/11474
[11480]: https://github.com/bevyengine/bevy/pull/11480
[11483]: https://github.com/bevyengine/bevy/pull/11483
[11484]: https://github.com/bevyengine/bevy/pull/11484
[11486]: https://github.com/bevyengine/bevy/pull/11486
[11487]: https://github.com/bevyengine/bevy/pull/11487
[11489]: https://github.com/bevyengine/bevy/pull/11489
[11491]: https://github.com/bevyengine/bevy/pull/11491
[11497]: https://github.com/bevyengine/bevy/pull/11497
[11498]: https://github.com/bevyengine/bevy/pull/11498
[11499]: https://github.com/bevyengine/bevy/pull/11499
[11500]: https://github.com/bevyengine/bevy/pull/11500
[11504]: https://github.com/bevyengine/bevy/pull/11504
[11506]: https://github.com/bevyengine/bevy/pull/11506
[11507]: https://github.com/bevyengine/bevy/pull/11507
[11508]: https://github.com/bevyengine/bevy/pull/11508
[11512]: https://github.com/bevyengine/bevy/pull/11512
[11513]: https://github.com/bevyengine/bevy/pull/11513
[11514]: https://github.com/bevyengine/bevy/pull/11514
[11519]: https://github.com/bevyengine/bevy/pull/11519
[11521]: https://github.com/bevyengine/bevy/pull/11521
[11523]: https://github.com/bevyengine/bevy/pull/11523
[11524]: https://github.com/bevyengine/bevy/pull/11524
[11526]: https://github.com/bevyengine/bevy/pull/11526
[11527]: https://github.com/bevyengine/bevy/pull/11527
[11528]: https://github.com/bevyengine/bevy/pull/11528
[11529]: https://github.com/bevyengine/bevy/pull/11529
[11531]: https://github.com/bevyengine/bevy/pull/11531
[11534]: https://github.com/bevyengine/bevy/pull/11534
[11538]: https://github.com/bevyengine/bevy/pull/11538
[11540]: https://github.com/bevyengine/bevy/pull/11540
[11541]: https://github.com/bevyengine/bevy/pull/11541
[11543]: https://github.com/bevyengine/bevy/pull/11543
[11548]: https://github.com/bevyengine/bevy/pull/11548
[11551]: https://github.com/bevyengine/bevy/pull/11551
[11555]: https://github.com/bevyengine/bevy/pull/11555
[11556]: https://github.com/bevyengine/bevy/pull/11556
[11560]: https://github.com/bevyengine/bevy/pull/11560
[11561]: https://github.com/bevyengine/bevy/pull/11561
[11573]: https://github.com/bevyengine/bevy/pull/11573
[11574]: https://github.com/bevyengine/bevy/pull/11574
[11575]: https://github.com/bevyengine/bevy/pull/11575
[11576]: https://github.com/bevyengine/bevy/pull/11576
[11578]: https://github.com/bevyengine/bevy/pull/11578
[11580]: https://github.com/bevyengine/bevy/pull/11580
[11581]: https://github.com/bevyengine/bevy/pull/11581
[11583]: https://github.com/bevyengine/bevy/pull/11583
[11585]: https://github.com/bevyengine/bevy/pull/11585
[11586]: https://github.com/bevyengine/bevy/pull/11586
[11591]: https://github.com/bevyengine/bevy/pull/11591
[11596]: https://github.com/bevyengine/bevy/pull/11596
[11597]: https://github.com/bevyengine/bevy/pull/11597
[11599]: https://github.com/bevyengine/bevy/pull/11599
[11600]: https://github.com/bevyengine/bevy/pull/11600
[11604]: https://github.com/bevyengine/bevy/pull/11604
[11605]: https://github.com/bevyengine/bevy/pull/11605
[11610]: https://github.com/bevyengine/bevy/pull/11610
[11611]: https://github.com/bevyengine/bevy/pull/11611
[11612]: https://github.com/bevyengine/bevy/pull/11612
[11615]: https://github.com/bevyengine/bevy/pull/11615
[11616]: https://github.com/bevyengine/bevy/pull/11616
[11617]: https://github.com/bevyengine/bevy/pull/11617
[11618]: https://github.com/bevyengine/bevy/pull/11618
[11622]: https://github.com/bevyengine/bevy/pull/11622
[11626]: https://github.com/bevyengine/bevy/pull/11626
[11627]: https://github.com/bevyengine/bevy/pull/11627
[11630]: https://github.com/bevyengine/bevy/pull/11630
[11635]: https://github.com/bevyengine/bevy/pull/11635
[11639]: https://github.com/bevyengine/bevy/pull/11639
[11640]: https://github.com/bevyengine/bevy/pull/11640
[11641]: https://github.com/bevyengine/bevy/pull/11641
[11644]: https://github.com/bevyengine/bevy/pull/11644
[11645]: https://github.com/bevyengine/bevy/pull/11645
[11649]: https://github.com/bevyengine/bevy/pull/11649
[11650]: https://github.com/bevyengine/bevy/pull/11650
[11652]: https://github.com/bevyengine/bevy/pull/11652
[11660]: https://github.com/bevyengine/bevy/pull/11660
[11662]: https://github.com/bevyengine/bevy/pull/11662
[11664]: https://github.com/bevyengine/bevy/pull/11664
[11666]: https://github.com/bevyengine/bevy/pull/11666
[11669]: https://github.com/bevyengine/bevy/pull/11669
[11671]: https://github.com/bevyengine/bevy/pull/11671
[11672]: https://github.com/bevyengine/bevy/pull/11672
[11675]: https://github.com/bevyengine/bevy/pull/11675
[11676]: https://github.com/bevyengine/bevy/pull/11676
[11678]: https://github.com/bevyengine/bevy/pull/11678
[11684]: https://github.com/bevyengine/bevy/pull/11684
[11686]: https://github.com/bevyengine/bevy/pull/11686
[11687]: https://github.com/bevyengine/bevy/pull/11687
[11688]: https://github.com/bevyengine/bevy/pull/11688
[11690]: https://github.com/bevyengine/bevy/pull/11690
[11693]: https://github.com/bevyengine/bevy/pull/11693
[11697]: https://github.com/bevyengine/bevy/pull/11697
[11699]: https://github.com/bevyengine/bevy/pull/11699
[11700]: https://github.com/bevyengine/bevy/pull/11700
[11703]: https://github.com/bevyengine/bevy/pull/11703
[11705]: https://github.com/bevyengine/bevy/pull/11705
[11709]: https://github.com/bevyengine/bevy/pull/11709
[11710]: https://github.com/bevyengine/bevy/pull/11710
[11712]: https://github.com/bevyengine/bevy/pull/11712
[11720]: https://github.com/bevyengine/bevy/pull/11720
[11721]: https://github.com/bevyengine/bevy/pull/11721
[11722]: https://github.com/bevyengine/bevy/pull/11722
[11725]: https://github.com/bevyengine/bevy/pull/11725
[11726]: https://github.com/bevyengine/bevy/pull/11726
[11728]: https://github.com/bevyengine/bevy/pull/11728
[11733]: https://github.com/bevyengine/bevy/pull/11733
[11735]: https://github.com/bevyengine/bevy/pull/11735
[11736]: https://github.com/bevyengine/bevy/pull/11736
[11737]: https://github.com/bevyengine/bevy/pull/11737
[11745]: https://github.com/bevyengine/bevy/pull/11745
[11747]: https://github.com/bevyengine/bevy/pull/11747
[11751]: https://github.com/bevyengine/bevy/pull/11751
[11758]: https://github.com/bevyengine/bevy/pull/11758
[11764]: https://github.com/bevyengine/bevy/pull/11764
[11767]: https://github.com/bevyengine/bevy/pull/11767
[11769]: https://github.com/bevyengine/bevy/pull/11769
[11773]: https://github.com/bevyengine/bevy/pull/11773
[11777]: https://github.com/bevyengine/bevy/pull/11777
[11780]: https://github.com/bevyengine/bevy/pull/11780
[11781]: https://github.com/bevyengine/bevy/pull/11781
[11783]: https://github.com/bevyengine/bevy/pull/11783
[11785]: https://github.com/bevyengine/bevy/pull/11785
[11791]: https://github.com/bevyengine/bevy/pull/11791
[11792]: https://github.com/bevyengine/bevy/pull/11792
[11795]: https://github.com/bevyengine/bevy/pull/11795
[11797]: https://github.com/bevyengine/bevy/pull/11797
[11798]: https://github.com/bevyengine/bevy/pull/11798
[11800]: https://github.com/bevyengine/bevy/pull/11800
[11801]: https://github.com/bevyengine/bevy/pull/11801
[11803]: https://github.com/bevyengine/bevy/pull/11803
[11805]: https://github.com/bevyengine/bevy/pull/11805
[11810]: https://github.com/bevyengine/bevy/pull/11810
[11818]: https://github.com/bevyengine/bevy/pull/11818
[11822]: https://github.com/bevyengine/bevy/pull/11822
[11832]: https://github.com/bevyengine/bevy/pull/11832
[11838]: https://github.com/bevyengine/bevy/pull/11838
[11847]: https://github.com/bevyengine/bevy/pull/11847
[11850]: https://github.com/bevyengine/bevy/pull/11850
[11855]: https://github.com/bevyengine/bevy/pull/11855
[11856]: https://github.com/bevyengine/bevy/pull/11856
[11860]: https://github.com/bevyengine/bevy/pull/11860
[11865]: https://github.com/bevyengine/bevy/pull/11865
[11866]: https://github.com/bevyengine/bevy/pull/11866
[11867]: https://github.com/bevyengine/bevy/pull/11867
[11868]: https://github.com/bevyengine/bevy/pull/11868
[11870]: https://github.com/bevyengine/bevy/pull/11870
[11878]: https://github.com/bevyengine/bevy/pull/11878
[11880]: https://github.com/bevyengine/bevy/pull/11880
[11882]: https://github.com/bevyengine/bevy/pull/11882
[11884]: https://github.com/bevyengine/bevy/pull/11884
[11889]: https://github.com/bevyengine/bevy/pull/11889
[11893]: https://github.com/bevyengine/bevy/pull/11893
[11894]: https://github.com/bevyengine/bevy/pull/11894
[11907]: https://github.com/bevyengine/bevy/pull/11907
[11909]: https://github.com/bevyengine/bevy/pull/11909
[11910]: https://github.com/bevyengine/bevy/pull/11910
[11911]: https://github.com/bevyengine/bevy/pull/11911
[11913]: https://github.com/bevyengine/bevy/pull/11913
[11914]: https://github.com/bevyengine/bevy/pull/11914
[11915]: https://github.com/bevyengine/bevy/pull/11915
