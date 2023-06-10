```toml
[advisory]
id = "RUSTSEC-2021-0094"
package = "rdiff"
date = "2021-02-03"
url = "https://github.com/dyule/rdiff/issues/3"
categories = ["memory-exposure"]
informational = "unsound"
aliases = ["CVE-2021-45694", "GHSA-2rxc-8f9w-fjq8", "GHSA-q579-9wp9-gfp2"]

[versions]
patched = []
```

# Window can read out of bounds if Read instance returns more bytes than buffer size

`rdiff` performs a diff of two provided strings or files. As part of its reading
code it uses the return value of a `Read` instance to set the length of
its internal character vector.

If the `Read` implementation claims that it has read more bytes than the length
of the provided buffer, the length of the vector will be set to longer than its
capacity. This causes `rdiff` APIs to return uninitialized memory in its API
methods.
