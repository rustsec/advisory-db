```toml
[advisory]
id = "RUSTSEC-2020-0059"
package = "futures-util"
aliases = ["CVE-2020-35905", "GHSA-rh4w-94hh-9943"]
cvss = "CVSS:3.1/AV:L/AC:H/PR:L/UI:N/S:U/C:N/I:N/A:H"
date = "2020-10-22"
url = "https://github.com/rust-lang/futures-rs/issues/2239"
categories = ["thread-safety"]
keywords = ["concurrency", "memory-corruption", "memory-management"]

[affected]
functions = { "futures_util::lock::MutexGuard::map" = [">= 0.3.2"] }

[versions]
patched = [">= 0.3.7"]
unaffected = ["< 0.3.2"]
```

# MutexGuard::map can cause a data race in safe code
Affected versions of the crate had a Send/Sync implementation for MappedMutexGuard that only considered variance on T, while MappedMutexGuard dereferenced to U.

This could of led to data races in safe Rust code when a closure used in MutexGuard::map() returns U that is unrelated to T.

The issue was fixed by fixing `Send` and `Sync` implementations, and by adding a `PhantomData<&'a mut U>` marker to the `MappedMutexGuard` type to tell the compiler that the guard is over
U too.
