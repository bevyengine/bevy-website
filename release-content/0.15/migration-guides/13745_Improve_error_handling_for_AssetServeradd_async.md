`AssetServer::add_async` can now return a custom error type in its future.
To return to the previous behavior, pass in an `E` generic of `AssetLoadError`.

To support these changes, `AssetLoadError` now has an additional arm that will need to be exhaustively matched against.
