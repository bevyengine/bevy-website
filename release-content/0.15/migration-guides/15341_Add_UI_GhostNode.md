Any code that previously relied on `Parent`/`Children` to iterate UI children may now want to use `bevy_ui::UiChildren` to ensure ghost nodes are skipped, and their first descendant Nodes included.

UI root nodes may now be children of ghost nodes, which means `Without<Parent>` might not query all root nodes. Use  `bevy_ui::UiRootNodes` where needed to iterate root nodes instead.
