When accessing a live entity through a query fails even though it should succeed, the first step in debugging is to find out what components the entity actually has. `QueryEntityError` now tells you in the error message:

```
QueryDoesNotMatch(0v1 with components Sprite, Transform, GlobalTransform, Visibility, InheritedVisibility, ViewVisibility, SyncToRenderWorld)
```
