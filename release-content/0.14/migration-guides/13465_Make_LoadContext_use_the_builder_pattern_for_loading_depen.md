Several `LoadContext` method calls will need to be updated:

- `load_context.load_with_settings(path, settings)` => `load_context.loader().with_settings(settings).load(path)`
- `load_context.load_untyped(path)` => `load_context.loader().untyped().load(path)`
- `load_context.load_direct(path)` => `load_context.loader().direct().load(path)`
- `load_context.load_direct_untyped(path)` => `load_context.loader().direct().untyped().load(path)`
- `load_context.load_direct_with_settings(path, settings)` => `load_context.loader().with_settings(settings).direct().load(path)`
- `load_context.load_direct_with_reader(reader, path)` => `load_context.loader().direct().with_reader(reader).load(path)`
- `load_context.load_direct_with_reader_and_settings(reader, path, settings)` => `load_context.loader().with_settings(settings).direct().with_reader(reader).load(path)`
- `load_context.load_direct_untyped_with_reader(reader, path)` => `load_context.loader().direct().with_reader(reader).untyped().load(path)`
