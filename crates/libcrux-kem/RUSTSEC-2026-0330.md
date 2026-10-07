```toml
[advisory]
id = "RUSTSEC-2026-0330"
package = "libcrux-kem"
date = "2026-09-28"

url = "https://github.com/celabshq/libcrux/pull/1595"
cvss = "CVSS:4.0/AV:L/AC:L/AT:P/PR:H/UI:N/VC:N/VI:N/VA:H/SC:N/SI:N/SA:N"

[affected.functions]
"libcrux_kem::PublicKey::encapsulate_derand" = ["<= 0.0.9"]

[versions]
patched = [">= 0.0.10"]
```

# Hybrid Encapsulation from Seed Panics on Short Seed

For a hybrid KEM public key of type `PublicKey::WingKemDraft06` or `PublicKey::X25519MlKem768Draft06`, the `PublicKey::encapsulate_derand` function would panic in an indexing operation on a seed input of length shorter than 32 bytes.

# Impact

Applications encapsulating with an attacker controlled seed value could be made to panic. Since the encapsulation seed should be considered a secret of the encapsulating party for the KEM to remain secure, an application should never take the seed value from a potentially attacker controlled source.

# Mitigation

With release of version `0.0.10` of `libcrux-kem` this bug has been fixed and the serialization functions return `InvalidPrivateKey` and `InvalidPublicKey` errors on invalid input buffer lengths.

We recommend users upgrade to `libcrux-kem` version `0.0.10`.
