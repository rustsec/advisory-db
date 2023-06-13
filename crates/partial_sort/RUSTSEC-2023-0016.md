```toml
[advisory]
id = "RUSTSEC-2023-0016"
package = "partial_sort"
date = "2023-02-20"
url = "https://github.com/sundy-li/partial_sort/issues/7"
informational = "unsound"
categories = ["memory-exposure"]
keywords = ["out-of-bounds read"]
aliases = ["GHSA-5x36-7567-3cw6"]

[versions]
patched = [">= 0.2.0"]
```

# Possible out-of-bounds read in release mode

Affected versions of this crate were using a debug assertion to validate the
`last` parameter of `partial_sort()`. This would allow invalid inputs to cause
an out-of-bounds read instead of immediately panicking, when compiled without
debug assertions.

All writes are bounds-checked, so the out-of-bounds memory access is read-only.
This also means that the first attempted out-of-bounds write will panic,
limiting the possible reads.

The accessible region is further limited by an initial bounds-checked read
at `(last / 2) - 1`, i.e., it is proportional to the size of the vector.

This bug has been fixed in v0.2.0.
