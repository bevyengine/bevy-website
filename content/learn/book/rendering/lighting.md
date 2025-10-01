+++
title = "Lighting"
insert_anchor_links = "right"
[extra]
weight = 2
status = 'hidden'
+++

Once you have a camera, meshes, and materials in your scene, the final entity you need to spawn are light sources.

## Theory

Light sources emit rays of light. These rays bounce around the scene, following different paths, and eventually end up hitting the camera and being visible on screen.

You can classify these paths into two types: direct lighting, and indirect lighting. Direct lighting is a 3-vertex path where a light (x2) emits a ray, it bounces off of one surface (x1), and then hits the camera (x0). Indirect lighting is an N-vertex path (where N>3) where a light (x4) emits a ray, it bounces off of one surface (x3), which then bounces off of another surface (x2), etc, until it eventually hits the camera (x0). Indirect lighting is often referred to as global illumination, and is typically much more expensive to compute.

(TODO: Diagram)

Direct lighting controls what regions are dark and in shadow and what regions are brightly illuminated. It's most important for outdoor scenes. Indirect lighting controls _how dark_ shadowed regions are. Without indirect lighting, shadowed regions will be pitch-black, which is not very physically plausible. Indirect lighting is most important for indoor scenes, where most of the illumination comes from light coming from a window or lamp and bouncing around the scene.

(TODO: Examples of indoor and outdoor scenes with direct/indirect lighting)

Furthermore, paths can further be classified into diffuse or specular lighting, based on what kinds of materials they hit along the way (link the StandardMaterial page). Diffuse lighting determines the overall mood of the scene, while specular lighting provides sharper reflections and makes metallic and translucent materials look realistic. Not adding indirect lighting, and then having metallic or translucent materials that don't look good, is a common beginner mistake.

(TODO: Examples of diffuse and specular lighting in scenes)

The final thing (TODO: better word) to consider is occlusion. Not all paths are possible. Geometry can get in the way of paths, and block them. For direct lighting, this is what determines what people typically think of as shadows. If you applied direct light equally to all parts of a scene, without considering occlusion, you wouldn't have any shadows. For indirect lighting, this is what makes objects appear "grounded" in the scene. If you applied indirect light equally to all aprts of a scene, without considering occlusion, objects would appear to "float". Applying indirect occlusion darkens the corners of a scene, and makes objects appear more "grounded".

(TODO: Screenshot without/without shadow maps, with/without indirect occlusion)

## Workflow

Lighting a scene and making it look "good" can be tricky. Bevy provides a large variety of different lighting systems with different tradeoffs and advantages. There is no one perfect lighting system that looks and performs well in all games. Consider which of the available options best fit your game's style, gameplay, and target audience.

Generally, there are "dynamic" and "baked" lighting methods. Dynamic methods are more expensive, and have worse quality, but can react in realtime as objects move around and light sources change. Baked methods are cheap to apply at runtime, and have better quality, but require a time-consuming process to pre-calculate (bake) the scene's lighting ahead of time. This means that if objects move around or the light sources change, the lighting won't adapt, and will look wrong. Additionally, storing the pre-calculated scene lighting takes some disk space.

A puzzle game with mainly static, indoor scenes, targeting laptops with weaker GPUs, might want to rely on mostly baked lighting methods. An FPS with lots of dynamic objects, targeting higher-end systems, might want to rely on more dynamic lighting methods. World size is also a factor. For larger games with many big levels, the lengthy baking process and large amount of disk space consumed can be good reasons to avoid baked lighting. The Lighting Systems section (TODO: link) will provide an overview of the different lighting systems available.

Either way, you'll probably want to start by referencing some real-world scenes. Identify the main source of light in your scene. For outdoor scenes, this will typically be the sun. For indoor scenes, this might be an overhead light fixture (TODO: wordy) or lamp. Spawn direct light sources like `DirectionalLight`, `PointLight`, or `SpotLight` to represent your main light sources. As always with PBR, use light intensities that match real-world measurements to keep things consistent.

After setting up direct lighting, the next step is to add some indirect lighting. Unlike direct lighting, indirect lighting in Bevy tends to be split into separate systems for diffuse and specular indirect light, as indirect lighting is much more expensive and difficult to calculate. Additionally, many systems are complementary, and handle e.g. only dynamic or static meshes, or provide additional small-scale detail on top of coarser systems. You'll likely want to use multiple forms of indirect lighting. Either way, stick to trying to match real-world or ground-truth references.

Finally, you'll want to handle occlusion. For direct lighting, enable a low amount (1-4) of shadow maps. For indirect lighting, occlusion will either be part of the baking process, approximated with something like screen space ambient occlusion, or ignored altogether (for performance reasons).

Also make sure that your camera settings (exposure, bloom, tonemapping) fit the scene.

## Lighting Systems

### Direct Lighting

#### DirectionalLight
* Cascades

#### PointLight

#### SpotLight

#### Shadow mapping
* PCF and PCSS
* Shadow map resolution
* NotShadowCaster/Receiver

### Indirect Lighting
* Hierachy of light (which sources override and take precedence over others)

#### AmbientLight

#### EnvironmentMapLight
* Global vs local
* GeneratedEnvironmentMapLight

#### IrradianceVolume

#### Lightmap

#### Screenspace Methods

#### Faking GI with PointLights

### Volumetrics

#### DistanceFog
* Non-PBR

#### FogVolume

#### VoumetricFog
* FogVolume, VolumetricLight

### Solari
