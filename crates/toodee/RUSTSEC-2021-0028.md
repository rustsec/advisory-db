```toml
[advisory]
id = "RUSTSEC-2021-0028"
package = "toodee"
aliases = ["CVE-2021-28028", "CVE-2021-28029", "GHSA-wcvp-r8j8-47pc", "GHSA-xm9m-2vj8-fmfr"]
date = "2021-02-19"
url = "https://github.com/antonmarsden/toodee/issues/13"
categories = ["memory-corruption"]
keywords = ["memory-safety", "double-free"]

[versions]
patched = [">= 0.3.0"]
unaffected = []

[affected]
functions = { "toodee::TooDee::insert_row" = ["< 0.3.0"] }
```

# Multiple memory safety issues in insert_row

When inserting rows from an iterator at a particular index, `toodee` would shift
items over, duplicating their ownership. The space reserved for the new elements
was based on the `len()` returned by the `ExactSizeIterator`.

This could result in elements in the array being freed twice if the iterator
panics. Uninitialized or previously freed elements could also be exposed if the
`len()` didn't match the number of elements.

These issues were fixed in commit `ced70c17` by temporarily setting the length
of the array smaller while processing it and adding assertions on the number
of elements returned by the iterator.
