The GLTF extension `KHR_texture_transform` is used to transform a texture before applying it. By reading this extension, Bevy can now support a variety of new workflows.
The one we want to highlight here is the ability to easily repeat textures a set number of times. This is useful for creating textures that are meant to be tiled across a surface. We will show how to do this using Blender, but the same principles apply to any 3D modeling software.

Let's look at an example scene that we've prepared in Blender, exported as a GLTF file and loaded into Bevy. We will first use the most basic shader node setup available in Blender:

![Basic shader node setup](basic_nodes.jpg)

The result is the following scene in Bevy:

![Scene with stretched textures](bevy_no_rep.jpg)

Oh no! Everything is stretched! This is because we have set up our UVs in a way that maps the texture exactly once onto the mesh. There are a few ways to deal with this, but the most convenient is to add shader nodes that scale the texture so that it repeats:

![Repeating shader node setup](rep_nodes.jpg)

The data of the `Mapping` node is the one exported to `KHR_texture_transform`. Look at the part in red. These scaling factors determine how often the texture should be repeated in the material. Tweaking this value for all textures results in a much nicer render:

![Scene with repeated textures](bevy_rep.jpg)
