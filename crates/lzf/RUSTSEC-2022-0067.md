```toml
[advisory]
id = "RUSTSEC-2022-0067"
package = "lzf"
date = "2022-10-22"
url = "https://github.com/badboy/lzf-rs/issues/9"
informational = "unsound"
keywords = ["uninitialized-memory"]
aliases = ["GHSA-5m39-wx2q-mxg3"]

[versions]
patched = [">= 0.3.2"]

[affected]
functions = { "lzf::compress" = ["< 0.3.2"], "lzf::decompress" = ["< 0.3.2"] }
```

#  Invalid use of `mem::uninitialized` causes `use-of-uninitialized-value`

The compression and decompression function used `mem:uninitialized`
to create an array of uninitialized values, to later write values into it.
This later leads to reads from uninitialized memory.

The flaw was corrected in commit b633bf265e41c60dfce3be7eac4e4dd5e18d06cf
by using a heap-allocated `Vec` and removing out use of `mem::uninitialized`.
The fix was released in v0.3.2 and v1.0.0

Subsequently the crate was deprecated and its use is discouraged.
