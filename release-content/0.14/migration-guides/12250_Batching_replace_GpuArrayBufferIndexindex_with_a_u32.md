`GpuArrayBufferIndex::index` is now a `u32` instead of a `NonMaxU32`, since restricting the number isn't necessary anymore. Please update any usage to use `u32` instead.
