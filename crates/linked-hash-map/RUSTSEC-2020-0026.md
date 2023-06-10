```toml
[advisory]
id = "RUSTSEC-2020-0026"
package = "linked-hash-map"
aliases = ["CVE-2020-25573", "GHSA-r43h-gmrm-h5c9"]
cvss = "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H"
date = "2020-06-23"
informational = "unsound"
url = "https://github.com/contain-rs/linked-hash-map/pull/100"

[versions]
patched = [">= 0.5.3"]
```

# linked-hash-map creates uninitialized NonNull pointer

Affected versions of this crate called `mem::uninitialized()` to create a `NonNull<T>`,
which is undefined behavior.
 
The flaw was corrected by avoiding the use of `mem::uninitialized()`.
