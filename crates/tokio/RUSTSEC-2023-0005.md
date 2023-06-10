```toml
[advisory]
id = "RUSTSEC-2023-0005"
package = "tokio"
date = "2023-01-11"
url = "https://github.com/tokio-rs/tokio/issues/5372"
categories = ["memory-exposure"]
informational = "unsound"
aliases = ["GHSA-4q83-7cq4-p6wg"]

[versions]
patched = [">= 1.18.5, < 1.19.0", ">= 1.20.4, < 1.21.0", ">= 1.24.2"]
unaffected = ["< 0.2.0"]
```

# `tokio::io::ReadHalf<T>::unsplit` is Unsound

`tokio::io::ReadHalf<T>::unsplit` can violate the `Pin` contract

The soundness issue is described in the [tokio/issues#5372](https://github.com/tokio-rs/tokio/issues/5372)

Specific set of conditions needed to trigger an issue (a !Unpin type in ReadHalf)
is unusual, combined with the difficulty of making any arbitrary use-after-free
exploitable in Rust without doing a lot of careful alignment of data types in
the surrounding code.

The `tokio` feature `io-util` is also required to be enabled to trigger this
soundness issue.

Thanks to zachs18 reporting the issue to Tokio team responsibly and taiki-e
and carllerche appropriately responding and fixing the soundness bug.

Tokio before 0.2.0 used `futures` 0.1 that did not have `Pin`, so it is not
affected by this issue.
