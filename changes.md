## Release TODO
* bevy_ecs relicense (not required ... tbd)
* re-count prs + contributors
* match typed contributor names to auto-gen contributor list (sanity check)
* update docs (especially tutorial) 

## Changes


### WASM

* WASM / WebGL2
    * internal renderer changes mrk-its
        * delegate buffer aligning to render_resource_context
        * reflection to renderresourcecontext
        * auto-reflect dynamic bindings
        * non-spirv shader specializatoin
        * instant
    * plugin
    
### Write once run anywhere

* Cross platform main

### Live Shader Reloading
* Live shader reloading @yrns

### ECS

* Flexible ECS Params
* Query Filters
* System Inputs, Outputs, Chaining, Registration Ergo @cart
* bevy_ecs + bevy_hecs merger
* upsrteam hecs improvements (@Veykril, upstream contribs)
    * better Bundle derive macro (generics, tuple, unit structs)
* Check for conflicting system resource parameters  @memoryruins
* ECS Change Bitflags @cart
* Schedule V2
    * improve usablityl of systemsage 

### GLTF
* gltf improvements
    * Camera and Hierarchy @FuriouZz
    * Pixel format conversion @iwikal
    * Default material loading @rod-salazar
### Scene Usability 
* Scene usability
    * Spawn scenes as children @mockersf

### Dynamic Linking

* Dynamic Linking @bjorn3 @cart

### Text Improvements

* Text Improvements @AlisCode
    * use glyph_brush_layout for text layout
    * text alignment
    * fixes a number of text releated bugs (such as the infamous bouncing text)

### Performance Improvements

* Renderer Optimization @cart
    * w/ help from profiling changes
    * round 1
    * Text Rendering / Shared Buffers
    * Asset GPU data transfer
* Dont draw text when it isn't visible @marius851000
* mailbox when possible @cart

### Reflection
* Reflection @cart

### 3D Texture Assets

* 3D texture asset support @bonsairobo

### Logging and Profiling

* Tracing @superdump @cart
    * logging
    * profiling
    * enabled by default
    * tracing-chrome @superdump
    * Log Plugin @cart
        * cross platform logging
        * custom Android tracing logger
    * Helpful span names tracing-chrome @superdump

### Task Scheduler Improvements
* Task scheduler deadlock fix (and perf improvements) @aclysma

### HIDPI
* HiDPI
    * use logical units by default @mockersf
    * swap chain scaling @cart
    * float width/height @blunted2night
    * handle rounding issues @blunted2night
    * stretch hidpi @blunted2night

### Timer Improvements
* Timer improvements
    * pausing and encapsulation @amberkowalski
    * Timer ergo @marcusbuffett
    *  Timer ergo and tests @CleanCut
    * Timer polishing @CleanCut
    * Refactor @amberkowalski

### Apple Silicon Support
* Apple Silicon Support
    * use shaderc + winit + testing @frewsxcv
    * upstream
        * coreaudo-sys @wyhaya 
        * winit @scoopr

### Examples
* contributors example @karoffel
* bevymark example @robdavenport


### MISC
* receivedchar event
* Controllable ambient light color no1hitjam
* Color constants @milkybit
* EventId for events @bjorn3
* set cursor position @smokku
