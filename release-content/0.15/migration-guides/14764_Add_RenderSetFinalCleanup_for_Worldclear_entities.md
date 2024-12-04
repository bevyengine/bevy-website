`World::clear_entities` is now part of `RenderSet::PostCleanup` rather than `RenderSet::Cleanup`. Your cleanup systems should likely stay in `RenderSet::Cleanup`.
