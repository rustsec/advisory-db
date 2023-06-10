```toml
[advisory]
id = "RUSTSEC-2021-0100"
package = "sha2"
date = "2021-09-08"
categories = ["crypto-failure"]
keywords = ["cryptography"]
url = "https://github.com/RustCrypto/hashes/pull/314"
aliases = ["CVE-2021-45696", "GHSA-fc7x-2cmc-8j2g", "GHSA-xpww-g9jx-hp8r"]

[versions]
patched = [">= 0.9.8"]
unaffected = ["< 0.9.7"]
```

# Miscomputed results when using AVX2 backend

The v0.9.7 release of the `sha2` crate introduced a new AVX2-accelerated
backend which was automatically enabled for all x86/x86_64 CPUs where AVX2
support was autodetected at runtime.

This backend was buggy and would miscompute results for long messages
(i.e. messages spanning multiple SHA blocks).

The crate has since been yanked, but any users who upgraded to v0.9.7 should
immediately upgrade to v0.9.8 and recompute any hashes which were previously
computed by v0.9.7.
