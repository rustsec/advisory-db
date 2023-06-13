```toml
[advisory]
id = "RUSTSEC-2023-0017"
package = "maligned"
date = "2023-03-04"
url = "https://github.com/tylerhawkes/maligned/issues/5"
informational = "unsound"
categories = ["memory-corruption"]
keywords = ["unsound", "alloc", "align"]
aliases = ["GHSA-wm8x-php5-hvq6"]

[versions]
patched = []
unaffected = []

[affected.functions]
"maligned::align_first" = ["*"]
"maligned::align_first_boxed" = ["*"]
"maligned::align_first_boxed_cloned" = ["*"]
"maligned::align_first_boxed_default" = ["*"]
```

# `maligned::align_first` causes incorrect deallocation

`maligned::align_first` manually allocates with an alignment larger than T, and then uses `Vec::from_raw_parts` on that allocation to get a `Vec<T>`.

[`GlobalAlloc::dealloc`](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html#tymethod.dealloc) requires that the `layout` argument must be the same layout that was used to allocate that block of memory.

When deallocating, `Box` and `Vec` may not respect the specified alignment and can cause undefined behavior.
