+++
title = "Features"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

Before we start learning how to build Bevy apps, lets take a quick look at the features Bevy currently offers:

## Cross Platform

* Windows (Vulkan, DirectX 12), MacOS (Metal), Linux (Vulkan)
* More platforms (like the web) coming soon

## Data Driven and Massively Parallel

* All engine and game logic uses Bevy ECS, a custom Entity Component System built on top of [hecs](https://github.com/Ralith/hecs)  
* Simple data processing model:
    * Components: just normal structs
    * Entities: sets of components
    * Resources: global data
    * Systems: functions that iterate over entities/components and perform logic
* Cache friendly: Bevy ECS is an archetypal ECS. It lays out components in memory to maximize query performance
* Parallel by default: Bevy ECS analyzes system dependencies and automatically executes them in parallel

## Modular Plugin-Based Design

* All Bevy logic is implemented as [Plugins](/learn/book/getting-started/plugins). This includes all engine logic _and_ game logic.
* Dynamic plugin loading (optional)

## Modern Renderer

* Multiple Backends: Vulkan, DirectX 12, Metal
* Modern and flexible low level "Render Graph" api
    * Easy to use high level defaults for beginners 
    * Experts can create their own Render Graphs or modify the defaults
* Powerful data-driven shader system
    * Define your shaders in either GLSL or SPIR-V
    * Automatically generates pipelines for shaders using SPIR-V reflection
    * Easily and efficiently bind ECS component data to shader uniforms
    * Use component data to define macros in shaders and automatically recompile them if the shader has changed

## Expressive UI System

* Compose complex UIs using a familiar "box model" 

## Events

* Cleanly integrates with ECS systems
* Efficient: rarely (if ever) allocates new memory. Event readers are very cheap and never allocate on the heap. 
* No distinction between custom events and engine events. Everything uses the same system!

## Fast Iterative Compile Times

* The examples can generally be iteratively compiled in 1-3 seconds (when using the LLD linker on nightly rust)