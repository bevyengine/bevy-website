`BufferVec` has been renamed to `RawBufferVec` because a new implementation of `BufferVec` has taken its name. The new `BufferVec<T>` no longer requires `T: Pod`, but instead `ShaderType` from the `encase` library.

For most cases you can simply switch to using `RawBufferVec`, but if you have more complex data you may be interested in the new `BufferVec` implementation.
