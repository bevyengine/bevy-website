The `bevy_picking::backend::ray::RayMap::map` method is removed as redundant,
In systems using `Res<RayMap>` replace `ray_map.map()` with `&ray_map.map`
