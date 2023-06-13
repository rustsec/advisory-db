```toml
[advisory]
id = "RUSTSEC-2021-0072"
package = "tokio"
date = "2021-07-07"
url = "https://github.com/tokio-rs/tokio/issues/3929"
categories = ["memory-corruption"]
keywords = ["race condition", "send"]
aliases = ["CVE-2021-38191", "GHSA-2grh-hm3w-w7hv"]

[affected]
functions = { "tokio::task::JoinHandle::abort" = ["<= 1.8.0, >= 0.3.0"] }

[versions]
patched = [">= 1.5.1, < 1.6.0", ">= 1.6.3, < 1.7.0", ">= 1.7.2, < 1.8.0", ">= 1.8.1"]
unaffected = ["< 0.3.0"]
```

# Task dropped in wrong thread when aborting `LocalSet` task

When aborting a task with `JoinHandle::abort`, the future is dropped in the
thread calling abort if the task is not currently being executed. This is
incorrect for tasks spawned on a `LocalSet`.

This can easily result in race conditions as many projects use `Rc` or `RefCell`
in their Tokio tasks for better performance.

See [tokio#3929][issue] for more details.

[issue]: https://github.com/tokio-rs/tokio/issues/3929
