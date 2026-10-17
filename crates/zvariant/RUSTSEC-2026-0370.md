```toml
[advisory]
id = "RUSTSEC-2026-0370"
package = "zvariant"
date = "2026-10-16"
url = "https://github.com/z-galaxy/zbus/issues/2043"
informational = "unmaintained"

[versions]
patched = []
```

# `zvariant` is unmaintained

The `zvariant` crate is no longer maintained as a standalone crate.

It has been merged into [`zbus`](https://crates.io/crates/zbus) starting with version `6.0.0`, as its `wire` module.

Users should migrate to [`zbus`](https://crates.io/crates/zbus), with its default features disabled if they only need the D-Bus wire format. Users of the GVariant format should migrate to [`zgvariant`](https://crates.io/crates/zgvariant).
