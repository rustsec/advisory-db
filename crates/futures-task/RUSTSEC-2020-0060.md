```toml
[advisory]
id = "RUSTSEC-2020-0060"
package = "futures-task"
aliases = ["CVE-2020-35906", "GHSA-r93v-9p5q-vhpf"]
cvss = "CVSS:3.1/AV:L/AC:L/PR:L/UI:N/S:U/C:H/I:H/A:H"
date = "2020-09-04"
url = "https://github.com/rust-lang/futures-rs/pull/2206"
categories = ["code-execution", "memory-corruption"]
keywords = ["use-after-free", "arbitrary code execution", "memory-corruption", "memory-management"]

[versions]
patched = [">= 0.3.6"]
unaffected = ["<= 0.2.1"]

[affected]
functions = { "futures_task::waker" = [">= 0.3.0"] }
```

# futures_task::waker may cause a use-after-free if used on a type that isn't 'static

Affected versions of the crate did not properly implement a `'static` lifetime bound on the `waker` function.
This resulted in a use-after-free if `Waker::wake()` is called after original data had been dropped.

The flaw was corrected by adding `'static` lifetime bound to the data `waker` takes.
