## Contributors

A huge thanks to the 143 contributors that made this release (and associated docs) possible! In random order:

- @makspll
- @sullyj3
- @kurtkuehnert
- @xtr3m3nerd
- @maccesch
- @maxwellodri
- @IceSentry
- @maniwani
- @lovelymono
- @Edwox
- @fishykins
- @ManevilleF
- @manokara
- @ramirezmike
- @VitalyAnkh
- @bwhitt7
- @marlyx
- @jakobhellermann
- @pascualex
- @MrGVSV
- @zicklag
- @msvbg
- @elbertronnie
- @jgoday
- @mwcz
- @cryscan
- @asherkin
- @ian-h-chamberlain
- @Ian-Yy
- @galkowskit
- @andresovela
- @Vrixyz
- @afonsolage
- @Sergi-Ferrez
- @gilescope
- @SpecificProtagonist
- @gak
- @CAD97
- @bzm3r
- @robtfm
- @Nilirad
- @pcone
- @TheRawMeatball
- @Piturnah
- @rparrett
- @tguichaoua
- @zmarlon
- @x3ro
- @CatThingy
- @shuoli84
- @rustui
- @JohnTheCoolingFan
- @robem
- @TimJentzsch
- @PROMETHIA-27
- @Lucidus115
- @leath-dub
- @targrub
- @slyedoc
- @nicopap
- @superdump
- @themasch
- @EMachad0
- @ottah
- @oceantume
- @Azervu
- @strattonbrazil
- @xgbwei
- @james7132
- @thebluefish
- @lewiszlw
- @Carter0
- @contagnas
- @hmeine
- @LarsDu
- @yrns
- @light4
- @hymm
- @cart
- @Suficio
- @dataphract
- @jiftoo
- @NathanSWard
- @Ixentus
- @BorisBoutillier
- @AlexOkafor
- @JMS55
- @lain-dono
- @Weibye
- @SleepySwords
- @bjorn3
- @SludgePhD
- @djeedai
- @harudagondi
- @CGMossa
- @merelymyself
- @james-j-obrien
- @benfrankel
- @Demiu
- @YohDeadfall
- @mahulst
- @MDeiml
- @polarvoid
- @Neo-Zhixing
- @Ptrskay3
- @BoxyUwU
- @Carlrs
- @wanderrful
- @KDecay
- @McSpidey
- @komadori
- @TehPers
- @vertesians
- @devil-ira
- @emersonmx
- @NoahShomette
- @torsteingrindvik
- @StarArawn
- @Aceeri
- @inodentry
- @DJMcNab
- @micron-mushroom
- @Shatur
- @coreh
- @mockersf
- @WaffleLapkin
- @jkb0o
- @amiani
- @hakolao
- @Moulberry
- @BeastLe9enD
- @Bleb1k
- @alice-i-cecile
- @DGriffin91
- @fvacek
- @Pietrek14
- @NiklasEi
- @0x182d4454fb211940
- @JoJoJet
- @anchpop
- @jwagner
- @github-actions[bot]
- @timokoesters

## Full Changelog

## A-Meta

- [Mention dev docs in CONTRIBUTING.md][5969]
- [Update license link in README.md][5614]

## A-Build-System

- [update allowed duplicate dependencies][6500]
- [Remove LTO][6472]
- [migrate away from actions-rs actions to dtolnay/rust-toolchain][6432]
- [ignore nanosec precision tests on apple m1][6377]
- [fix failing doc test and clear up docs][6314]
- [Disabling default features support in bevy_ecs, bevy_reflect and bevy][5993]
- [Warn when passing invalid argument to CI][5858]
- [Ignore RUSTSEC-2021-0139][5816]
- [Remove unneeded skipped crates for duplicate dependencies][5678]
- [Use latest stable version for CI 'build' job][5672]

## A-Math

- [Update glam 0.22, hexasphere 8.0, encase 0.4][6427]
- [Expose mint feature in bevy_math/glam][5857]
- [Fix documentation for looking_at/look_at][4696]

## A-Scenes

- [bevy_scene: Serialize entities to map][6416]
- [bevy_scene: Stabilize entity order in `DynamicSceneBuilder`][6382]
- [bevy_scene: Replace root list with struct][6354]
- [bevy_scene: Use map for scene `components`][6345]
- [Create a scene from a dynamic scene][6229]
- [dynamic scene builder][6227]
- [can clone a scene][5855]

- [Use default serde impls for Entity][6194]
- [scenes: simplify return type of iter_instance_entities][5994]
- [Scene example: write file in a task][5952]
- [Add writing of scene data to Scene example][5949]
- [fixes the types for Vec3 and Quat in scene example to remove WARN from the logs][5751]

## A-ECS

