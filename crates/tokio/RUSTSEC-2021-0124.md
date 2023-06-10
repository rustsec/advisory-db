```toml
[advisory]
id = "RUSTSEC-2021-0124"
package = "tokio"
aliases = ["CVE-2021-45710", "GHSA-fg7r-2g4j-5cgr"]
date = "2021-11-16"
url = "https://github.com/tokio-rs/tokio/issues/4225"
categories = ["memory-corruption", "thread-safety"]
keywords = ["race condition"]

[versions]
patched = [">= 1.8.4, < 1.9.0", ">= 1.13.1"]
unaffected = ["< 0.1.14"]

[affected.functions]
"tokio::sync::oneshot::Receiver::close" = ["<= 1.13.0, >= 0.1.14"]
```

# Data race when sending and receiving after closing a `oneshot` channel

If a `tokio::sync::oneshot` channel is closed (via the
[`oneshot::Receiver::close`] method), a data race may occur if the
`oneshot::Sender::send` method is called while the corresponding
`oneshot::Receiver` is `await`ed or calling `try_recv`.

When these methods are called concurrently on a closed channel, the two halves
of the channel can concurrently access a shared memory location, resulting in a
data race. This has been observed to [cause memory corruption][corruption].

Note that the race only occurs when **both** halves of the channel are used
after the `Receiver` half has called `close`. Code where `close` is not used, or where the
`Receiver` is not `await`ed and `try_recv` is not called after calling `close`,
is not affected.

See [tokio#4225][issue] for more details.

[corruption]: https://github.com/tokio-rs/tokio/issues/4225#issuecomment-967434847
[issue]: https://github.com/tokio-rs/tokio/issues/4225
[`oneshot::Receiver::close`]: https://docs.rs/tokio/1.14.0/tokio/sync/oneshot/struct.Receiver.html#method.close
