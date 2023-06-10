```toml
[advisory]
id = "RUSTSEC-2020-0062"
package = "futures-util"
aliases = ["CVE-2020-35908", "GHSA-5r9g-j7jj-hw6c"]
cvss = "CVSS:3.1/AV:L/AC:L/PR:L/UI:N/S:U/C:N/I:N/A:H"
date = "2020-01-24"
url = "https://github.com/rust-lang/futures-rs/issues/2050"
categories = ["memory-corruption", "thread-safety"]
keywords = ["concurrency", "memory-corruption", "memory-management"]

[affected]
functions = { "futures_util::stream::FuturesUnordered" = [">= 0.3.0"] }

[versions]
patched = [">= 0.3.2"]
unaffected = ["< 0.3.0"]
```

# Improper `Sync` implementation on `FuturesUnordered` in futures-utils can cause data corruption
Affected versions of the crate had an unsound `Sync` implementation on the `FuturesUnordered` structure, which used a `Cell` for
interior mutability without any code to handle synchronized access to the underlying task list's length and head safely.

This could of lead to data corruption since two threads modifying the list at once could see incorrect values due to the lack
of access synchronization.

The issue was fixed by adding access synchronization code around insertion of tasks into the list.
