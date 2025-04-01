`Components::get_name` now returns `Option<Cow<'_, str>` instead of `Option<&str>`. This is because it now returns results for queued components. If that behavior is not desired, or you know the component is not queued, you can use `components.get_info().map(ComponentInfo::name)` instead.

Similarly, `ScheduleGraph::conflicts_to_string` now returns `impl Iterator<Item = (String, String, Vec<Cow<str>>)>` instead of `impl Iterator<Item = (String, String, Vec<&str>)>`. Because `Cow<str>` derefs to `&str`, most use cases can remain unchanged.
