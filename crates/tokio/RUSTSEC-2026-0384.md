```toml
[advisory]
id = "RUSTSEC-2026-0384"
package = "tokio"
date = "2026-10-21"
url = "https://github.com/tokio-rs/tokio/pull/8692"
categories = ["memory-corruption", "thread-safety"]
keywords = ["race condition", "use-after-free"]
aliases = ["GHSA-5wjr-9356-4wc8"]

[versions]
patched = [">= 1.54.1"]
unaffected = ["< 1.54.0"]
```

# Race condition in `tokio::time::Interval` on Tokio v1.54.0

## Summary

On the multi-threaded runtime, there is a race condition between `Interval::poll_tick` and the runtime's time driver.

This affects `Interval` only. `Sleep` and `Timeout` are **not** affected.

## Details

In Tokio v1.54.0, a race condition between `tokio::time::Interval::poll_tick` (including `.tick().await`) and the runtime time driver can drop an internal timer entry without synchronizing with the driver thread that is firing it. This causes a data race and use-after-free on the timer's waker and task state in safe Rust, leading to assertion panics, task reference-count corruption, or heap memory corruption.

This bug was introduced by https://github.com/tokio-rs/tokio/pull/8458, and it was fixed by https://github.com/tokio-rs/tokio/pull/8692.

## Impact

This may lead to crashes and use-after-frees in applications using `Interval` on a multi-threaded runtime.

Since this issue is known to lead to crashes in production systems, the v1.54.0 release of Tokio has been yanked.

## Credits

Thank you to [creepycode](https://github.com/ByteZ1337) and [carabistouille](https://github.com/carabistouflette) for reporting this privately using the procedure outlined in our security policy. They both reported the issue independently of each other.
