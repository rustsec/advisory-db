```toml
[advisory]
id = "RUSTSEC-2026-0349"
package = "rustls"
date = "2026-10-14"
url = "https://github.com/rustls/rustls/pull/3355"
categories = ["crypto-failure"]
aliases = ["GHSA-w44r-93j7-qv28"]
cvss = "CVSS:3.1/AV:N/AC:H/PR:N/UI:N/S:U/C:L/I:N/A:N"

[versions]
patched = [">= 0.23.46"]
```

# Unauthenticated ServerHello can downgrade later connections to classical key exchange

The client recorded the server-selected key exchange group for each server
name when processing the ServerHello, before the server was authenticated.
This record is used to predict which key share to send in future ClientHellos
to that server name.

A network attacker can answer one connection attempt with a ServerHello
selecting X25519, and then close the connection. Later ClientHellos for that
name carry only an X25519 key share, while `supported_groups` still lists
X25519MLKEM768 first. If the genuine server accepts that share rather than
sending a HelloRetryRequest for its preferred hybrid group, the handshake
completes with classical-only key exchange and no error. This persists for as
long as the entry stays in the client's session store.

The impact is loss of post-quantum confidentiality for affected connections
("harvest now, decrypt later"). There is no key compromise and no
authentication bypass.

The following are not affected:

- connections to servers which send a HelloRetryRequest for their preferred group,
- clients with resumption disabled (as nothing is stored), and
- other server names.

This behaviour is discussed in
[draft-ietf-tls-key-share-prediction](https://www.ietf.org/archive/id/draft-ietf-tls-key-share-prediction-01.html#name-security-considerations),
so the practical impact depends largely on server policy.
