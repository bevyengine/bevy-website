+++
title = "Apps and Worlds"
insert_anchor_links = "right"
[extra]
weight = 5
status = 'hidden'
+++

TODO:
- Everything we've talked about so far is in a `World`
	- Worlds contain all the entities, components, and resources
	- Queries, systems, and commands are executed on a world
	- In the database model, this is your database
- `App`s are a larger structure that contains a world
	- `App`s handle other things around the world: plugins, windowing, other config, etc.
