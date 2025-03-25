Code that directly access `Image` data will now need to use unwrap or handle the case where no data is provided.
Behaviour of new_fill slightly changed, but not in a way that is likely to affect anything. It no longer panics and will fill the whole texture instead of leaving black pixels if the data provided is not a nice factor of the size of the image.
