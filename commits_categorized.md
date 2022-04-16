## Rendering

### Animation
    5e70ad96c animations: don't ignore curves with one keyframe (#4406)
    449a1d223 animation player (#4375)

### Skinning

    9d54f3397 Skinned extraction speedup (#4428)
    3537c6ae2 Fix animation: shadow and wireframe support (#4367)
    31bd4ecbb Mesh Skinning. Attempt #3 (#4238)

### More lights / Storage Buffers

    c5963b4fd Use storage buffers for clustered forward point lights (#3989)
    4feb0d520 increase the maximum number of point lights with shadows to the max supported by the device (#4435)
    0a4136d26 Add a helper for storage buffers similar to `UniformVec` (#4079)
    3f6068da3 fix issues with too many point lights (#3916)
    575ea81d7 add Visibility for lights (#3958)
    a9f2817c4 bevy_pbr: Do not panic when more than 256 point lights are added the scene (#3697)

### Clustered Improvements
    207ebde02 (james7132/main) Always update clusters and remove per-frame allocations (#4169)
    5af746457 fix cluster tiling calculations (#4148)
    244687a0b Dynamic light clusters (#3968)
    b4483dbfc perf: only recalculate frusta of changed lights (#4086)
    e3a3b5b9c Fixed the frustum-sphere collision and added tests (#4035)
    786654307 bevy_pbr: Optimize assign_lights_to_clusters (#3984)

### Faster Culling
ac8bbafc5 Faster view frustum culling (#4181)

### Sprites
    8e864fdd1 can specify an anchor for a sprite (#3463)
f6bc9a022 Sprites - keep color as 4 f32 (#4361)

### Gltf

d478723e1 insert the gltf mesh name on the entity if there is one (#4119)
8268e7fa9 expose extras from gltf nodes (#2154)
703ae5df5 gltf: add a name to nodes without names (#4396)
    30878e3a7 (mockersf/main) add AnimationPlayer component only on scene roots that are also animation roots (#4417)
48ac955af Fix loading non-TriangleList meshes without normals in gltf loader (#4376)
    fbe7a49d5 Gltf animations (#3751)
3e631e623 gltf-loader: disable backface culling if material is double-sided (#4270)
b4bf5b5d8 Fix glTF perspective camera projection (#4006)

### Shaders
551d9f6cd use error scope to handle errors on shader module creation (#3675)
e4203c392 shader preprocessor - do not import if scope is not valid (#4012)
e9f52b9dd Move import_path definitions into shader source (#3976)
56b0e88b5 Add view transform to view uniform (#3885)
b7dfe1677 include sources in shader validation error (#3724)
600ee7eee support all line endings in shader preprocessor (#3603)

### Compressed Textures
    0529f633f KTX2/DDS/.basis compressed texture support (#3884)

### Compute Pipeline Specialization
    9e450f282 Compute Pipeline Specialization (#3979)

### Render To Texture
    81d57e129 Add capability to render to a texture (#3412)

### Custom Vertex Buffer Layouts
    e369a8ad5 Mesh vertex buffer layouts (#3959)

### Wgpu Feature / Limit Overrides
803e8cdf8 bevy_render: Support overriding wgpu features and limits (#3912)
936468aa1 bevy_render: Use RenderDevice to get limits/features and expose AdapterInfo (#3931)

### Other

579928e8e bevy_pbr: Support flipping tangent space normal map y for DirectX normal maps (#4433)
6c085cba4 bevy_render: Support removal of nodes, edges, subgraphs (#3048)
1d5145fd6 StandardMaterial: expose a cull_mode option (#3982)
40b36927f Expose draw indirect (#4056)
0ccb9dd07 bevy_render: Fix Quad flip (#3741)
55da31543 bevy_render: Provide a way to opt-out of the built-in frustum culling (#3711)

### Camera Marker Components

    bf6de8962 use marker components for cameras instead of name strings (#3635)

### Tracing
fee7a2613 Tracy spans around main 3D passes (#4182)

### Tweaks

ef823d369 bevy_render: Do not automatically enable MAPPABLE_PRIMARY_BUFFERS (#3698)
865698548 Fix HDR asset support (#3795)
aa7b15889 Add a size method on Image. (#3696)
33ef5b503 bevy_render: Only auto-disable mappable primary buffers for discrete GPUs (#3803)
01bdf67c3 Improve the `set_active_camera` system (#4251)
ea6e6f7db Do not crash if RenderDevice doesn't exist (#4427)
dbd5e7ab6 Fixed case of R == G, following original conversion formula (#4383)
7ff3d876f Clean up duplicated color conversion code (#4360)
6844a6f4f Introduce `SystemLabel`'s for `RenderAssetPlugin`, and change `Image` preparation system to run before others (#3917)
0e821da70 bevy_render: Batch insertion for prepare_uniform_components (#4179)
e36c9b6cf Add conversions from Color to u32 (#4088)
cba9bcc7b improve error messages for render graph runner (#3930)
544b6dfb8 Change default `ColorMaterial` color to white (#3981)

## ECS
    63fee2572 `ParamSet` for conflicting `SystemParam`:s (#2765)

    509548190 Add get_multiple and get_multiple_mut APIs for Query and QueryState (#4298)
    b33dae31e Rename get_multiple APIs to get_many (#4384)

    c26be3971 Remove unnecessary system labels (#4340)
    b1c3e9862 Auto-label function systems with SystemTypeIdLabel (#4224)

73edb11db Add more FromWorld implementations (#3945)
    f16768d86 bevy_derive: Add derives for `Deref` and `DerefMut` (#4328)

    b6a647cc0 default() shorthand (#4071)

    557ab9897 Make get_resource (and friends) infallible (#4047)

2f11c9dca Add Query::contains (#3090)
64d217823 Allow iter combinations on queries with filters (#3656)

b1afe2dcc Make `System` responsible for updating its own archetypes (#4115)
21a875d67 Some small changes related to run criteria piping (#3923)
032b0f4ba Fix derive(SystemParam) macro (#4400)
1b63d949e Re-add ECS benchmark suite (#4332)
4c1678c78 Hide docs for concrete impls of Fetch, FetchState, and SystemParamState (#4250)
6e61fef67 Obviate the need for `RunSystem`, and remove it (#3817)
cf46baa17 Add clear_schedule (#3941)
c1a4a2f6c Remove the config api (#3633)
    ba6b74ba2 Implement `WorldQuery` derive macro (#2713)
bdbf62634 Implement init_resource for `Commands` and `World` (#3079)
6615b7bf6 Deprecate `.system` (#3302)
b3462428c Move the CoreStage::Startup to a seperate StartupSchedule label (#2434)
    760466588 Implement AnyOf queries (#2889)
fe4a42a64 Mut to immut impls (#3621)
f00aec245 Added method to restart the current state (#3328)
7d712406f Simplify sending empty events (#2935)
6f111136b Cleanup some things which shouldn't be components (#2982)
    1f99363de Add &World as SystemParam (#2923)
c216738b3 Implement len and is_empty for EventReaders (#2969)
21ac4bc0a impl Command for <impl FnOnce(&mut World)> (#2996)
69e9a47d9 SystemParam Derive fixes (#2838)
ce752d252 Increment last event count on next instead of iter (#2382)

## UI
6f16580b8 Fix clicked UI nodes getting reset when hovering child nodes (#4194)
954022c79 Add text wrapping support to Text2d (#4347)
d3e526bfc Add FocusPolicy to NodeBundle and ImageBundle (#3952)
e749ee786 Fix ui interactions when cursor disappears suddenly (#3926)
fe0e5580d Fix node update (#3785)

## Utils
b3a1db60f Proper prehashing (#3963)

## Audio

    b21c69c60 Audio control - play, pause, volume, speed, loop (#3948)

## Assets
baae97d00 `iter_mut` on Assets: send modified event only when asset is iterated over (#3565)
98938a855 Internal Asset Hot Reloading (#3966)
62329f7fd Useful error message when two assets have the save UUID (#3739)
75286b854 check if resource for asset already exists before adding it (#3560)
e928acb9f bevy_asset: Add AssetServerSettings watch_for_changes member (#3643)

## Windowing

39d89fe0d Enable drag-and-drop events on windows (#3772)
3756181e2 (HEAD, origin/staging, origin/main) Change scaling mode to FixedHorizontal (#4055)
b3aff9a7b Add docs and common helper functions to `Windows` (#4107)
    2d674e7c3 Reduce power usage with configurable event loop (#3974)
1477765f6 Replace VSync with PresentMode (#3812)
c16d0c5a3 do not set cursor grab on window creation if not asked for (#3617)

## Transforms
54d2e86af bevy_transform: Use Changed in the query for much faster transform_propagate_system (#4180)
54fbaf4b4 Add transform hierarchy stress test (#4170)
a304fd9a9 Split bevy_hierarchy out from bevy_transform (#4168)
2b1120261 (robtfm/main) fix mul_vec3 tranformation order: should be scale -> rotate -> translate (#3811)
59ee51229 Add TransformBundle (#3054)
f584e7295 Add Transform::rotate_around method (#3107)
37a7be56d Make transform builder methods const (#3045)

## Safety
    dba779001 REMOVE unsound lifetime annotations on `EntityMut` (#4096)
    024d98457 yeet unsound lifetime annotations on `Query` methods (#4243)
    637a14991 unsafeify `World::entities_mut` (#4093)
    050d2b7f0 yeet `World::components_mut` >:( (#4092)
    142e7f3c5 Backport soundness fix (#3685)
    af22cc1dc Use ManuallyDrop instead of forget in insert_resource_with_id (#2947)

## Examples

3555603df example on how to create an animation in code (#4399)
e7e74457c scene viewer improvements: animation reset (#4420)
7e3637c36 fix scene_viewer example on wasm (#4426)
28d0a4002 Only enable mouse movement when pressed in scene_viewer (#4405)
ac29cbecf add support for loading animations in `scene_viewer` example (#4403)
738cd5e74 Add a scene viewer tool (#4183)
21f6760b2 Render to texture example: No need to create an image handle manually. (#4223)
9dfd4e4b0 (ManevilleF/staging) Add examples for Transforms (#2441)
6ce8e5006 Add mouse grab example (#4114)
a188babce many_cubes: Add a cube pattern suitable for benchmarking culling changes (#4126)
159fe527a (Ku95/main) Slow down the many_cubes example (#4117)
6c95b582a (tornewuff/staging, DJMcNab/main) Make many_cubes example more interesting (#4015)
3ffa655cd examples: add screenspace texture shader example (#4063)
38f6da5a8 Add generic systems example (#2636)
385a2b189 add examples on how to have a data source running in another thread / in a task pool thread (#2915)
f991c73bd Add move sprite example.  (#2414)
ac63c491f Simple 2d rotation example (#3065)
3e8e6c567 add an example using UI & states to create a game menu (#2960)

## CI

    28ba87e6c (alice-i-cecile/main) CI runs `cargo miri test -p bevy_ecs` (#4310)
c44f8b2b6 Run tests (including doc tests) in `cargo run -p ci` command (#3849)

## Profiling

de677dbfc Use more ergonomic span syntax (#4246)

## Dev Docs

e020c5721 Add automatic docs deployment to GitHub Pages (#3535)
026563cb1 Deploy dev docs - install dependencies (#4222)
6ff17eaab Deploy dev docs - fix sed command (#4221)

## Tweaks / Docs
f23ae104b Slight perf improvements and tidy for contributors example (#3764)
f907d67d7 Fix formatting, spelling, phrasing and consistency (#4275)
dbb2fcb67 Re-enable test_property_type_registration() (#4419)
cf831d518 input clear should not clear pressed (#4418)
99a2dc50a Cleanup some outdated parts of ecs_guide (#4342)
d9d2fb640 Added pressed state to input clear so it clears everything (#4410)
f90da74e3 Remove `face_toward.rs` (#4277)
0ed08d6a1 fix multiple_windows example (#4389)
dd2001f2f bevy_render: add a small color box to each color (#4387)
648544faf Bump actions/cache from 2 to 3 (#4348)
aca7fc185 Remove outdated perf comments (#4374)
3af90b67a Update RemovedComponents doc comment (#4373)
4480b36bf (BoxyUwU/main) Replace confusing links from CONTRIBUTING.md with short instruction (#4369)
2b35dbabf impl Reflect and Debug for Mesh2dHandle (#4368)
c7c08f95c Fix gamepad connection system ordering (#4313)
f3a61327a Remove `margins.rs` (#4284)
f5e53ba6f Only insert or remove input if needed (#4273)
8570b651f Clean up Breakout logic (#4311)
a190cd59d Revert changes to colors in Breakout game (#4325)
ab83336f5 Replace `_system` system names in Breakout examples (#4312)
d51b54a65 Break out Breakout components into a more sensible organization (#4261)
fa791d6bb (bilsen/main) Make a note about the MSRV (#4274)
31636a350 Update linux_dependencies for Gentoo. (#4195)
685b6e59b (tornewuff/main) register `Camera{2,3}d` components for reflection (#4269)
cd694c0d0 Prevent event from getting registered twice (#4258)
b0ddce36b Check an `Input`'s `pressed` set before adding to `just_released`. (#4209)
af24576b9 Move magic numbers into constants in Breakout example (#4255)
95d3f32b9 Revert "Add cart's fork of ecs_bench_suite (#4225)" (#4252)
08ef2f0a2 Add cart's fork of ecs_bench_suite (#4225)
e7a942044 Change `Cow<[ComponentId]>` to `Box<[ComponentId]>` (#4185)
7ce3ae43e Bump Bevy to 0.7.0-dev (#4230)
c1a237879 Add "Changelog" and "Migration Guide" to PR template (#4143)
a291b5aae Ignore duplicate wasi crate in dependency tree (#4190)
b493165bf Use reactive rendering for ui examples. (#4164)
1a85fb5ea Fix mesh2d_manual example (#4037)
4add96b1b Cleanup doc / comments about changed defaults (#4144)
de2a47c2b export TaskPoolThreadAssignmentPolicy (#4145)
e41c5c212 Fix UI node Transform change detection (#4138)
fb02b8422 Bump actions/checkout from 2 to 3 (#4136)
a88a59c9e Bump actions/upload-artifact from 1 to 3 (#4135)
ce871d16f Bump actions/labeler from 3 to 4 (#4134)
c05ba2370 Add Reflect support for DMat3, DMat4, DQuat (#4128)
0eec2ea0d Slight changes from the book (#4077)
72bb38cad Example of module-level log usage and RUST_LOG usage in main doc (#3919)
caf6611c6 remove Events from bevy_app, they now live in bevy_ecs (#4066)
1ba9818a7 Significantly reduce the amount of building required for benchmarks (#4067)
258f49535 log spans on panic when trace is enabled (#3848)
461cf536b Slight perf improvements and tidy for bevymark (#3765)
44bf66e43 Minor Dark/Light Logo Tweak (#4051)
371c90f6f Minor Bevy Logo Tweaks (#4050)
b697e73c3 Enhanced par_for_each and par_for_each_mut docs (#4039)
c4e88fe4b Rename "2d rotation" example name to "rotation" (#3965)
1fa54c200 Updated visibility of reflected trait (#4034)
519148275 Rename rg3d to Fyrox in README (#4032)
95bc99fd3 Implement Reflect for missing Vec* types (#4028)
a2d49f4a6 Make `WinitWindows` non send (#4027)
b3a2cbbc9 remove external_type_uuid macro (#4018)
5afda8df6 Fix all_tuples macro for non-0/1 starts (#4002)
fb8af3aec Update Hexasphere to 7.0.0 (#4001)
c4f132afb Fix call with features in docs/profiling.md (#3967)
b0768a583 Fix custom material glsl example using incorrect CameraViewProj (#3962)
510439768 Alien cake addict: Allow holding movement keys (#2072)
330160cf1 SystemState usage docs (#3783)
d8974e7c3 small and mostly pointless refactoring (#2934)
5bb4201f2 add informative panic message when adding render commands to a DrawFunctions that does not exist (#3924)
d305e4f02 only use unique type UUIDs (#3579)
647526835 Fix hardcoded texture bind group index in bevy_ui (#3905)
9a7852db0 (robtfm/staging) Fix SetSpriteTextureBindGroup to use index (#3896)
1e049a651 Fix type mismatch error with latest winit (#3883)
1468211e2 fix unreachable macro calls for rust 2021 (#3889)
f7478f448 doc: remove mention of void return type in entry_point docs (#3881)
3d6e4893f reverse how diagnostic values are saved (#3056)
b13f238fc allow Entity to be deserialized with serde_json (#3873)
a0af066af fix typo in bevy_ecs/README.md (#3869)
3431335ee add missing `into_inner` to `ReflectMut` (#3841)
b11ee3ffb Remove duplicate call to set_vertex_buffer(0, ...) in shader_instancing example (#3738)
ca83e8a6d fix: remove unneeded filter in check_light_mesh_visibility (#3861)
6b8d64cd0 impl more traits for bevy_core::Name (#3611)
e2cce092d derive clone for Input (#3569)
ef65548fb Change default window title to "app" (#3417)
bb1538a13 improve error message for attempting to add systems using add_system_to_stage (#3287)
6ac9d6876 Make ECS benchmark more representative (#2941)
19bd6b9c3 Update rodio 0.15 (#3846)
b506c30cd fix: only init_resource() once for AmbientLight (#3853)
506642744 docs: Fix private doc links and enable CI test (#3743)
6cab36165 Update ClearColor Resource docs (#3842)
6a499b125 Update gltf requirement from 0.16.0 to 1.0.0 (#3826)
c285a69f7 Add the Inside version to the Collision enum (#2489)
4134577e6 Fix Code of Conduct bolding in readme (#3822)
ca029ef0e Naga export (#3714)
44d09dc46 fix timer test to be less reliant on float precision (#3789)
435fb7af4 Improve shader_material example documentation (#3601)
6d76229c3 Fix a typo in shader_defs example (#3762)
514754d65 Add crate level docs to bevy_log and enable #![warn(missing_docs)] (#3520)
f5039a476 Mark .id() methods which return an `Entity` as must_use (#3750)
f3de12bc5 Add a warning when `watch_for_changes` has no effect (#3684)
f1f6fd349 Remove `ComponentsError` (#3716)
e30d600db Update docstrings for `text_system` and `text2d_system` (#3732)
cb2ba19d9 rename Texture to Image in doc of `from_buffer` function (#3713)
e88e394fe Remove wasm specific examples (#3705)
836ffeda3 Add missing dependencies for Fedora with Wayland (#3708)
a3e43b6ab Remove cargo-lipo from iOS ci job (#3671)
8139022ec Change bevy_core::Name to implement Deref<Target = str> (#3681)
e16ba80bd Add bevy_bird SVG for simpleicons.org (#3672)
d11cd6345 Fixed doc comment with render Node input/output methods (#3642)
3fcdc5a49 Expanded Msaa documentation. (#3693)
758fc5af6 Remove a couple of unnecessary full paths (#3699)
e30199f7a Document bevy_tasks and enable #![warn(missing_docs)] (#3509)
71814ca91 Added API guidelines to CONTRIBUTING.md (#3646)
8e1f660e1 Don't panic in macro shape validation (#3647)
2186eae89 bevy_crevice: Fix incorrect iterator usage in WriteStd430 impl for [T] (#3591)
7c22f92ce Document sub apps (#3403)
f073b2d7f document more of bevy_reflect (#3655)
39db8ecd0 Added docs for bevy_transform (#3516)
17bb812d5 Ignore clippy 1.58 (#3667)
7fd781e67 Fix documentation for QueryState::iter_manual (#3644)
84144c942 Remove documentation warning on EntityCommands::insert that is no longer necessary (#3653)
bc499591c Use `use` instead of lots of full paths (#3564)
fc0f15f11 Documentation: simplify NixOS dependencies (#3527)
6bc5c6098 Remove tests from example style guide (#3582)
130953c71 Enable the `doc_markdown` clippy lint (#3457)
e56685370 Fix `doc_markdown` lints in `bevy_render` (#3479)
6f167aa3d Documented `Events` (#3306)
8c81e8167 Thread local example cleanup (#3586)
4b4dbb021 document `Struct`, `TupleStruct` and `Tuple` (#3081)
