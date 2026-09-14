```toml
[advisory]
id = "RUSTSEC-2026-0285"
package = "rustls"
date = "2026-09-14"
url = "https://github.com/rustls/rustls/security/advisories/GHSA-2mjx-qc3c-rqvc"
categories = ["crypto-failure"]
aliases = ["GHSA-2mjx-qc3c-rqvc"]
cvss = "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:L/I:N/A:N"

[versions]
patched = [">= 0.23.45"]
unaffected = ["< 0.23.13"]
```

# TLS 1.3 handshake messages incorrectly accepted across encryption level boundaries

Rustls accepted TLS 1.3 handshake messages sent at the wrong encryption level
when they followed a key-changing message in the same record. For example,
a plaintext `EncryptedExtensions` message packed into the same record as the
`ServerHello` was accepted.

RFC 8446 section 5.1 requires that handshake messages do not span key changes,
and that implementations terminate the connection with an "unexpected_message"
alert if they do.

The handshake transcript is still authenticated, so a network-position attacker
cannot use this to alter or complete a handshake; the practical effect is that
a peer could send handshake messages that should be encrypted in plaintext
without rustls rejecting the connection.

This is functionally the same bug as Go's
[GO-2026-4340](https://pkg.go.dev/vuln/GO-2026-4340) (CVE-2025-61730).
