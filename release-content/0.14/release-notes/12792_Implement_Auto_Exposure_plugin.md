Since Bevy 0.13, you can configure the the EV100 of a camera, which allows you to adjust the exposure of the camera in a physically based way. This also allows you to dynamically change the exposure values for various effects. However, this is a manual process and requires you to adjust the exposure values yourself.

Bevy 0.14 introduces a new `AutoExposurePlugin` that automatically adjusts the exposure of your scene based on the brightness of the scene. This can be useful when you want to create the feeling of a very high dynamic range, since your eyes also adjust to large changes in brightness. Note that this is not a replacement for hand-tuning the exposure values, rather an additional tool that you can use to create dramatic effects when brightness changes rapidly.

[insert image here]

Auto exposure is implemented by making a histogram of the scene's brightness in a post processing step. The exposure is then adjusted based on the average of the histogram. A histogram is used rather than a simple average so that additional filtering can be applied, such as ignoring very bright or very dark pixels, or giving more weight to certain parts of the image. Read the docs of the `AutoExposureSettings` component if you want to know more!
