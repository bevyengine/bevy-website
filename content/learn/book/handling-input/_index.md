+++
title = "Handling Input"
template = "docs.html"
insert_anchor_links = "right"
[extra]
weight = 14
status = 'hidden'
+++

Most developers want their game to be interactive.
If a player provides some kind of input, the game should react or change in some way.
To make this happen, we need to receive that input from the player (usually through a device) and then make the systems in our game react in some way.

The specific devices you want to read input from might also vary depending on what game you want to make.
Thankfully, Bevy can collect input from gamepads, keyboards, mice, or even touch screens.
How you access and use input data from these devices is detailed on the [Using Input page](/learn/book/handling-input/using-input), which is where we'd recommend you start.
