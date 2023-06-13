```toml
[advisory]
id = "RUSTSEC-2019-0011"
package = "memoffset"
aliases = ["CVE-2019-15553", "GHSA-rh89-x75f-rh3c"]
cvss = "CVSS:3.0/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N"
date = "2019-07-16"
informational = "unsound"
url = "https://github.com/Gilnaa/memoffset/issues/9#issuecomment-505461490"

[versions]
patched = [">= 0.5.0"]
```

# Flaw in offset_of and span_of causes SIGILL, drops uninitialized memory of arbitrary type on panic in client code

Affected versions of this crate caused traps and/or memory unsafety by zero-initializing references.
They also could lead to uninitialized memory being dropped if the field for which the offset is requested was behind a deref coercion, and that deref coercion caused a panic.

The flaw was corrected by using `MaybeUninit`.
