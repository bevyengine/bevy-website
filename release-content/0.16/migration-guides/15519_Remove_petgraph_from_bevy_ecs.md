Bevy's ECS no longer depends on the `petgraph` crate. As such, usage of `petgraph::graph::DiGraph` has been replaced with `bevy::ecs::schedule::graph::DiGraph`. This mainly affects code that uses the `Dag::graph()` method.

If you require the `petgraph` version of `DiGraph`, you can manually construct it by iterating over all edges and nodes in Bevy's `DiGraph`.