- [Implement `Bundle` for `Component`. Use `Bundle` tuples for insertion][2975]
- [Replace `WorldQueryGats` trait with actual gats][6319]
- [Add iter_entities to World #6228][6242]
- [Rename system chaining to system piping][6230]
- [Add methods for silencing system-order ambiguity warnings][6158]
- [Remove ambiguity sets][5916]
- [implemented #[bundle(ignore)]][6123]
- [Exclusive Systems Now Implement `System`. Flexible Exclusive System Params][6083]
- [Spawn now takes a Bundle][6054]
- [Accept Bundles for insert and remove. Deprecate insert/remove_bundle][6039]
- [Make `Resource` trait opt-in, requiring `#[derive(Resource)]` V2][5577]
- [make `WorldQuery` very flat][5205]
- [Implement IntoIterator for ECS wrapper types.][5096]
- [Nested spawns on scope][4466]

- [Add `send_event` and friends to `WorldCell`][6515]
- [Fix unsound `EntityMut::remove_children`. Add `EntityMut::world_scope`][6464]
- [Remove unnecesary branches/panics from Query accesses][6461]
- [Speed up `Query::get_many` and add benchmarks][6400]
- [Fix query.to_readonly().get_component_mut() soundness bug][6401]
- [Allow access to non-send resource through `World::resource_scope`][6113]
- [Add get_entity to Commands][5854]
- [`Query` filter types must be `ReadOnlyWorldQuery`][6008]
- [Added the ability to get or set the last change tick of a system.][5838]
- [Add a module for common system `chain`/`pipe` adapters][5776]
- [SystemParam for the name of the system you are currently in][5731]
- [Add a change detection bypass and manual control over change ticks][5635]
- [Avoid making `Fetch`s `Clone`][5593]
- [Replace `many_for_each_mut` with `iter_many_mut`.][5402]
- [Start running systems while prepare_systems is running][4919]
- [Extract Resources into their own dedicated storage][4809]
- [Clean up Fetch code][4800]
- [Skip empty archetypes and tables when iterating over queries][4724]

- [Fix spawning empty bundles][6425]
- [Improve logging consistency for entity despawning][6501]
- [`debug_checked_unwrap` should track its caller][6452]
- [Remove outdated uses of single-tuple bundles][6406]
- [document insert_non_send_resource panics][6328]
- [Fix tests breaking when new WorldQuery impls are added][6317]
- [Mention world_query(ignore) attribute for WorldQuery derivation][6309]
- [Document EntityCommands/EntityMut insert()][6270]
- [Add More Description to the Iter Combinations Documentation][6260]
- [Add a method for accessing the width of a `Table`][6249]
- [Adding Debug implementations for App, Stage, Schedule, Query, QueryState, etc.][6214]
- [Add a method for mapping `Mut<T>` -> `Mut<U>`][6199]
- [Document `EntityMut::remove()`][6168]
- [Deduplicate ambiguity reporting code][6149]
- [Update API docs for `Commands::get_or_spawn` to inform the user about invalid returned values][6117]
- [[Fixes #6059] ``Entity``'s “ID” should be named “index” instead][6107]
- [Add ambiguity detection tests][6053]
- [Clarify the behaviour of `iter_many` in the docs][5973]
- [Fix API docs for `Commands` methods][5955]
- [Move ambiguity detection into its own file][5918]
- [relax `Sized` bounds around change detection types][5917]
- [Update WorldQueryGats doc with type aliases][5898]
- [Remove ExactSizeIterator from QueryCombinationIter][5895]
- [Remove Sync bound from Command][5871]
- [#5817: derive_bundle macro is not hygienic][5835]
- [Fix example in `AnyOf` docs][5798]
- [Update `Query` methods documentation][5742]
- [Update `Query` struct docs][5741]
- [Improve `WorldQuery` docs][5740]
- [Add documentation to `QueryCombinationIter`][5739]
- [Warning message for missing events][5730]
- [Better error message for `World::resource_scope`][5727]
- [Make most `Entity` methods `const`][5688]
- [Remove an outdated workaround for `impl Trait`][5659]
- [lifetime related cleanup in `entity_ref.rs`][5611]
- [Remove `insert_resource_with_id`][5608]
- [Misc query.rs cleanup][5591]
- [drop old value in `insert_resource_by_id` if exists][5587]
- [Add into_world_mut to EntityMut][5586]
- [Fix lifetime bound on `From` impl for `NonSendMut` -> `Mut`][5560]
- [Remove `Sync` bound from `Local`][5483]
- [Add `FromWorld` bound to `T` in `Local<T>`][5481]
- [Add From<EntityMut> for EntityRef (fixes #5459)][5461]
- [Remove last uses of string-labels][5420]
- [Fixed docs for `derive(WorldQuery)`.][5283]
- [Change Detection Benchmarks][4972]
- [bevy_ecs: Use 32-bit entity ID cursor on platforms without AtomicI64][4452]
- [add `Res::clone`][4109]

## A-Reflection

- [bevy_reflect: Improve serialization format even more][5723]
- [bevy_reflect: Binary formats][6140]
- [bevy_reflect: Reflect enums][4761]
- [Implement `Debug` for dynamic types][5948]
- [bevy_reflect: Get owned fields][5728]

- [Add `reflect(skip_serializing)` which retains reflection but disables automatic serialization][5250]
- [bevy_reflect: Add `Reflect::into_reflect`][6502]
- [Add reflect_owned][6494]
- [make `register` on `TypeRegistry` idempotent][6487]
- [Enable Constructing ReflectComponent/Resource][6257]
- [Support multiple `#[reflect]`/`#[reflect_value]` + improve error messages][6237]
- [Make arrays behave like lists in reflection][5987]
- [Add `pop` method for `List` trait.][5797]

- [`Reflect` for `Tonemapping` and `ClusterConfig`][6488]
- [add `ReflectDefault` to std types][6429]
- [Add FromReflect for Visibility][6410]
- [Register `RenderLayers` type in `CameraPlugin`][6308]
- [bevy_reflect: Fix `DynamicScene` not respecting component registrations during serialization][6288]
- [bevy_reflect: Reflect doc comments][6234]
- [Reflect Default for GlobalTransform][6200]
- [Impl Reflect for PathBuf and OsString][6193]
- [Reflect Default for `ComputedVisibility` and `Handle<T>`][6187]
- [Register `Wireframe` type][6152]
- [Derive `FromReflect` for `Transform` and `GlobalTransform`][6015]
- [bevy_reflect: Update `Reflection` documentation][5841]
- [Remove extra spaces from Range reflect impls][5839]
- [Implemented `Reflect` for all the ranges][5806]
- [bevy_reflect: Remove unnecessary `Clone` bounds][5783]
- [bevy_reflect: `GetTypeRegistration` for `SmallVec<T>`][5782]
- [bevy_reflect: Fix `apply` method for `Option<T>`][5780]
- [register missing reflect types][5747]
- [bevy_reflect: Add `FromReflect` to the prelude][5720]
- [implement `Reflect` for `Input<T>`, some misc improvements to reflect value derive][5676]
- [register `Cow<'static, str>` for reflection][5664]
- [bevy_reflect: Relax bounds on `Option<T>`][5658]
- [remove `ReflectMut` in favor of `Mut<dyn Reflect>`][5630]
- [add some info from `ReflectPathError` to the error messages][5626]
- [Added reflect/from reflect impls for NonZero integer types][5556]
- [bevy_reflect: Update enum derives][5473]

## A-Input

- [feat: add GamepadInfo, expose gamepad names][6342]
- [Add getters and setters for `InputAxis` and `ButtonSettings`][6088]
- [Add a Gamepad Viewer tool to examples][6074]
- [Added keyboard scan input event][5495]

- [Derive `Reflect` + `FromReflect` for input types][6232]
- [Make TouchInput and ForceTouch serializable][6191]
- [Derived `Copy` trait for `bevy_input` events, `Serialize`/`Deserialize` for events in `bevy_input` and `bevy_windows`, `PartialEq` for events in both, and `Eq` where possible in both.][6023]
- [Support for additional gamepad buttons and axis][5853]
- [Document `gamepad.rs`][5548]
- [`Gamepad` type is `Copy`; do not require / return references to it in `Gamepads` API][5296]
- [bevy_input: Fix process touch event][4352]

## A-Assets

- [Macro for Loading Internal Binary Assets][6478]

- [Add From<String> for AssetPath<'a>][6337]
- [Add Eq & PartialEq to AssetPath][6274]
- [make Handle::<T> field id private, and replace with a getter][6176]
- [Remove `AssetServer::watch_for_changes()`][5968]
- [add `ReflectAsset` and `ReflectHandle`][5923]
- [Update to notify 5.0 stable][5865]
- [Add warning when using load_folder on web][5827]
- [Correctly parse labels with '#'][5729]
- [Add note on ordering to AssetServerSettings docs.][5706]
- [Rename Handle::as_weak() to cast_weak()][5321]

## A-Rendering

- [Add "end of main pass post processing" render graph node][6468]
- [Rework ViewTarget to better support post processing][6415]
- [separate tonemapping and upscaling passes][3425]
- [Add FXAA postprocessing][6393]
- [Bloom][6397]
- [Add multi draw indirect draw calls][6392]
- [Take DirectionalLight's GlobalTransform into account when calculating shadow map volume (not just direction)][6384]
- [Support arbitrary RenderTarget texture formats][6380]
- [Add mutating `toggle` method to `Visibility` component][6268]
- [remove mandatory mesh attributes][6127]
- [Add `Camera::viewport_to_world`][6126]
- [add support for .comp glsl shaders][6084]
- [Increase the `MAX_DIRECTIONAL_LIGHTS` from 1 to 10][6066]
- [get proper texture format after the renderer is initialized, fix #3897][5413]
- [add globals to mesh view bind group][5409]
- [Support array / cubemap / cubemap array textures in KTX2][5325]

- [Allow passing `glam` vector types as vertex attributes][6442]
- [Add globals struct to mesh2d][6222]
- [Update `wgpu` to 0.14.0, `naga` to `0.10.0`, `winit` to 0.27.4, `raw-window-handle` to 0.5.0, `ndk` to 0.7][6218]
- [Reconfigure surface on present mode change][6049]
- [Sprite: allow using a sub-region (Rect) of the image][6014]
- [Optimize rendering slow-down at high entity counts][5509]

- [Add CameraRenderGraph::set][6470]
- [Remove Camera2d in bloom example][6462]
- [Fix panic when using globals uniform in wasm builds][6460]
- [Specialize UI pipeline on "hdr-ness"][6459]
- [Fix `mesh.wgsl` error for meshes without normals][6439]
- [Respect mipmap_filter when create ImageDescriptor with linear()/nearest()][6349]
- [Resolve most remaining execution-order ambiguities][6341]
- [Use wgsl saturate][6318]
- [doc: document `PerspectiveProjection`][6310]
- [Tidy up surface creation in RenderPlugin][6276]
- [Link to `linux_dependencies.md` in the panic message when failing to detect a GPU][6261]
- [Avoid creating `SurfaceConfiguration` in `prepare_windows`][6255]
- [use bevy default texture format if the surface is not yet available][6233]
- [Call `mesh2d_tangent_local_to_world` with the right arguments][6209]
- [Fixes Camera not being serializable due to missing registrations in core functionality.][6170]
- [fix spot dir nan bug][6167]
- [Update window.rs PresentMode docs to clarify which PresentMode will panic and which will fallback][6160]
- [log pipeline cache errors earlier][6115]
- [Fix some outdated file reference comments in bevy_pbr][6111]
- [Add details about intel linux vulkan driver][6103]
- [fix #6062 incorrect links for render module docs][6099]
- [Clarify index order in TextureAtlas::from_grid][6058]
- [Merge TextureAtlas::from_grid_with_padding into TextureAtlas::from_grid through option arguments][6057]
- [use alpha mask even when unlit][6047]
- [Implement `IntoIterator` for `&Extract<P>`][6025]
- [add Debug, Copy, Clone derives to Circle][6009]
- [Fix DrawFunctionId typo][5996]
- [Add TextureFormat::Rg16Unorm support for Image and derive Resource for SpecializedComputePipelines][5991]
- [adjust cluster index for viewport origin][5947]
- [update camera projection if viewport changed][5945]
- [Ensure 2D phase items are sorted before batching][5942]
- [Document all StandardMaterial fields][5921]
- [Add more documentation and tests to collide_aabb::collide()][5910]
- [Use 3 bits of PipelineKey to store MSAA sample count][5826]
- [Helpers to check pipeline cache status][5796]
- [bevy_pbr: Fix incorrect and unnecessary normal-mapping code][5766]
- [Add explicit ordering between `update_frusta` and `camera_system`][5757]
- [fix: grammar and typo fixes in rendergraph docs][5710]
- [Limit FontAtlasSets][5708]
- [Move `sprite::Rect` into `bevy_math`][5686]
- [Make vertex colors work without textures in bevy_sprite][5685]
- [Remove unused DepthCalculation enum][5684]
- [Grammar fixes in render graph doc][5671]
- [bevy_pbr: Fix tangent and normal normalization][5666]
- [Use circle for breakout example][5657]
- [insert_attribute panic with full message][5651]
- [Fix shader syntax][5613]
- [Make internal struct `ShaderData` non-`pub`][5609]
- [use bevy_default() for texture format in post_processing][5601]
- [Document `PipelineCache` and related types][5600]
- [Remove unnecessary `use` from examples][5583]
- [Add `bevy_render::texture::ImageSettings` to prelude][5566]
- [Add `Projection` component to prelude.][5557]
- [Remove duplicate `RenderGraph` insertion to render world][5551]
- [Correctly use as_hsla_f32 in Add<Color> and AddAssign<Color>, fixes #5543][5546]
- [don't render completely transparent UI nodes][5537]
- [Add docs for arguments of various color functions][5533]
- [Sync up bevy_sprite and bevy_ui shader View struct][5531]
- [Expose `Image` conversion functions (fixes #5452)][5527]
- [Fix View by adding missing fields present in ViewUniform][5512]
- [add default direction to DirectionalLight docs][5188]
- [make TextLayoutInfo a Component][4460]
- [Document the `bevy_render::camera` module tree][3528]
- [Freeing memory held by visible entities vector][3009]

## A-App

- [Unique plugin][6411]
- [can get the settings of a plugin from the app][6372]
- [Plugins own their settings. Rework PluginGroup trait.][6336]
- [Use plugin setup for resource only used at setup time][6360]
- [Add `TimeUpdateStrategy` resource for manual `Time` updating][6159]

- [bevy_dynamic_plugin: make it possible to handle loading errors][6437]
- [Remove unused dependency from bevy_app][5894]
- [improve panic messages for add_system_to_stage and add_system_set_to_stage][5847]

## A-Diagnostics

- [Add warning when a hierarchy component is missing][5590]
- [changed diagnostics from seconds to milliseconds][5554]
- [Add Exponential Moving Average into diagnostics][4992]

## A-Tasks

- [Add `is_finished` to `Task<T>`][6444]
- [TaskPool Panic Handling][6443]
- [tick local executor][6121]

- [StreamReceiver does not need to be mutable][6119]
- [Mark `Task` as `#[must_use]`][6068]
- [Fix a small doc typo: grater -> greater][5970]
- [Swap out num_cpus for std::thread::available_parallelism][4970]

## No area label

- [fix: explicitly specify required version of async-task][6509]
- [Fix `clippy::iter_with_drain`][6485]
- [Use `cbrt()` instead of `powf(1./3.)`][6481]
- [Improve BloomSettings docs][6465]
- [Use new let-else syntax where possible][6463]
- [Fix trybuild tests broken by rust 1.65][6457]
- [Add more info to texture loading error in texture_atlas example][6456]
- [Revert "Show prelude re-exports in docs (#6448)"][6449]
- [Show prelude re-exports in docs][6448]
- [Fix doctest warnings][6447]
- [Fix return_after_run example][6420]
- [Removed web-sys from bevy_window, never used][6414]
- [Update tracing-chrome to 0.6.0][6398]
- [fix nightly clippy warnings][6395]
- [Fix "previous release tag" link in the changelog][6394]
- [Prepare for upcoming rustlang by fixing upcoming clippy warnings][6376]
- [Revert thiserror version requirement to match version for all crates.][6365]
- [Update deny configuration][6359]
- [fix: specify required trybuild patch version][6333]
- [Update clap requirement from 3.2 to 4.0][6303]
- [Fix camera ambiguity warning in IOS example][6300]
- [Update linux_dependencies.md][6205]
- [Reduced code duplication in gamepad_viewer example][6175]
- [More explicit help how to cycle the cameras][6162]
- [Example cleanup][6131]
- [Fix some grammatical errors in the docs][6109]
- [remove copyless][6100]
- [Rename shapes examples for consistency][6082]
- [Don't bundle extra transform with camera in many sprites examples][6079]
- [Fix CI issues arising from use of Rust 1.64][6067]
- [Use `SpatialBundle`/`TransformBundle` in examples][6002]
- [Fix typo in link to dev-docs][5999]
- [Optimize use statement][5992]
- [unused dep references?][5954]
- [Update to ron 0.8][5864]
- [Miscellaneous code-quality improvements.][5860]
- [Add troubleshooting for pkgconfig errors on fedora][5821]
- [fix `Quat` type name in scene example scene file][5803]
- [fix typos in examples][5711]
- [Fix for bevy CI on main - clippy safety comments on trait.][5665]
- [fix: typo in system params docs][5624]

## A-Hierarchy

- [Add methods to `Query<&Children>` and `Query<&Parent>` to iterate over descendants and ancestors][6185]
- [Add `set_parent` and `remove_parent` to `EntityCommands`][6189]

- [Fix `RemoveChildren` command][6192]
- [Fix inconsistent children removal behavior][6017]
- [Remove duplicate asserts in test][5648]
- [Make `Children` constructor `pub(crate)`.][5532]

## A-Transform

- [add serialize feature to bevy_transform][6379]
- [flaky test: put panicking system in a single threaded stage][6172]
- [Remove `Transform::apply_non_uniform_scale`][6133]
- [Rename `Transform::mul_vec3` to `transform_point` and improve docs][6132]
- [Adding transform example links to documentation][5997]
- [Add associated constant `IDENTITY` to `Transform` and friends.][5340]

## A-Windowing

- [do not set cursor grab on window creation if not asked for][6381]
- [expose window alpha mode][6331]
- [Fix outdated and badly formatted docs for `WindowDescriptor::transparent`][6329]
- [Make bevy_window and bevy_input events serializable][6180]
- [Make `raw_window_handle` field in `Window` and `ExtractedWindow` an `Option`.][6114]
- [disable window pre creation for ios][5883]
- [Support monitor selection for all window modes.][5878]
- [Remove unnecessary unsafe `Send` and `Sync` impl for `WinitWindows` on wasm.][5863]
- [Add window resizing example][5813]
- [Fix window centering when scale_factor is not 1.0][5582]
- [fix order of exit/close window systems][5558]

## A-Core

- [Add `serialize` feature to `bevy_core`][6423]
- [Clarify `Commands` API docs][5938]
- [Consistently use `PI` to specify angles in examples.][5825]
- [Add missing type registrations for bevy_math types][5758]

## A-Animation

- [Rename `play` to `start` and add new `play` method that won't overwrite the existing animation if it's already playing][6350]

- [Fix end-of-animation index OOB][6210]

## A-UI

- [Change UI coordinate system to have origin at top left corner][6000]
- [Add z-index support with a predictable UI stack][5877]
- [Add UI scaling][5814]

- [UI scaling fix][6479]
- [Cleaning up NodeBundle, and some slight UI module re-organization][6473]
- [Fix clipping in UI][6351]
- [Rename example file scaling.rs to ui_scaling.rs][6296]
- [Fixes scroll example after inverting UI Y axis][6290]
- [Fixes incorrect glyph positioning for text2d][6273]
- [Make the default background color of `NodeBundle` transparent][6211]
- [Utility methods for Val][6134]
- [Don't use the UIBundle's Transform Fields][6095]
- [Rename `UiColor`  to `BackgroundColor`][6087]
- [Register missing bevy_text types][6029]
- [Clarify `bevy::ui::Node` field and documentation][5995]
- [Add additional constructors for `UiRect` to specify values for specific fields][5988]
- [Clean up taffy nodes when UI node entities are removed][5886]
- [Add AUTO and UNDEFINED const constructors for `Size`][5761]
- [Remove `Size` and `UiRect` generics][5404]

## A-Audio

- [Add a way to toggle `AudioSink`][6321]

- [Expose rodio's Source and Sample traits in bevy_audio][6374]
- [Update rodio requirement from 0.15 to 0.16][6020]
- [Remove `Sync` requirement in `Decodable::Decoder`][5819]

## A-Time

- [add time wrapping to Time][5982]
- [Add global time scaling][5752]

- [Add FromReflect for Timer][6422]
- [elaborate on Timer docs][6385]
- [Re-add local bool `has_received_time` in `time_system`][6357]
- [better wording for time scaling docs][6340]
- [Add default implementation of Serialize and Deserialize to Timer and Stopwatch][6248]
- [Replace the `bool` argument of `Timer` with `TimerMode`][6247]
- [Fix doc for Timer::percent_left][6198]
- [Clarify that Stopwatch.reset does not affect paused state][6016]
- [Stopwatch elapsed secs f64][5978]
- [Remaining fn in Timer][5971]
- [Replace fixed timestep in `alien_cake_addict` example with timer][5760]

[6515]: https://github.com/bevyengine/bevy/pull/6515
[6509]: https://github.com/bevyengine/bevy/pull/6509
[6502]: https://github.com/bevyengine/bevy/pull/6502
[6501]: https://github.com/bevyengine/bevy/pull/6501
[6500]: https://github.com/bevyengine/bevy/pull/6500
[6494]: https://github.com/bevyengine/bevy/pull/6494
[6488]: https://github.com/bevyengine/bevy/pull/6488
[6487]: https://github.com/bevyengine/bevy/pull/6487
[6485]: https://github.com/bevyengine/bevy/pull/6485
[6481]: https://github.com/bevyengine/bevy/pull/6481
[6479]: https://github.com/bevyengine/bevy/pull/6479
[6478]: https://github.com/bevyengine/bevy/pull/6478
[6473]: https://github.com/bevyengine/bevy/pull/6473
[6472]: https://github.com/bevyengine/bevy/pull/6472
[6470]: https://github.com/bevyengine/bevy/pull/6470
[6468]: https://github.com/bevyengine/bevy/pull/6468
[6465]: https://github.com/bevyengine/bevy/pull/6465
[6464]: https://github.com/bevyengine/bevy/pull/6464
[6463]: https://github.com/bevyengine/bevy/pull/6463
[6462]: https://github.com/bevyengine/bevy/pull/6462
[6461]: https://github.com/bevyengine/bevy/pull/6461
[6460]: https://github.com/bevyengine/bevy/pull/6460
[6459]: https://github.com/bevyengine/bevy/pull/6459
[6457]: https://github.com/bevyengine/bevy/pull/6457
[6456]: https://github.com/bevyengine/bevy/pull/6456
[6452]: https://github.com/bevyengine/bevy/pull/6452
[6449]: https://github.com/bevyengine/bevy/pull/6449
[6448]: https://github.com/bevyengine/bevy/pull/6448
[6447]: https://github.com/bevyengine/bevy/pull/6447
[6444]: https://github.com/bevyengine/bevy/pull/6444
[6443]: https://github.com/bevyengine/bevy/pull/6443
[6442]: https://github.com/bevyengine/bevy/pull/6442
[6439]: https://github.com/bevyengine/bevy/pull/6439
[6437]: https://github.com/bevyengine/bevy/pull/6437
[6432]: https://github.com/bevyengine/bevy/pull/6432
[6429]: https://github.com/bevyengine/bevy/pull/6429
[6427]: https://github.com/bevyengine/bevy/pull/6427
[6425]: https://github.com/bevyengine/bevy/pull/6425
[6423]: https://github.com/bevyengine/bevy/pull/6423
[6422]: https://github.com/bevyengine/bevy/pull/6422
[6420]: https://github.com/bevyengine/bevy/pull/6420
[6416]: https://github.com/bevyengine/bevy/pull/6416
[6415]: https://github.com/bevyengine/bevy/pull/6415
[6414]: https://github.com/bevyengine/bevy/pull/6414
[6411]: https://github.com/bevyengine/bevy/pull/6411
[6410]: https://github.com/bevyengine/bevy/pull/6410
[6406]: https://github.com/bevyengine/bevy/pull/6406
[6401]: https://github.com/bevyengine/bevy/pull/6401
[6400]: https://github.com/bevyengine/bevy/pull/6400
[6398]: https://github.com/bevyengine/bevy/pull/6398
[6397]: https://github.com/bevyengine/bevy/pull/6397
[6395]: https://github.com/bevyengine/bevy/pull/6395
[6394]: https://github.com/bevyengine/bevy/pull/6394
[6393]: https://github.com/bevyengine/bevy/pull/6393
[6392]: https://github.com/bevyengine/bevy/pull/6392
[6385]: https://github.com/bevyengine/bevy/pull/6385
[6384]: https://github.com/bevyengine/bevy/pull/6384
[6382]: https://github.com/bevyengine/bevy/pull/6382
[6381]: https://github.com/bevyengine/bevy/pull/6381
[6380]: https://github.com/bevyengine/bevy/pull/6380
[6379]: https://github.com/bevyengine/bevy/pull/6379
[6377]: https://github.com/bevyengine/bevy/pull/6377
[6376]: https://github.com/bevyengine/bevy/pull/6376
[6374]: https://github.com/bevyengine/bevy/pull/6374
[6372]: https://github.com/bevyengine/bevy/pull/6372
[6365]: https://github.com/bevyengine/bevy/pull/6365
[6360]: https://github.com/bevyengine/bevy/pull/6360
[6359]: https://github.com/bevyengine/bevy/pull/6359
[6357]: https://github.com/bevyengine/bevy/pull/6357
[6354]: https://github.com/bevyengine/bevy/pull/6354
[6351]: https://github.com/bevyengine/bevy/pull/6351
[6350]: https://github.com/bevyengine/bevy/pull/6350
[6349]: https://github.com/bevyengine/bevy/pull/6349
[6345]: https://github.com/bevyengine/bevy/pull/6345
[6342]: https://github.com/bevyengine/bevy/pull/6342
[6341]: https://github.com/bevyengine/bevy/pull/6341
[6340]: https://github.com/bevyengine/bevy/pull/6340
[6337]: https://github.com/bevyengine/bevy/pull/6337
[6336]: https://github.com/bevyengine/bevy/pull/6336
[6333]: https://github.com/bevyengine/bevy/pull/6333
[6331]: https://github.com/bevyengine/bevy/pull/6331
[6329]: https://github.com/bevyengine/bevy/pull/6329
[6328]: https://github.com/bevyengine/bevy/pull/6328
[6321]: https://github.com/bevyengine/bevy/pull/6321
[6319]: https://github.com/bevyengine/bevy/pull/6319
[6318]: https://github.com/bevyengine/bevy/pull/6318
[6317]: https://github.com/bevyengine/bevy/pull/6317
[6314]: https://github.com/bevyengine/bevy/pull/6314
[6310]: https://github.com/bevyengine/bevy/pull/6310
[6309]: https://github.com/bevyengine/bevy/pull/6309
[6308]: https://github.com/bevyengine/bevy/pull/6308
[6303]: https://github.com/bevyengine/bevy/pull/6303
[6300]: https://github.com/bevyengine/bevy/pull/6300
[6296]: https://github.com/bevyengine/bevy/pull/6296
[6290]: https://github.com/bevyengine/bevy/pull/6290
[6288]: https://github.com/bevyengine/bevy/pull/6288
[6276]: https://github.com/bevyengine/bevy/pull/6276
[6274]: https://github.com/bevyengine/bevy/pull/6274
[6273]: https://github.com/bevyengine/bevy/pull/6273
[6270]: https://github.com/bevyengine/bevy/pull/6270
[6268]: https://github.com/bevyengine/bevy/pull/6268
[6261]: https://github.com/bevyengine/bevy/pull/6261
[6260]: https://github.com/bevyengine/bevy/pull/6260
[6257]: https://github.com/bevyengine/bevy/pull/6257
[6255]: https://github.com/bevyengine/bevy/pull/6255
[6249]: https://github.com/bevyengine/bevy/pull/6249
[6248]: https://github.com/bevyengine/bevy/pull/6248
[6247]: https://github.com/bevyengine/bevy/pull/6247
[6242]: https://github.com/bevyengine/bevy/pull/6242
[6237]: https://github.com/bevyengine/bevy/pull/6237
[6234]: https://github.com/bevyengine/bevy/pull/6234
[6233]: https://github.com/bevyengine/bevy/pull/6233
[6232]: https://github.com/bevyengine/bevy/pull/6232
[6230]: https://github.com/bevyengine/bevy/pull/6230
[6229]: https://github.com/bevyengine/bevy/pull/6229
[6227]: https://github.com/bevyengine/bevy/pull/6227
[6222]: https://github.com/bevyengine/bevy/pull/6222
[6218]: https://github.com/bevyengine/bevy/pull/6218
[6214]: https://github.com/bevyengine/bevy/pull/6214
[6211]: https://github.com/bevyengine/bevy/pull/6211
[6210]: https://github.com/bevyengine/bevy/pull/6210
[6209]: https://github.com/bevyengine/bevy/pull/6209
[6205]: https://github.com/bevyengine/bevy/pull/6205
[6200]: https://github.com/bevyengine/bevy/pull/6200
[6199]: https://github.com/bevyengine/bevy/pull/6199
[6198]: https://github.com/bevyengine/bevy/pull/6198
[6194]: https://github.com/bevyengine/bevy/pull/6194
[6193]: https://github.com/bevyengine/bevy/pull/6193
[6192]: https://github.com/bevyengine/bevy/pull/6192
[6191]: https://github.com/bevyengine/bevy/pull/6191
[6189]: https://github.com/bevyengine/bevy/pull/6189
[6187]: https://github.com/bevyengine/bevy/pull/6187
[6185]: https://github.com/bevyengine/bevy/pull/6185
[6180]: https://github.com/bevyengine/bevy/pull/6180
[6176]: https://github.com/bevyengine/bevy/pull/6176
[6175]: https://github.com/bevyengine/bevy/pull/6175
[6172]: https://github.com/bevyengine/bevy/pull/6172
[6170]: https://github.com/bevyengine/bevy/pull/6170
[6168]: https://github.com/bevyengine/bevy/pull/6168
[6167]: https://github.com/bevyengine/bevy/pull/6167
[6162]: https://github.com/bevyengine/bevy/pull/6162
[6160]: https://github.com/bevyengine/bevy/pull/6160
[6159]: https://github.com/bevyengine/bevy/pull/6159
[6158]: https://github.com/bevyengine/bevy/pull/6158
[6152]: https://github.com/bevyengine/bevy/pull/6152
[6149]: https://github.com/bevyengine/bevy/pull/6149
[6140]: https://github.com/bevyengine/bevy/pull/6140
[6134]: https://github.com/bevyengine/bevy/pull/6134
[6133]: https://github.com/bevyengine/bevy/pull/6133
[6132]: https://github.com/bevyengine/bevy/pull/6132
[6131]: https://github.com/bevyengine/bevy/pull/6131
[6127]: https://github.com/bevyengine/bevy/pull/6127
[6126]: https://github.com/bevyengine/bevy/pull/6126
[6123]: https://github.com/bevyengine/bevy/pull/6123
[6121]: https://github.com/bevyengine/bevy/pull/6121
[6119]: https://github.com/bevyengine/bevy/pull/6119
[6117]: https://github.com/bevyengine/bevy/pull/6117
[6115]: https://github.com/bevyengine/bevy/pull/6115
[6114]: https://github.com/bevyengine/bevy/pull/6114
[6113]: https://github.com/bevyengine/bevy/pull/6113
[6111]: https://github.com/bevyengine/bevy/pull/6111
[6109]: https://github.com/bevyengine/bevy/pull/6109
[6107]: https://github.com/bevyengine/bevy/pull/6107
[6103]: https://github.com/bevyengine/bevy/pull/6103
[6100]: https://github.com/bevyengine/bevy/pull/6100
[6099]: https://github.com/bevyengine/bevy/pull/6099
[6095]: https://github.com/bevyengine/bevy/pull/6095
[6088]: https://github.com/bevyengine/bevy/pull/6088
[6087]: https://github.com/bevyengine/bevy/pull/6087
[6084]: https://github.com/bevyengine/bevy/pull/6084
[6083]: https://github.com/bevyengine/bevy/pull/6083
[6082]: https://github.com/bevyengine/bevy/pull/6082
[6079]: https://github.com/bevyengine/bevy/pull/6079
[6074]: https://github.com/bevyengine/bevy/pull/6074
[6068]: https://github.com/bevyengine/bevy/pull/6068
[6067]: https://github.com/bevyengine/bevy/pull/6067
[6066]: https://github.com/bevyengine/bevy/pull/6066
[6058]: https://github.com/bevyengine/bevy/pull/6058
[6057]: https://github.com/bevyengine/bevy/pull/6057
[6054]: https://github.com/bevyengine/bevy/pull/6054
[6053]: https://github.com/bevyengine/bevy/pull/6053
[6049]: https://github.com/bevyengine/bevy/pull/6049
[6047]: https://github.com/bevyengine/bevy/pull/6047
[6039]: https://github.com/bevyengine/bevy/pull/6039
[6029]: https://github.com/bevyengine/bevy/pull/6029
[6025]: https://github.com/bevyengine/bevy/pull/6025
[6023]: https://github.com/bevyengine/bevy/pull/6023
[6020]: https://github.com/bevyengine/bevy/pull/6020
[6017]: https://github.com/bevyengine/bevy/pull/6017
[6016]: https://github.com/bevyengine/bevy/pull/6016
[6015]: https://github.com/bevyengine/bevy/pull/6015
[6014]: https://github.com/bevyengine/bevy/pull/6014
[6009]: https://github.com/bevyengine/bevy/pull/6009
[6008]: https://github.com/bevyengine/bevy/pull/6008
[6002]: https://github.com/bevyengine/bevy/pull/6002
[6000]: https://github.com/bevyengine/bevy/pull/6000
[5999]: https://github.com/bevyengine/bevy/pull/5999
[5997]: https://github.com/bevyengine/bevy/pull/5997
[5996]: https://github.com/bevyengine/bevy/pull/5996
[5995]: https://github.com/bevyengine/bevy/pull/5995
[5994]: https://github.com/bevyengine/bevy/pull/5994
[5993]: https://github.com/bevyengine/bevy/pull/5993
[5992]: https://github.com/bevyengine/bevy/pull/5992
[5991]: https://github.com/bevyengine/bevy/pull/5991
[5988]: https://github.com/bevyengine/bevy/pull/5988
[5987]: https://github.com/bevyengine/bevy/pull/5987
[5982]: https://github.com/bevyengine/bevy/pull/5982
[5978]: https://github.com/bevyengine/bevy/pull/5978
[5973]: https://github.com/bevyengine/bevy/pull/5973
[5971]: https://github.com/bevyengine/bevy/pull/5971
[5970]: https://github.com/bevyengine/bevy/pull/5970
[5969]: https://github.com/bevyengine/bevy/pull/5969
[5968]: https://github.com/bevyengine/bevy/pull/5968
[5955]: https://github.com/bevyengine/bevy/pull/5955
[5954]: https://github.com/bevyengine/bevy/pull/5954
[5952]: https://github.com/bevyengine/bevy/pull/5952
[5949]: https://github.com/bevyengine/bevy/pull/5949
[5948]: https://github.com/bevyengine/bevy/pull/5948
[5947]: https://github.com/bevyengine/bevy/pull/5947
[5945]: https://github.com/bevyengine/bevy/pull/5945
[5942]: https://github.com/bevyengine/bevy/pull/5942
[5938]: https://github.com/bevyengine/bevy/pull/5938
[5923]: https://github.com/bevyengine/bevy/pull/5923
[5921]: https://github.com/bevyengine/bevy/pull/5921
[5918]: https://github.com/bevyengine/bevy/pull/5918
[5917]: https://github.com/bevyengine/bevy/pull/5917
[5916]: https://github.com/bevyengine/bevy/pull/5916
[5910]: https://github.com/bevyengine/bevy/pull/5910
[5898]: https://github.com/bevyengine/bevy/pull/5898
[5895]: https://github.com/bevyengine/bevy/pull/5895
[5894]: https://github.com/bevyengine/bevy/pull/5894
[5886]: https://github.com/bevyengine/bevy/pull/5886
[5883]: https://github.com/bevyengine/bevy/pull/5883
[5878]: https://github.com/bevyengine/bevy/pull/5878
[5877]: https://github.com/bevyengine/bevy/pull/5877
[5871]: https://github.com/bevyengine/bevy/pull/5871
[5865]: https://github.com/bevyengine/bevy/pull/5865
[5864]: https://github.com/bevyengine/bevy/pull/5864
[5863]: https://github.com/bevyengine/bevy/pull/5863
[5860]: https://github.com/bevyengine/bevy/pull/5860
[5858]: https://github.com/bevyengine/bevy/pull/5858
[5857]: https://github.com/bevyengine/bevy/pull/5857
[5855]: https://github.com/bevyengine/bevy/pull/5855
[5854]: https://github.com/bevyengine/bevy/pull/5854
[5853]: https://github.com/bevyengine/bevy/pull/5853
[5847]: https://github.com/bevyengine/bevy/pull/5847
[5841]: https://github.com/bevyengine/bevy/pull/5841
[5839]: https://github.com/bevyengine/bevy/pull/5839
[5838]: https://github.com/bevyengine/bevy/pull/5838
[5835]: https://github.com/bevyengine/bevy/pull/5835
[5827]: https://github.com/bevyengine/bevy/pull/5827
[5826]: https://github.com/bevyengine/bevy/pull/5826
[5825]: https://github.com/bevyengine/bevy/pull/5825
[5821]: https://github.com/bevyengine/bevy/pull/5821
[5819]: https://github.com/bevyengine/bevy/pull/5819
[5816]: https://github.com/bevyengine/bevy/pull/5816
[5814]: https://github.com/bevyengine/bevy/pull/5814
[5813]: https://github.com/bevyengine/bevy/pull/5813
[5806]: https://github.com/bevyengine/bevy/pull/5806
[5803]: https://github.com/bevyengine/bevy/pull/5803
[5798]: https://github.com/bevyengine/bevy/pull/5798
[5797]: https://github.com/bevyengine/bevy/pull/5797
[5796]: https://github.com/bevyengine/bevy/pull/5796
[5783]: https://github.com/bevyengine/bevy/pull/5783
[5782]: https://github.com/bevyengine/bevy/pull/5782
[5780]: https://github.com/bevyengine/bevy/pull/5780
[5776]: https://github.com/bevyengine/bevy/pull/5776
[5766]: https://github.com/bevyengine/bevy/pull/5766
[5761]: https://github.com/bevyengine/bevy/pull/5761
[5760]: https://github.com/bevyengine/bevy/pull/5760
[5758]: https://github.com/bevyengine/bevy/pull/5758
[5757]: https://github.com/bevyengine/bevy/pull/5757
[5752]: https://github.com/bevyengine/bevy/pull/5752
[5751]: https://github.com/bevyengine/bevy/pull/5751
[5747]: https://github.com/bevyengine/bevy/pull/5747
[5742]: https://github.com/bevyengine/bevy/pull/5742
[5741]: https://github.com/bevyengine/bevy/pull/5741
[5740]: https://github.com/bevyengine/bevy/pull/5740
[5739]: https://github.com/bevyengine/bevy/pull/5739
[5731]: https://github.com/bevyengine/bevy/pull/5731
[5730]: https://github.com/bevyengine/bevy/pull/5730
[5729]: https://github.com/bevyengine/bevy/pull/5729
[5728]: https://github.com/bevyengine/bevy/pull/5728
[5727]: https://github.com/bevyengine/bevy/pull/5727
[5723]: https://github.com/bevyengine/bevy/pull/5723
[5720]: https://github.com/bevyengine/bevy/pull/5720
[5711]: https://github.com/bevyengine/bevy/pull/5711
[5710]: https://github.com/bevyengine/bevy/pull/5710
[5708]: https://github.com/bevyengine/bevy/pull/5708
[5706]: https://github.com/bevyengine/bevy/pull/5706
[5688]: https://github.com/bevyengine/bevy/pull/5688
[5686]: https://github.com/bevyengine/bevy/pull/5686
[5685]: https://github.com/bevyengine/bevy/pull/5685
[5684]: https://github.com/bevyengine/bevy/pull/5684
[5678]: https://github.com/bevyengine/bevy/pull/5678
[5676]: https://github.com/bevyengine/bevy/pull/5676
[5672]: https://github.com/bevyengine/bevy/pull/5672
[5671]: https://github.com/bevyengine/bevy/pull/5671
[5666]: https://github.com/bevyengine/bevy/pull/5666
[5665]: https://github.com/bevyengine/bevy/pull/5665
[5664]: https://github.com/bevyengine/bevy/pull/5664
[5659]: https://github.com/bevyengine/bevy/pull/5659
[5658]: https://github.com/bevyengine/bevy/pull/5658
[5657]: https://github.com/bevyengine/bevy/pull/5657
[5651]: https://github.com/bevyengine/bevy/pull/5651
[5648]: https://github.com/bevyengine/bevy/pull/5648
[5635]: https://github.com/bevyengine/bevy/pull/5635
[5630]: https://github.com/bevyengine/bevy/pull/5630
[5626]: https://github.com/bevyengine/bevy/pull/5626
[5624]: https://github.com/bevyengine/bevy/pull/5624
[5614]: https://github.com/bevyengine/bevy/pull/5614
[5613]: https://github.com/bevyengine/bevy/pull/5613
[5611]: https://github.com/bevyengine/bevy/pull/5611
[5609]: https://github.com/bevyengine/bevy/pull/5609
[5608]: https://github.com/bevyengine/bevy/pull/5608
[5601]: https://github.com/bevyengine/bevy/pull/5601
[5600]: https://github.com/bevyengine/bevy/pull/5600
[5593]: https://github.com/bevyengine/bevy/pull/5593
[5591]: https://github.com/bevyengine/bevy/pull/5591
[5590]: https://github.com/bevyengine/bevy/pull/5590
[5587]: https://github.com/bevyengine/bevy/pull/5587
[5586]: https://github.com/bevyengine/bevy/pull/5586
[5583]: https://github.com/bevyengine/bevy/pull/5583
[5582]: https://github.com/bevyengine/bevy/pull/5582
[5577]: https://github.com/bevyengine/bevy/pull/5577
[5566]: https://github.com/bevyengine/bevy/pull/5566
[5560]: https://github.com/bevyengine/bevy/pull/5560
[5558]: https://github.com/bevyengine/bevy/pull/5558
[5557]: https://github.com/bevyengine/bevy/pull/5557
[5556]: https://github.com/bevyengine/bevy/pull/5556
[5554]: https://github.com/bevyengine/bevy/pull/5554
[5551]: https://github.com/bevyengine/bevy/pull/5551
[5548]: https://github.com/bevyengine/bevy/pull/5548
[5546]: https://github.com/bevyengine/bevy/pull/5546
[5537]: https://github.com/bevyengine/bevy/pull/5537
[5533]: https://github.com/bevyengine/bevy/pull/5533
[5532]: https://github.com/bevyengine/bevy/pull/5532
[5531]: https://github.com/bevyengine/bevy/pull/5531
[5527]: https://github.com/bevyengine/bevy/pull/5527
[5512]: https://github.com/bevyengine/bevy/pull/5512
[5509]: https://github.com/bevyengine/bevy/pull/5509
[5495]: https://github.com/bevyengine/bevy/pull/5495
[5483]: https://github.com/bevyengine/bevy/pull/5483
[5481]: https://github.com/bevyengine/bevy/pull/5481
[5473]: https://github.com/bevyengine/bevy/pull/5473
[5461]: https://github.com/bevyengine/bevy/pull/5461
[5420]: https://github.com/bevyengine/bevy/pull/5420
[5413]: https://github.com/bevyengine/bevy/pull/5413
[5409]: https://github.com/bevyengine/bevy/pull/5409
[5404]: https://github.com/bevyengine/bevy/pull/5404
[5402]: https://github.com/bevyengine/bevy/pull/5402
[5340]: https://github.com/bevyengine/bevy/pull/5340
[5325]: https://github.com/bevyengine/bevy/pull/5325
[5321]: https://github.com/bevyengine/bevy/pull/5321
[5296]: https://github.com/bevyengine/bevy/pull/5296
[5283]: https://github.com/bevyengine/bevy/pull/5283
[5250]: https://github.com/bevyengine/bevy/pull/5250
[5205]: https://github.com/bevyengine/bevy/pull/5205
[5188]: https://github.com/bevyengine/bevy/pull/5188
[5096]: https://github.com/bevyengine/bevy/pull/5096
[4992]: https://github.com/bevyengine/bevy/pull/4992
[4972]: https://github.com/bevyengine/bevy/pull/4972
[4970]: https://github.com/bevyengine/bevy/pull/4970
[4919]: https://github.com/bevyengine/bevy/pull/4919
[4809]: https://github.com/bevyengine/bevy/pull/4809
[4800]: https://github.com/bevyengine/bevy/pull/4800
[4761]: https://github.com/bevyengine/bevy/pull/4761
[4724]: https://github.com/bevyengine/bevy/pull/4724
[4696]: https://github.com/bevyengine/bevy/pull/4696
[4466]: https://github.com/bevyengine/bevy/pull/4466
[4460]: https://github.com/bevyengine/bevy/pull/4460
[4452]: https://github.com/bevyengine/bevy/pull/4452
[4352]: https://github.com/bevyengine/bevy/pull/4352
[4109]: https://github.com/bevyengine/bevy/pull/4109
[3528]: https://github.com/bevyengine/bevy/pull/3528
[3425]: https://github.com/bevyengine/bevy/pull/3425
[3009]: https://github.com/bevyengine/bevy/pull/3009
[2975]: https://github.com/bevyengine/bevy/pull/2975
