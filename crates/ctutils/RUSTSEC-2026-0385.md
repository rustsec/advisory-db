```toml
[advisory]
id = "RUSTSEC-2026-0385"
package = "ctutils"
date = "2026-10-23"
url = "https://github.com/RustCrypto/utils/pull/1569"
categories = ["crypto-failure"]
related = ["RUSTSEC-2026-0003"]
license = "CC-BY-4.0"

[versions]
patched = [">= 0.4.4"]
```

# Non-constant-time mask generation and `u128` comparisons

## Summary

The `ctutils::Choice` type has mask generation functions for each unsigned
integer size (`u8`, `u16`, `u32`, `u64`, `u128`). These are `const fn` and
therefore can't reuse the `asm!`-backed mask generation in the `cmov` crate
that's otherwise used to ensure constant-time.

In releases prior to v0.4.4, the generated code from these functions could
include a branch on commonly used platforms. This has been corrected in this
release and now verified on several platforms using BINSEC.

Additionally, `u128` comparison operations provided via `ctutils::Choice` and
intended to work in constant-time were generating branches on 32-bit targets.
This has been resolved via a special case implementation of these functions
for such targets.

## Impact

Branch instructions on secret values can potentially leak such values via
covert channels. This can enable attackers to potentially recover such secrets.

## Resolution

This has been resolved by calling `Choice::to_u8()` when generating the mask,
which in turn uses `black_box` as an optimization barrier. Verification that
this resulted in the desired change in the generated assembly across a variety
of target CPU architectures was performed by BINSEC, which now runs
automatically in CI using `cargo checkct`.
