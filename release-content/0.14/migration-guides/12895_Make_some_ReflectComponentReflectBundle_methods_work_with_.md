`ReflectComponentFns` and `ReflectBundleFns` have been updated to work with `EntityMut`, as compared to the more restricting `EntityWorldMut`. You will have to update your usage of `ReflectComponentFns::apply`, `ReflectComponentFns::reflect_mut`, and `ReflectBundleFns::apply`.

If you just use `ReflectComponent` and `ReflectBundle`, you will not have change your code because `EntityWorldMut` implements `Into<EntityMut>`.
