+++
title = "Bevy 0.14"
date = 2024-05-17
[extra]
public_draft = 1188
+++

<!-- TODO Intro -->

<!-- more -->

{{ release_notes(version="0.14") }}

## What's Next?

Sure this release was great, but what does the future hold for Bevy?
Peering deep into the mists of time (predictions are *extra* hard when your team is almost all volunteers!), we can see some exciting work taking shape:

- **Better scenes:** Scenes are one of Bevy's core building blocks: designed to be a powerful tool for saving games, authoring levels and creating reusable game objects (whether they're a form or a monster). More features and a unified syntax between assets and code should unblock progress on a UI widget abstraction, tools less boilerplates. Check out the [design doc](TODO) for more information.
- **Relations please?:** Relations (a first-class feature for linking entities together) is wildly desired but remarkably complex, driving features and refactors to our ECS internals. The [working group](https://discord.com/channels/691052431525675048/1237010014355456115) has been patiently laying out what we need to do and why in this [RFC](https://github.com/bevyengine/rfcs/pull/79).
- **Better audio:** Bevy's built-in audio solution has never really hit the right notes. The [Better Audio working group](https://discord.com/channels/691052431525675048/1236113088793677888) is plotting a path forward and exploring [ECS-ified interface](https://github.com/SolarLiner/bevy-kira-components) to the popular [`kira`](https://crates.io/crates/kira) audio backend.
- **Contributing book:** Our documentation on how to contribute is scattered to the four corners of our repositories. By gathering this together, the [Contributing Book working group](https://discord.com/channels/691052431525675048/1236112637662724127) hopes to make it easier to discover and maintain.
- **A curve abstraction:** Curves come up all of the time in game dev, and the mathmagicians that make up the [Curve Crew](https://discord.com/channels/691052431525675048/1236110755212820581) are [designing a trait](https://github.com/bevyengine/rfcs/pull/80) to unify and power them.
- **Better text:** our existing text solution isn't up to the demands of modern UI. We're looking at replacing it with a better solution.

{{ support_bevy() }}
{{ contributors(version="0.14") }}
{{ changelog(version="0.14")}}
