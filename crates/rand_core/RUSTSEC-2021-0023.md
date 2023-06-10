```toml
[advisory]
id = "RUSTSEC-2021-0023"
package = "rand_core"
aliases = ["CVE-2021-27378", "GHSA-w7j2-35mf-95p7"]
cvss = "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H"
date = "2021-02-12"
url = "https://github.com/rust-random/rand/pull/1096"
categories = ["crypto-failure"]
keywords = []

# Optional: metadata which narrows the scope of what this advisory affects
[affected.functions]
"rand_core::le::read_u32_into" = ["< 0.6.2, >= 0.6.0"]
"rand_core::le::read_u64_into" = ["< 0.6.2, >= 0.6.0"]

[versions]
patched = [">= 0.6.2"]
unaffected = ["< 0.6.0"]
```

# Incorrect check on buffer length when seeding RNGs

Summary: rand_core::le::read_u32_into and read_u64_into have incorrect checks on the source buffer length, allowing the destination buffer to be under-filled.

Implications: some downstream RNGs, including Hc128Rng (but not the more widely used ChaCha*Rng), allow seeding using the SeedableRng::from_seed trait-function with too short keys.
