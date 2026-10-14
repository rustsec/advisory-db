```toml
[advisory]
id = "RUSTSEC-2026-0347"
package = "rustls"
date = "2026-10-14"
url = "https://github.com/rustls/rustls/pull/3300"
categories = ["crypto-failure"]
aliases = ["GHSA-vp88-798v-h67f"]
cvss = "CVSS:3.1/AV:L/AC:L/PR:H/UI:N/S:U/C:L/I:N/A:N"

[versions]
patched = [">= 0.23.46"]
unaffected = ["< 0.23.44"]
```

# Buffered application data sent to outer server after ECH rejection

When a server rejects Encrypted Client Hello (ECH), the client completes the
handshake using the outer ClientHello and authenticates the server against the
ECH config's `public_name`. At the end of that handshake, rustls encrypted and
queued any application data buffered via `ClientConnection::writer()` *before*
queueing the `ech_required` alert.

The application correctly receives
`Error::PeerIncompatible(ServerRejectedEncryptedClientHello)`, but the next
`write_tls()` call (needed to send the alert) also sends the buffered
application data. Plaintext meant for the inner server therefore goes to the
client-facing server named by `public_name`.

That server is authenticated, so this is not exploitable by an unauthenticated
party, but it violates RFC 9849 section 6.1.6, which requires the client to
abort with `ech_required` "before sending any application data to the server".

This affects only clients that:

- use `EchMode::Enable` (GREASE ECH is not affected),
- write application data before the handshake completes, and
- connect to a server that rejects their ECH config (for example, because it is stale).

The underlying ordering bug is older, but before 0.23.44 the client checked
the server certificate against the inner server name rather than `public_name`,
so certificate verification failed first and no data was sent.
