In rendering, **depth of field** is an effect that mimics the [limitations of physical lenses]((https://en.wikipedia.org/wiki/Depth_of_field)).
By virtue of the way light works, lens (like that of the human eye or a film camera) can only focus on objects that are within a specific range (depth) from them, causing all others to be blurry and out of focus.

Bevy now ships with this effect, implemented as a post-processing shader.
There are two options available: a fast Gaussian blur or a more physically accurate hexagonal bokeh technique.
The bokeh blur is generally more aesthetically pleasing than the Gaussian blur, as it simulates the effect of a camera more accurately. The shape of the bokeh circles are determined by the number of blades of the aperture. In our case, we use a hexagon, which is usually considered specific to lower-quality cameras.

TODO: add image

The blur amount is generally specified by the [f-number](https://en.wikipedia.org/wiki/F-number), which we use to compute the [focal length](https://en.wikipedia.org/wiki/Focal_length) from the film size and [field-of-view](https://en.wikipedia.org/wiki/Field_of_view). By default, we simulate standard cinematic cameras with an f/1 f-number and a film size corresponding to the classic Super 35 camera. The developer can customize these values as desired.

To see how this new API, please check out the dedicated [`depth_of_field` example](https://github.com/bevyengine/bevy/blob/main/examples/3d/depth_of_field.rs).
