+++
title = "Resources, Queries, and Commands"
insert_anchor_links = "right"
[extra]
weight = 4
status = 'hidden'
+++

TODO:
- Resources are global state singletons
- Queries fetch data (entities and their components) from the ECS that match a pattern
	- It's a lot like a `SELECT col1, col2 from ECS`
- Commands are an interface for write operations to the ECS
	- They are used in systems
	- You can do other things with them too, but that's covered in the main commands section
