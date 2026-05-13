```toml
[advisory]
id = "RUSTSEC-2026-0133"
package = "auto_vec"
date = "2026-05-02"
url = "https://github.com/lluvz/AutoVec/issues/1"
informational = "unsound"
categories = ["memory-corruption"]
keywords = ["out-of-bounds", "pointer-arithmetic"]

[versions]
patched = []
```

# Invalid pointer arithmetic in `iter()` and `iter_mut()`

The `iter()` and `iter_mut()` APIs compute
`current = (&children[0] as *const *const RawAutoChild).sub(1)`, which
performs pointer subtraction going before the start of the allocation. This
is undefined behavior per Rust's pointer arithmetic rules.

This can be triggered through safe public APIs — `iter()` and `iter_mut()`
— with no `unsafe` required from the caller.
