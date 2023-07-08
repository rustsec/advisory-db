```toml
[advisory]
id = "RUSTSEC-2023-0045"
package = "memoffset"
date = "2023-06-21"
url = "https://github.com/Gilnaa/memoffset/issues/24"
informational = "unsound"
categories = ["memory-corruption"]
keywords = ["memoffset", "offset"]
aliases = ["GHSA-wfg4-322g-9vqv"]
[affected]
functions = { "memoffset::offset_of" = ["< 0.6.2"] }
[versions]
patched = [">= 0.6.2"]
```

# memoffset allows reading uninitialized memory
memoffset allows attempt of reading data from address `0` with arbitrary type. This behavior is an undefined behavior because address `0` to `std::mem::size_of<T>` may not have valid bit-pattern with `T`. Old implementation dereferences uninitialized memory obtained from `std::mem::align_of`. Older implementation prior to it allows using uninitialized data obtained from `std::mem::uninitialized` with arbitrary type then compute offset by taking the address of field-projection. This may also result in an undefined behavior for "father" that includes (directly or transitively) type that [does not allow to be uninitialized](https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html).

This flaw was corrected by using `std::ptr::addr_of` in <https://github.com/Gilnaa/memoffset/pull/50>.
