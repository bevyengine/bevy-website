+++
title = "Why Should You Use Bevy?"
insert_anchor_links = "right"
[extra]
weight = 2
status = 'hidden'
+++

TODO

You should use Bevy if:
- You like Rust (you just like it)
- You have something particularly well-suited to ECS (factorio)
- You like ECS (you just like it)
- Highly modular and configurable! (you value being able to mess with your tools)
- You want to do something weird (CAD software, scientific simulation, strange game concept)
	- Bevy is highly customizable to weirder needs
- You like open source
	- Avoid vendor lock-in
	- Avoid large licensing fees
	- You just like it

You should not use Bevy if:
- You want to ship (relatively traditional) games quickly
	- Unity/Unreal/Godot will have much faster quick starts for common game types: platformers, FPS, strategy, etc.
- You need high stability
	- Bevy iterates quickly and often has breaking changes, although it also has good migration guides
	- Many serious users upgrade bevy on existing projects, but the work is non-trivial for large projects