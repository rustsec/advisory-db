```toml
[advisory]
id = "RUSTSEC-2026-0386"
package = "tokio"
date = "2026-10-13"
url = "https://github.com/tokio-rs/tokio/pull/8587"
references = [
  "https://github.com/tokio-rs/tokio/issues/8538",
  "https://github.com/tokio-rs/tokio/issues/8539"
]
informational = "unsound"
categories = ["memory-corruption"]
keywords = ["io-safety", "use-after-free"]

[versions]
patched = [">= 1.51.6, < 1.52.0", ">= 1.53.3"]
unaffected = ["< 0.3.2"]

[affected.functions]
"tokio::io::unix::AsyncFd::new" = [">= 0.3.2"]
"tokio::io::unix::AsyncFd::with_interest" = [">= 0.3.2"]
"tokio::io::unix::AsyncFd::try_new" = [">= 1.37.0"]
"tokio::io::unix::AsyncFd::try_with_interest" = [">= 1.37.0"]
```

# `AsyncFd::new` and related constructors violate I/O safety

`tokio::io::unix::AsyncFd` provides safe constructors (`new`, `with_interest`, `try_new`, and `try_with_interest`) that accept any `T: AsRawFd` rather than requiring proof of ownership or validity of the file descriptor. Because `RawFd` implements `AsRawFd`, safe code can construct an `AsyncFd` from an arbitrary or closed file descriptor, or close/replace the underlying file descriptor while the `AsyncFd` is still alive.

This violates I/O safety in two ways:

1. `AsyncFd<T>` implements `AsFd` for all `T: AsRawFd` by calling `BorrowedFd::borrow_raw(self.as_raw_fd())`, allowing safe code to obtain an arbitrary `BorrowedFd<'_>` without owning the file descriptor.
2. Tokio's I/O driver stores a pointer to an internal allocation in the `epoll`/`kqueue` user data for registered file descriptors. If the underlying file descriptor is duplicated and then closed while an `AsyncFd` is alive, and the OS reuses that file descriptor number for a another IO resource on the same runtime, dropping the first `AsyncFd` deregisters the reused file descriptor number and frees the first while the original file description remains registered in `epoll`, leading to a use-after-free when a readiness event fires.

Starting in Tokio 1.51.6, 1.53.3, and 1.54.0, `AsyncFd::new`, `AsyncFd::with_interest`, `AsyncFd::try_new`, and `AsyncFd::try_with_interest` are deprecated in favor of the `unsafe` constructors `AsyncFd::register` and `AsyncFd::register_with_interest`. Callers must ensure that the file descriptor remains valid, open, and unchanged for the lifetime of the `AsyncFd`.
