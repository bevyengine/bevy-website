+++
title = "Roadmap"
weight = 3
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++


Here is the current list of planned features. All items are sorted in approximate priority order, but actual implementation order will vary based on the individual interests of Bevy contributors and/or funding.

## UI Framework Additions

* Text
* Styling

## Rendering

* Physically based rendering
* Skeletal animation
* Add additional runtime type safety to uniform bindings (and maybe compile time where possible)
* Inject layout set/bindings into shader source so they don't need to be defined in-shader. Specify set / binding indices in render resource providers?
* Pull as much logic as possible from renderer into a "render orchestrator" struct/trait. This should hopefully make it easier to implement new render backends

## Docs

* Fully document public apis
    * Add ```#![deny(warnings, missing_docs)]``` to ensure future contributions meet style/doc standards
* Finish "getting started" tutorial
* Add docs for specific features
* "Template" projects for common game types

## ECS

* Remove as many references to Resources as possible in favor of resolved resource types and/or systems
* Consider adding Renderer and World to Resources

## Error Handling

* Formalize error handling: should we use a custom error type?
* Remove as many panics / unwraps as possible

## Input

* Mouse events
* Gamepad events

## Assets

* Load GLTF files

## Scene

* Define scene format
* Load scenes from files (likely RON)

## Plugins

* Live/hot plugin reloading (this should build on top of the existing dynamic plugin loading system)

## Editor

* Editor <-> game communication protocol
* Build UI using bevy UI framework
* As much as possible support embedding parts of the editor directly into games

## Physics

* High level physics data types
* nphysics 2D physics backend
* nphysics 3D physics backend

## Platform Support

* Android
* iOS
* Web