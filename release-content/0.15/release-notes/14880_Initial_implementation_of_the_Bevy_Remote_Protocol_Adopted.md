We have introduced the Bevy Remote Protocol, which allows the ECS of a running
Bevy application to be interacted with remotely. This can be used, for example,
to inspect and edit entities and their components at runtime. We anticipate 
that this will be used primarily to create things like inspectors for editing
and debugging.

The default methods included with the associated plugin include:
- getting the serialized values of a set of components from an entity;
- performing a query for all entities matching a set of components and retrieving
  their associated values;
- creating a new entity with a given set of component values;
- despawning an entity;
- inserting a set of components into an entity;
- removing a set of components from an entity;
- reparenting one or more entities;
- listing the components registered in the ECS or present on an entity.

Details on these methods are available in the `bevy_remote` module documentation.

The functionality itself is split up between plugins; the `RemotePlugin` handles 
the processing of remote requests and is separate from the transport.  An HTTP 
transport is provided by default by the `RemoteHttpPlugin`.
