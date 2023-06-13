```toml
[advisory]
id = "RUSTSEC-2019-0029"
package = "chacha20"
aliases = ["CVE-2019-25005", "GHSA-j2r6-2m5c-vgh5"]
cvss = "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N"
categories = ["crypto-failure"]
date = "2019-10-22"
url = "https://github.com/RustCrypto/stream-ciphers/pull/64"

[versions]
patched = [">= 0.2.3"]
```

# ChaCha20 counter overflow can expose repetitions in the keystream

The ChaCha20 stream cipher can produce a maximum of 2^32 blocks (~256GB)
before the 32-bit counter overflows. Releases of the `chacha20` crate prior
to v0.2.3 allow generating keystreams larger than this, including seeking
past the limit. When this occurs, the keystream is duplicated, with failure
modes similar to nonce reuse (i.e. exposure of the XOR of two plaintexts).

The v0.2.3 release now panics in this event, rather than exposing the
duplicated keystream. Note this is a "hot fix" solution to the problem
and future releases will pursue returning an error in this case.

Users of the `chacha20poly1305` crate are unaffected by this as this crate
properly asserts the length of the plaintext is less than the maximum allowed
(`P_MAX` as described in RFC 8439 Section 2.8).
