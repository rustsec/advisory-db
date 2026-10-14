```toml
[advisory]
id = "RUSTSEC-2026-0348"
package = "rustls"
date = "2026-10-14"
url = "https://github.com/rustls/rustls/pull/3297"
categories = ["denial-of-service"]
aliases = ["GHSA-h2v8-394m-p7vv"]
cvss = "CVSS:3.1/AV:N/AC:H/PR:N/UI:N/S:U/C:N/I:N/A:L"

[versions]
patched = [">= 0.23.46"]
unaffected = ["< 0.23.9"]
```

# Brotli certificate decompression can allocate up to 1 GiB before authentication

The built-in Brotli certificate decompressor (enabled by the non-default
`brotli` crate feature) bounded the size of the decompressed output, but
accepted a peer-selected history window. Brotli's large-window extension was
enabled, so a five-byte header could cause an allocation of 1 GiB. A peer can
abort a process whose allocator cannot satisfy that request.

No trusted certificate is required by the attacker. Affected configurations are:

- clients that enable the `brotli` feature and connect to a malicious server, and
- servers that enable the `brotli` feature and request client certificates.

Applications that do not enable the `brotli` feature are not affected.
