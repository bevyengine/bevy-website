Handling errors in your application can be a touchy topic!
While you're prototyping, you might *want* loud panics that crash your application: quickly alerting you to serious problems rather than making you waste hours debugging why something wasn't working.
But as you move into production, those panics can be embarrassing, frustrating or even dangerous!

The typical approach is to gradually refine your error handling one call site at a time,
gracefully recovering from expected failures, and swapping unhandled problems to logs rather than panicking.
While this works well enough, it can be challenging to ensure that you've caught all of the potential panics,
and in particular, it's very hard to ensure that your *dependencies* aren't going to panic in an edge case.

Moreover, if you're quickly swapping back and forth between development and production (that's Agile baby!),
you might want those panics *back*, rather than having to dig through logs. What a mess!

Bevy 0.16 introduces a new unified paradigm for error handling, to help you ship crash-free games (and other applications!)
without sacrificing the loud-and-fast development that panics enable.

The core ideas are pretty simple:

- Bevy (and libraries built for Bevy) should bubble up errors to the user whenever possible, rather than panicking
- gracefully unwrapping errors should be *easy*, with the help of Rust's [`?` operator]
- the standard "please just log this" error type should always be an [`anyhow`]-style `bevy::ecs::error::Result` trait object
- figuring out what went wrong from the logs should be easy: so we've added [high quality backtraces]
- you should be able to quickly configure your error-handler of last-resort in a single place, using the [`GLOBAL_ERROR_HANDLER`]
- this should work everywhere: in your systems, your observers, your commands, and even your fallible system parameters like [`Single`]

By default, failures result in panics: it's great for prototyping and it works everywhere.
When you're ready to ship to production, turn on Bevy's `configurable_error_handling` feature,
and then set the [`GLOBAL_ERROR_HANDLER`] to the behavior you want.
We even provide a built-in set of built-in logging helpers for you:
making it dead simple to add a `production`-flagged configuration to your application.
