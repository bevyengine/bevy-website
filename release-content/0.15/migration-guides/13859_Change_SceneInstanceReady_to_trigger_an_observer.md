If you have a system which read `SceneInstanceReady` events:

> 
`fn ready_system(ready_events: EventReader<'_, '_, SceneInstanceReady>) {`


It must be rewritten as an observer:

> 
`commands.observe(|trigger: Trigger<SceneInstanceReady>| {`


Or, if you were expecting the event in relation to a specific entity or entities, as an entity observer:

> 
`commands.entity(entity).observe(|trigger: Trigger<SceneInstanceReady>| {`
