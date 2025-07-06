+++
title = "Plugins"
insert_anchor_links = "right"
[extra]
weight = 0
+++

WIP

When a plugin is added though `App::add_plugins`, the app calls `Plugin::build`, and the plugin typically accesses and configures the world.  Then, when the app is run, a few other plugin life-cycle functions are called, and finally we enter the run loop:
+ The app polls `Plugin::finished` until all the added plugins return `true`.
+ The app calls `Plugin::finish` on all plugins.
+ The app calls `Plugin::cleanup` an all plugins. 
+ The app calls the run loop function on itself.
