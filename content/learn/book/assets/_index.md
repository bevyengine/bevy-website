+++
title = "Assets"
template = "docs.html"
insert_anchor_links = "right"
[extra]
weight = 15
status = 'hidden'
+++

A complete game needs to load and use all sorts of files.
Most of these are tangible pieces of art that you may be familiar with: sounds, sprites, models, or fonts.
But the same machinery can be used for more abstract bits of data loaded from disk:
level layouts, enemy statistics, or even scripts.

In game development, these are called **assets.**
This chapter covers the underlying machinery of assets in Bevy,
helping you structure your game to work with them effectively.

At the scale of a hobbyist who is just starting out, this is quite simple:
simply drop files in an `assets` folder.
The initial [`Bevy's Asset Framework`](./bevy's-asset-framework.md) section of this chapter focus on the basics needed to form a useful mental model and get started with real projects.
