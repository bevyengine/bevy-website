`EventWriter::send()` and its family of methods have been renamed to `EventWriter::write()` in order to reduce confusion and increase consistency. The old methods have been deprecated.

|0.15|0.16|
|-|-|
|`EventWriter::send()`|`EventWriter::write()`|
|`EventWriter::send_batch()`|`EventWriter::write_batch()`|
|`EventWriter::send_default()`|`EventWriter::write_default()`|
