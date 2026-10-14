```toml
[advisory]
id = "RUSTSEC-2026-0346"
package = "rustls"
date = "2026-10-14"
url = "https://github.com/rustls/rustls/pull/3299"
categories = ["crypto-failure"]
aliases = ["GHSA-c8m5-c952-6cm4"]
cvss = "CVSS:3.1/AV:L/AC:H/PR:H/UI:R/S:U/C:H/I:N/A:N"

[versions]
patched = [">= 0.23.46"]
unaffected = ["< 0.21.0"]
```

# `Debug` impls for client session values expose TLS secrets

The `Debug` implementations of `Tls12ClientSessionValue` and
`Tls13ClientSessionValue` were derived, and so included the TLS 1.2 master
secret or TLS 1.3 resumption PSK in their output.

Custom `ClientSessionStore` implementations that format these values with
`Debug` (for example, when logging) could disclose these secrets to readers of
the logs. This conflicts with rustls's
[key-material redaction policy](https://github.com/rustls/rustls/blob/99f2358cae2954837dbb866faf6727de75489ab9/SECURITY.md#boundary-public-api).

Ordinary connection and built-in session cache `Debug` output is redacted and
is not affected.
