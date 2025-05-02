Since **Bevy 0.13**, you can [configure the EV100 of a camera](/news/bevy-0-13/#camera-exposure), which allows you to adjust the exposure of the camera in a physically based way. This also allows you to dynamically change the exposure values for various effects. However, this is a manual process and requires you to adjust the exposure values yourself.

**Bevy 0.14** introduces **Auto Exposure**, which automatically adjusts the exposure of your camera based on the brightness of the scene. This can be useful when you want to create the feeling of a very high dynamic range, since your eyes also adjust to large changes in brightness. Note that this is not a replacement for hand-tuning the exposure values, rather an additional tool that you can use to create dramatic effects when brightness changes rapidly. Check out this video recorded from the [example](https://github.com/bevyengine/bevy/tree/v0.14.0/examples/3d/auto_exposure.rs) to see it in action!

<video controls><source src="auto_exposure.mp4" type="video/mp4"/></video>

Bevy's Auto Exposure is implemented by making a **histogram** of the scene's brightness in a post processing step. The exposure is then adjusted based on the average of the histogram. Because the histogram is calculated using a compute shader, Auto Exposure is **not available on WebGL**. It's also not enabled by default, so you need to add the [`AutoExposurePlugin`](https://docs.rs/bevy/0.14/bevy/core_pipeline/auto_exposure/struct.AutoExposurePlugin.html) to your app.

Auto Exposure is controlled by the [`AutoExposureSettings`](https://docs.rs/bevy/0.14/bevy/core_pipeline/auto_exposure/struct.AutoExposureSettings.html) component, which you can add to your camera entity. You can configure a few things:

* A relative *range* of F-stops that the exposure can change by.
* The *speed* at which the exposure changes.
* An optional **metering mask**, which allows you to, for example, give more weight to the center of the image.
* An optional histogram *filter*, which allows you to ignore very bright or very dark pixels.
