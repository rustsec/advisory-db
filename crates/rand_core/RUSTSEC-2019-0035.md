```toml
[advisory]
id = "RUSTSEC-2019-0035"
package = "rand_core"
aliases = ["GHSA-mmc9-pwm7-qj5w", "CVE-2020-25576"]
cvss = "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H"
date = "2019-04-19"
informational = "unsound"
url = "https://github.com/rust-random/rand/blob/master/rand_core/CHANGELOG.md#050---2019-06-06"

[affected.functions]
"rand_core::BlockRng::fill_bytes" = ["< 0.4.2"]
"rand_core::BlockRng::next_u64" = ["< 0.4.2"]

[versions]
patched = ["^ 0.3.1", ">= 0.4.2"]
```

# Unaligned memory access

Affected versions of this crate violated alignment when casting byte slices to
integer slices, resulting in undefined behavior.

The flaw was corrected by Ralf Jung and Diggory Hardy.
