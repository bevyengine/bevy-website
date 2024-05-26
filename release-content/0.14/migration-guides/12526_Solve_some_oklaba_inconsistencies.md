If you were creating a Oklaba instance directly, instead of using L, you should use lightness

```rust
// Before
let oklaba = Oklaba { l: 1., ..Default::default() };

// Now
let oklaba = Oklaba { lightness: 1., ..Default::default() };
```

if you were using the function `Oklaba::lch`, now the method is named `Oklaba::lab`
