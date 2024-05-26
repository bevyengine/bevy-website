`GpuArrayBufferIndex::index` is now a u32 instead of a `NonMaxU32`. Remove any calls to `NonMaxU32::get` on the member.
