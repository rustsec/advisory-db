```toml
[advisory]
id = "RUSTSEC-2025-0107"
package = "borrowck_sacrifices"
date = "2025-10-21"
url = "https://github.com/alexpyattaev/borrowck_sacrifices/issues/1"
informational = "unsound"
categories = ["memory-exposure"]
keywords = ["uninitialized-memory", "soundness"]
aliases = ["GHSA-xcpm-76hf-c9cc"]

[affected.functions]
"borrowck_sacrifices::unsafe_casts::any_as_u8_slice" = ["< 0.2.0"]

[versions]
patched = [">= 0.2.0"]
```

# Uninitialized memory exposure in any_as_u8_slice

The safe function `any_as_u8_slice` can create byte slices that reference uninitialized memory when used with types containing padding bytes.

The function uses `slice::from_raw_parts` to create a `&[u8]` covering the entire size of a type, including padding bytes. According to Rust's documentation, `from_raw_parts` requires all bytes to be properly initialized, but padding bytes in structs are not guaranteed to be initialized. This violates the safety contract and causes undefined behavior.
