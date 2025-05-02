The `affine_to_square()` **shader** function has been renamed to `affine3_to_square`, in order to give room for `affine2_to_square`. Please update your import statements and usages accordingly. (Note that this is not Rust, but instead WGSL.)

```wgsl
// 0.13
#import bevy_render::maths::affine_to_square

// 0.14
#import bevy_render::maths::affine3_to_square
```
