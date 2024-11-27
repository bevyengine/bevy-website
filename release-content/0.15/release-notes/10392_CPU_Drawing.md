There are many situations where you might want to just set the color of pixels
from CPU code.  Procedural assets, certain art styles, or simply because
it is easier. No need to bother with shaders and materials, when you just want
to change a few specific pixels!

In previous versions of Bevy, this was difficult and tedious. Bevy gives you
access to the raw data bytes of an [`Image`], but you had to compute the byte
offset corresponding to your desired pixel coordinate, make sure to encode your
bytes with respect to the [`TextureFormat`], etc. Very low level!

In Bevy 0.15, there are now user-friendly APIs for reading and writing the
colors of pixels in an [`Image`]. The tricky low-level details are dealt with
for you! You can even use `bevy_color`'s fancy color space APIs!

```rust
fn my_system(mut images: ResMut<Assets<Image>>, mut commands: Commands) {
    // Create a new image.
    // (This is the same as before)
    let mut image = Image::new_fill(
        // 64x64 size
        Extent3d {
            width: 64,
            height: 64,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &Srgba::WHITE.to_u8_array(),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::all(),
    );

    // This is new:

    // Make the pixel at x: 23, y: 32 magenta
    image.set_color_at(23, 32, Color::srgb(1.0, 0.0, 1.0))
        .expect("Error writing color");

    // Set the pixel at 10,10 to a color specified using the Oklch color space:
    image.set_color_at(10, 10, Color::oklch(0.3, 0.2, 0.5))
        .expect("Error writing color");

    // read the bytes of the pixel we just wrote:
    let bytes = image.pixel_bytes(UVec3::new(10, 10, 0)).unwrap();

    // read the (approximate) color back (as sRGB):
    let color = image.get_color_at(10, 10);

    // We could add our new image to Bevy's assets
    // and spawn a sprite to display it:
    commands.spawn(Sprite {
        image: images.add(image),
        ..default()
    });
}
```

Note: The [`Color`]-based methods are lossy. They have to convert to/from the
[`Image`]'s [`TextureFormat`]. Round-trips will not work. The color you read back
will not be identical to the color you wrote.

[`Image`]: https://docs.rs/bevy/0.15.0-rc.3/bevy/prelude/struct.Image.html
[`TextureFormat`]: https://docs.rs/bevy/0.15.0-rc.3/bevy/render/render_resource/enum.TextureFormat.html
[`Color`]: https://docs.rs/bevy/0.15.0-rc.3/bevy/color/enum.Color.html
