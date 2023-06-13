```toml
[advisory]
id = "RUSTSEC-2018-0013"
package = "safe-transmute"
aliases = ["CVE-2018-21000", "GHSA-2v78-j59h-fmpf"]
cvss = "CVSS:3.0/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H"
date = "2018-11-27"
keywords = ["memory-corruption"]
url = "https://github.com/nabijaczleweli/safe-transmute-rs/pull/36"

[versions]
patched = [">= 0.10.1"]
unaffected = ["< 0.4.0"]
```

# Vec-to-vec transmutations could lead to heap overflow/corruption

Affected versions of this crate switched the length and capacity arguments in the Vec::from_raw_parts() constructor,
which could lead to memory corruption or data leakage.

The flaw was corrected by using the constructor correctly.
