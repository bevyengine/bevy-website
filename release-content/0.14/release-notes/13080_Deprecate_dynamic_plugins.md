`bevy_dynamic_plugin` was a tool added in Bevy's original 0.1 release: intended to serve as a tool for dynamically loading / linking Rust code for use with things like modding.
Unfortunately, this feature didn't see much community uptake, and as a result had a vanishingly small number of contributions to refine and document it over the years.

Combined with a challenging, intrinsically unsafe API that was producing [worrying failures](https://github.com/bevyengine/bevy/issues/13073) for users, we've decided to deprecate `bevy_dynamic_plugin` and will be removing it completely in Bevy 0.15.
If you were a happy user of this, simply copy the rather-small crate into your own project and proceed as before.

We still think that both modding and hot-reloading code for faster development times are valuable use cases that Bevy *should* help support one day.
Our hope is that by removing this as a first-party crate, we can spur on third-party experiments and avoid wasting users' time as they investigate a complex potential solution before concluding that it doesn't yet meet their needs.
