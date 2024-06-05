Instead of passing `&mut World` to `WorldQuery::init_state` directly, pass in a mutable reference to the struct returned from `World::component_initializer`.
