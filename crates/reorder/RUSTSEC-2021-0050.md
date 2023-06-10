```toml
[advisory]
id = "RUSTSEC-2021-0050"
package = "reorder"
aliases = ["CVE-2021-29941", "CVE-2021-29942", "GHSA-3h87-v52r-p9rg", "GHSA-jpwg-6gf5-5vh9"]
cvss = "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:L/I:L/A:L"
date = "2021-02-24"
url = "https://github.com/tiby312/reorder/issues/1"
keywords = ["memory-corruption", "out-of-bounds"]

[versions]
patched = [">= 1.1.0"]
```

# swap_index can write out of bounds and return uninitialized memory

`swap_index` takes an iterator and swaps the items with their corresponding
indexes. It reserves capacity and sets the length of the vector based on the
`.len()` method of the iterator.

If the `len()` returned by the iterator is larger than the actual number of
elements yielded, then `swap_index` creates a vector containing uninitialized
members. If the `len()` returned by the iterator is smaller than the actual
number of members yielded, then `swap_index` can write out of bounds past
its allocated vector.

As noted by the Rust documentation, [`len()`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html#method.len)
and `size_hint()` are primarily meant for optimization and incorrect values
from their implementations should not lead to memory safety violations.


# Patch

A new version crate was pushed that marks this function as unsafe.

reorder = "1.1.0"

Previous versions have also been yanked from crates.io.
