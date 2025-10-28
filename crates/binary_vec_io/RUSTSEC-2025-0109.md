```toml
[advisory]
id = "RUSTSEC-2025-0109"
package = "binary_vec_io"
date = "2025-10-21"
url = "https://github.com/RustSec/advisory-db/pull/2428"
informational = "unsound"
categories = ["memory-corruption"]
keywords = ["buffer-overflow", "soundness"]
aliases = ["GHSA-wwxp-hxh6-8gf8"]

[affected.functions]
"binary_vec_io::binary_read_to_ref" = ["<= 0.1.12"]
"binary_vec_io::binary_write_from_ref" = ["<= 0.1.12"]

[versions]
patched = []
```

# Out-of-bounds memory access in binary_read_to_ref and binary_write_from_ref

Safe functions accept a single `&T` or `&mut T` but multiply by `n` to create slices extending beyond allocated memory when `n > 1`.

These functions use `from_raw_parts` to create slices larger than the underlying allocation, violating memory safety.

The binary_vec_io repository is archived and unmaintained.
