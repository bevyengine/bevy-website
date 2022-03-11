+++
title = "Physically Based Rendering (PBR)"
weight = 5
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

PBR is a rendering technique that tries to imitate physical properties of a surface and how it influences the flow of light on these surfaces.

<!-- TODO actually explain some basic properties of PBR -->

## Surface properties

When using the `StandarMaterial` you can define multiple properties that affects the final equation used in the PBR shader when rendering a surface.

<!-- TODO this is pretty much just a copy paste of the filament doc -->
<!-- <https://google.github.io/filament/Material%20Properties.pdf>. -->
<!-- TODO add images similar to filament but using bevy to render them -->

### Base Color

Defines the perceived color of an object (sometimes calledalbedo)

* The **diffuse color** of a **non-metallic** object
* The **specular color** of a **metallic** object

### Metalic

Defines whether a surface is **dielectric** (0.0, non-metal) or **conductor** (1.0, metal). Pure, unweathered surfaces are rare and will be either 0.0 or 1.0. Rust (oxidized iron not the language 😉)  is not a conductor.

### Roughness

Defines the perceived **smoothness** (0.0) or **roughness** (1.0). It is sometimes called **glossiness**.

### Reflectance

Specular intensity for **non-metals**. The default is 0.5, or 4% reflectance.
