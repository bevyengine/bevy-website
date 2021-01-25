+++
title = "Code"
weight = 1
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
[extra]
long_title = "Contributing Code"
+++

Would you like to contribute code to Bevy?  Here's how!

# How to Contribute

1. Fork the [`bevyengine/bevy` repository on GitHub](bevy), you'll need to create a GitHub account if you don't have one already.*
2. Make your changes in a local clone of your fork
4. For a higher chance of CI passing the first time, consider running the `tools/ci` script from the root of the bevy repository. Or you can run the commands manually:
   1. `cargo +nightly fmt --all`
   2. `cargo clippy --all-targets --all-features -- -D warnings -A clippy::type_complexity -A clippy::manual-strip`
5. Push your changes to your fork and open a [Pull Request][pull]
6. Respond to any CI failures or review feedback.

Remember to follow Bevy's [Code of Conduct][coc], and thanks for contributing!

*The same steps apply for any other repository in the [Bevy organization][bevyorg] that you would like to contribute to.


[bevy]: https://github.com/bevyengine/bevy
[bevyorg]: https://github.com/bevyengine
[coc]: https://github.com/bevyengine/bevy/blob/master/CODE_OF_CONDUCT.md
[pull]: https://github.com/bevyengine/bevy/compare