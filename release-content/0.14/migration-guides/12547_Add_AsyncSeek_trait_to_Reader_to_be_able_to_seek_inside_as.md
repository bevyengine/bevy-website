Every custom reader (which previously only needed the `AsyncRead` trait implemented) now also needs to implement the `AsyncSeek` trait to add the seek capability.
