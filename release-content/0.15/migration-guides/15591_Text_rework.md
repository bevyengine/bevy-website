The `Text` API in Bevy has been overhauled in several ways as part of Bevy 0.15.
There are several major changes to consider:

- `ab_glyph` has been replaced with `cosmic-text`. These changes are mostly internal and the majority of users will not interact with either text backend directly.
- each text section is now stored as a distinct entity within the standard hierarchy, rather than as a `Vec<TextSection>` on the `Text` component. Children of `Text`/`Text2d` entities with `TextSpan` components will act as additional text sections.
- like other aspects of Bevy's API, required components have replaced bundles

## `TextBundle` and text styling

`TextBundle` has been removed.
Add the `Text` component to set the string displayed.

`TextLayout`, `TextFont` and `TextColor` are required components for `Text`,
and are automatically added whenever `Text` is.
Set those values to change the text section's style.

Like elsewhere in Bevy, there is no style inheritance. Consider writing your own abstraction for this if this is something you'd like to use.

To control the layout of a `Text` section, modify the properties of its `Node`.

## Accessing text spans by index

Previously, text sections were elements of a vector stored within `Text`.
Now, they are stored as distinct entities under the same `Parent`.
You can use the new `TextUiReader` and `TextUiWriter` system parameters to conveniently access text spans by index.

Before:

```rust
fn refresh_text(mut query: Query<&mut Text, With<TimeText>>, time: Res<Time>) {
    let text = query.single_mut();
    text.sections[1].value = format_time(time.elapsed());
}
```

After:

```rust
fn refresh_text(
    query: Query<Entity, With<TimeText>>,
    mut writer: UiTextWriter,
    time: Res<Time>
) {
    let entity = query.single();
    *writer.text(entity, 1) = format_time(time.elapsed());
}
```

2D equivalents (`Text2dReader` and `Text2dWriter`) also exist.

## Internal layout information

`TextBundle` additional fields have been moved into the `TextNodeFlags` component,
while `Text2dBundle`'s additional fields turned into the `TextBounds` and `Anchor` components.
