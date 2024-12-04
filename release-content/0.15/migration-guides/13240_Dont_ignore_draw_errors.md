If you were using `RenderCommandResult::Failure` to just ignore an error and retry later, use `RenderCommandResult::Skip` instead.

This wasn’t intentional, but this PR should also help with https://github.com/bevyengine/bevy/issues/12660 since we can turn a few unwraps into error messages now.
