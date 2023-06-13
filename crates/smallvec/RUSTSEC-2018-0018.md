```toml
[advisory]
date = "2018-09-25"
id = "RUSTSEC-2018-0018"
package = "smallvec"
informational = "unsound"
url = "https://github.com/servo/rust-smallvec/issues/126"
aliases = ["CVE-2018-25023", "GHSA-55m5-whcv-c49c", "GHSA-66p5-j55p-32r9"]

[versions]
patched = [">= 0.6.13"]
```

# smallvec creates uninitialized value of any type

Affected versions of this crate called `mem::uninitialized()` to create values of a user-supplied type `T`.
This is unsound e.g. if `T` is a reference type (which must be non-null and thus may not remain uninitialized).
 
The flaw was corrected by avoiding the use of `mem::uninitialized()`, using `MaybeUninit` instead.
