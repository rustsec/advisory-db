```toml
[advisory]
id = "RUSTSEC-2020-0061"
package = "futures-task"
aliases = ["CVE-2020-35907", "GHSA-p9m5-3hj7-cp5r"]
cvss = "CVSS:3.1/AV:L/AC:L/PR:L/UI:N/S:U/C:N/I:N/A:H"
date = "2020-05-03"
url = "https://github.com/rust-lang/futures-rs/issues/2091"
categories = ["denial-of-service"]
keywords = ["NULL pointer dereference", "memory-management"]
[versions]
patched = [">= 0.3.5"]
[affected]
functions = { "futures_task::noop_waker_ref" = [">= 0.3.0"] }
```

# futures_task::noop_waker_ref can segfault due to dereferencing a NULL pointer

Affected versions of the crate used a `UnsafeCell` in thread-local storage to return a noop waker reference,
assuming that the reference would never be returned from another thread.

This resulted in a segmentation fault crash if `Waker::wake_by_ref()` was called on a waker returned from another thread due to 
it attempting to dereference a pointer that wasn't accessible from the main thread.

Reproduction Example (from issue):
```rust
use futures_task::noop_waker_ref;
fn main() {
    let waker = std::thread::spawn(|| noop_waker_ref()).join().unwrap();
    waker.wake_by_ref();
}
```

The flaw was corrected by using a `OnceCell::Lazy<>` wrapper around the noop waker instead of thread-local storage.
