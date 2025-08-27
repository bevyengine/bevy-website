+++
title = "Standard Material"
insert_anchor_links = "right"
[extra]
weight = 1
status = 'hidden'
+++

In Bevy, the main `Material` the engine provides is the `StandardMaterial`.

StandardMaterial, like most of the functionality in `bevy_pbr`, implements an idea called "physically based rendering", or PBR.

PBR is when you use formulas and units derived from real-world physics and math. PBR is oftentimes a concept more than a strict set of rules, and approximations are used in the name of performance, but where possible you try to stick to real-world physics.

Before PBR, artists designed lighting, material, and camera properties more ad-hoc. When answering the question "what color should this object be?", artists would just choose a value that they thought looked good. Properties like how shiny or smooth an object are were similiary made up by the artist.

While this process worked fine for smaller scenes, as larger movies and games got created, assets became harder to reuse. A coin that looked the correct shade of yellow, with a certain shininess in one scene, might look completely wrong when reused in another scene under different lighting conditions.

With PBR, to answer "what color should this object be?", you instead reference values from a database of real-world measurements like [Physically Based](https://physicallybased.info). Assets are more scene-independent, and behave consistently in a variety of different lighting conditions. Even if you're making a stylized game, PBR is still important.

## Theory

While `Material`s in Bevy implement fragment shaders (TODO: reference rendering stuff and the custom material chapter) that run arbitrary functions for computing the color of each pixel, `StandardMaterial` and PBR materials in general are based on the concept of a bidirectional reflectance distribution function (BRDF).

In the real world, when light hits a surface, a portion of the energy is absorbed, and a portion is reflected and bounces in a different direction. Given an incoming ray of light, a BRDF is a formula that outputs the possible directions the ray can bounce, and the amount of energy reflected.

The `StandardMaterial`'s fragment shader loops over the light of lights in the scene, and accumulates each light's interaction with the BRDF described by the material's properties, giving the final color of the pixel.

There are different types of BRDFs. Light might bounce equally in many directions (diffuse), towards one general direction (glossy), or even a perfect reflection in one direction (mirror). The glossy and mirror cases are typically referred to as specular BRDFs, as opposed to diffuse BRDFs. Diagrams: https://en.wikipedia.org/wiki/Bidirectional_reflectance_distribution_function#Models.

Ofentimes real-world materials are not perfectly diffuse or perfectly specular, but a combination of the two. A common way to classify materials is into "metals" and "non-metals" (dielectrics). Metals are actual metals like silver, gold, copper, etc, while dielectrics are everything else including plastic, wood, stone, rubber, etc. Metals have only a specular BRDF, while dielectrics have a combination of diffuse and specular BRDFs.

Bevy's `StandardMaterial` is built to represent arbitrary real-world materials using a combination of multiple diffuse and specular BRDFs mixed together with a set of weights based on the `metallic` property. A subset of more complex materials like wax, cloth, and gemstones that have effects like subsurface scattering, sheen, and refraction, can be modelled with additional properties.

Showcase screenshots: https://bevy.org/examples/3d-rendering/pbr and other examples

## Property Reference

Each property has both a value, and an optional texture map.

TODO: Explain and showcase each StandardMaterial property
