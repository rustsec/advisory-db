```toml
[advisory]
id = "RUSTSEC-2026-0387"
package = "crypto-bigint"
date = "2026-10-23"
url = "https://github.com/RustCrypto/crypto-bigint/pull/1365"
categories = ["crypto-failure"]
license = "CC-BY-4.0"

[versions]
patched = [">= 0.7.6"]
```

# Multiple timing variability issues in operations intended to be constant-time

## Summary

Several operations intended to be constant-time could execute secret-dependent
branches, depending on the compiler and target CPU. These include subtraction,
zero counting, division, and Montgomery multiplication.

Additionally, `UintRef` equality could terminate at the first unequal limb,
and `BoxedMontyForm::invert` branched on whether an inverse exists.

## Impact

Applications using affected operations on secret integers may leak information
through timing or microarchitectural side channels. This includes information
about intermediate arithmetic values, matching limbs, and invertibility.

Such leakage could potentially enable recovery of cryptographic secrets.
Practical exploitability depends on the downstream algorithm, compiler, target
CPU, and an attacker's ability to observe the side channel.

## Resolution

Upgrade to v0.7.6, which removes the identified branches, updates `ctutils` to
include its constant-time fixes, and adds BINSEC-based regression checks using
`cargo checkct` for arithmetic on multiple targets.
