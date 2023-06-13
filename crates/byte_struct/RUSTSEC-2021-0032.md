```toml
[advisory]
id = "RUSTSEC-2021-0032"
package = "byte_struct"
aliases = ["CVE-2021-28033", "GHSA-8fgg-5v78-6g76"]
cvss = "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H"
date = "2021-03-01"
url = "https://github.com/wwylele/byte-struct-rs/issues/1"
categories = ["memory-corruption"]
keywords = ["memory-safety"]

[versions]
patched = [">= 0.6.1"]
```

# Deserializing an array can drop uninitialized memory on panic

The `read_bytes_default_le` function for `[T; n]` arrays, used to deserialize
arrays of `T` from bytes created a `[T; n]` array with `std::mem::uninitialized`
and then called `T`'s deserialization method.

If `T`'s deserialization method panicked, the uninitialized memory could drop
invalid objects.

This flaw was corrected in `a535678` by removing the unsafe block and using
a `.map` function to deserialize each element of the array instead.
