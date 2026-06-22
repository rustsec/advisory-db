```toml
[advisory]
id = "RUSTSEC-2026-0186"
package = "memmap2"
date = "2026-06-20"
url = "https://github.com/RazrFalcon/memmap2-rs/issues/169"
references = ["https://github.com/RazrFalcon/memmap2-rs/pull/170"]
informational = "unsound"
keywords = ["pointer-arithmetic", "out-of-bounds"]

[affected.functions]
"memmap2::Mmap::advise_range" = [">= 0.5.9","< 0.9.11"]
"memmap2::Mmap::unchecked_advise_range" = [">= 0.8.0","< 0.9.11"]
"memmap2::MmapMut::advise_range" = [">= 0.5.9","< 0.9.11"]
"memmap2::MmapMut::flush_async_range" = ["< 0.9.11"]
"memmap2::MmapMut::flush_range" = ["< 0.9.11"]
"memmap2::MmapMut::unchecked_advise_range" = [">= 0.8.0","< 0.9.11"]

[versions]
patched = [">= 0.9.11"]
```

# Unchecked pointer offset in crate `memmap2`

Affected versionf of `memmap2` did not perform enough validation on the `offset` and `len` parameters of
`Mmap::[unchecked_]advise_range()`,
`MmapMut::[unchecked_]advise_ranage()`
and `MmapMut::flush[_async]_range()`.

This can cause undefined behavior due to invalid values being passed to [`pointer::offset()`] and [`pointer::add()`]
when passing an out-of-bounds range to any of the affected functions.

The flaw was corrected in commit [`cee7cf0`] and released in version `0.9.11`.

The invalid pointer is not dereferenced,
but it is passed to the `madvise` and `msync` syscalls and their Windows equivalents.

[`pointer::offset()`]: https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.offset-1
[`pointer::add()`]: https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.add-1
[`cee7cf0`] https://github.com/RazrFalcon/memmap2-rs/pull/170/changes/cee7cf03a9ee095982a3c37b7aac8e3f68f1a00c
