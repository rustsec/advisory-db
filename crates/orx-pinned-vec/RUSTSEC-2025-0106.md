```toml
[advisory]
id = "RUSTSEC-2025-0106"
package = "orx-pinned-vec"
date = "2025-10-21"
url = "https://github.com/orxfun/orx-pinned-vec/issues/52"
references = ["https://github.com/orxfun/orx-pinned-vec/pull/53"]
informational = "unsound"
categories = ["memory-corruption"]
keywords = ["undefined-behavior", "soundness"]
aliases = ["GHSA-h5j3-crg5-8jqm"]

[affected.functions]
"orx_pinned_vec::utils::slice::index_of_ptr" = ["< 3.21.0"]

[versions]
patched = [">= 3.21.0"]
```

# Undefined behavior in index_of_ptr with empty slices

The safe function `index_of_ptr` causes undefined behavior when called with an empty slice.

The issue occurs in the line `ptr.add(slice.len() - 1)` which underflows when `slice.len()` is 0, creating a pointer with a massive offset. According to Rust's safety rules, creating such a pointer causes immediate undefined behavior.
