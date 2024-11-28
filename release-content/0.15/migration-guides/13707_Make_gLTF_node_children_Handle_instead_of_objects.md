If accessing children, use `Assets<GltfNode>` resource to get the actual child object.

__Before__

```rs
fn gltf_print_first_node_children_system(gltf_component_query: Query<Handle<Gltf>>, gltf_assets: Res<Assets<Gltf>>, gltf_nodes: Res<Assets<GltfNode>>) {
    for gltf_handle in gltf_component_query.iter() {
        let gltf_root = gltf_assets.get(gltf_handle).unwrap();
        let first_node_handle = gltf_root.nodes.get(0).unwrap();
        let first_node = gltf_nodes.get(first_node_handle).unwrap();
        let first_child = first_node.children.get(0).unwrap();
        println!("First nodes child node name is {:?)", first_child.name);
    }
}
```

__After__

```rs
fn gltf_print_first_node_children_system(gltf_component_query: Query<Handle<Gltf>>, gltf_assets: Res<Assets<Gltf>>, gltf_nodes: Res<Assets<GltfNode>>) {
    for gltf_handle in gltf_component_query.iter() {
        let gltf_root = gltf_assets.get(gltf_handle).unwrap();
        let first_node_handle = gltf_root.nodes.get(0).unwrap();
        let first_node = gltf_nodes.get(first_node_handle).unwrap();
        let first_child_handle = first_node.children.get(0).unwrap();
        let first_child = gltf_nodes.get(first_child_handle).unwrap();
        println!("First nodes child node name is {:?)", first_child.name);
    }
}
```
