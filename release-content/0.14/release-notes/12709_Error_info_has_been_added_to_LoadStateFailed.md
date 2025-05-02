Rust prides itself on its error handling, and Bevy has been steadily catching up.
Previously, when checking if an asset was loaded using [`AssetServer::get_load_state`](https://docs.rs/bevy/0.14/bevy/asset/struct.AssetServer.html#method.get_load_state),
all you'd get back was a data-less [`LoadState::Failed`](https://docs.rs/bevy/0.14/bevy/asset/enum.LoadState.html) if something went wrong.
Not very useful for debugging!

Now, a full [`AssetLoadError`](https://docs.rs/bevy/0.14/bevy/asset/enum.AssetLoadError.html) is included, with 14 different variants telling you exactly what went wrong.
Great for troubleshooting, and it opens the door to proper error handling in more complex apps.
