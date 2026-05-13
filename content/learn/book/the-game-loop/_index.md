+++
title = "The Game Loop"
template = "docs.html"
insert_anchor_links = "right"
[extra]
weight = 4
status = 'hidden'
+++

At the heart of every game is a "game loop": a continuously running for-loop which reads inputs, calls logic and updates the renderer.

This chapter explains the basics of how that works in Bevy, the libraries we depend on, scheduling systems and how to customize the game loop to meet unusual needs.
