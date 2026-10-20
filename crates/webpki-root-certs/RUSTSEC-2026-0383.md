```toml
[advisory]
id = "RUSTSEC-2026-0383"
package = "webpki-root-certs"
date = "2026-10-19"
url = "https://github.com/rustls/webpki-roots/pull/133"
references = [
    "https://emudhra.we-have-a-problem.com/",
    "https://bugzilla.mozilla.org/show_bug.cgi?id=2083449",
]
categories = ["crypto-failure"]
keywords = ["certificate", "distrust", "x509"]
aliases = ["CVE-2026-111943"]

[versions]
patched = [">= 1.0.10"]
unaffected = ["= 0.26.11"]
```

# Bundled root certificates allow trust in distrusted emSign intermediates

Affected versions include `emSign ECC Root CA - G3`, which can allow certificate
paths through distrusted emSign intermediates.

Affected versions may accept certificate paths containing distrusted emSign
intermediate public keys. The private keys corresponding to these intermediates
are now well known.

Version 1.0.10+ removes this root.

Version 0.26.11 is a compatibility wrapper that re-exports
`parent::TLS_SERVER_ROOT_CERTS` from its `webpki-root-certs` 1.x dependency; it
contains no separate root certificate list. Its resolved 1.x dependency must be
updated to 1.0.10 or later.
