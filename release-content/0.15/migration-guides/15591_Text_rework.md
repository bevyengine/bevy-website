TODO: very breaking

__Accessing text spans by index__

Text sections are now text sections on different entities in a hierarchy, Use the new `TextReader` and `TextWriter` system parameters to access spans by index.

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

__Iterating text spans__

Text spans are now entities in a hierarchy, so the new `UiTextReader` and `UiTextWriter` system parameters provide ways to iterate that hierarchy. The `UiTextReader::iter` method will give you a normal iterator over spans, and `UiTextWriter::for_each` lets you visit each of the spans.
