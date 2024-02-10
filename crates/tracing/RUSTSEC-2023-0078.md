```toml
[advisory]
id = "RUSTSEC-2023-0078"
package = "tracing"
date = "2023-10-19"
url = "https://github.com/tokio-rs/tracing/pull/2765"
informational = "unsound"
categories = ["memory-corruption"]
keywords = ["use-after-free"]
aliases = ["GHSA-8f24-6m29-wm2r"]

[versions]
patched = [">= 0.1.40"]
unaffected = ["<= 0.1.37"]

[affected]
functions = { "tracing::instrument::Instrumented::into_inner" = [">= 0.1.38", "< 0.1.40"] }
```

# Potential stack use-after-free in `Instrumented::into_inner`

The implementation of the [`Instrumented::into_inner`] method in affected
versions of this crate contains undefined behavior due to incorrect use of
[`std::mem::forget`] The function creates `*const` pointers to `self`, calls
[`mem::forget(self)`][`std::mem::forget`], and then moves values out of those
pointers using [`std::ptr::read`].

```rust
// To manually destructure `Instrumented` without `Drop`, we
// move it into a ManuallyDrop and use pointers to its fields
let span: *const Span = &this.span;
let inner: *const ManuallyDrop<T> = &this.inner;
mem::forget(self);
// SAFETY: Those pointers are valid for reads, because `Drop` didn't
//         run, and properly aligned, because `Instrumented` isn't
//         `#[repr(packed)]`.
let _span = unsafe { span.read() };
let inner = unsafe { inner.read() };
```

However, the [`mem::forget` documentation][`std::mem::forget`] states:

> Any resources the value manages, such as heap memory or a file handle, will
> linger forever in an unreachable state. **However, it does not guarantee that
> pointers to this memory will remain valid.**

This means that these pointers are no longer valid. This could result in a stack
use-after-free if LLVM chooses to reuse `self`'s stack slot for a rebinding
after the call to [`std::mem::forget`].

This undefined behavior has not been observed to cause miscompilation as of Rust
1.73.0. However, any use of this method with the affected versions of `tracing`
are unsound.

The flaw was corrected in commit [20a1762] ([PR #2765]) by replacing the use of
[`std::mem::forget`] with `std::mem::ManuallyDrop`, ensuring that the stack slot
is not reused and the pointers remain valid when they are read. The fix is
published in `tracing` [v0.1.40]. Affected versions have been yanked from
crates.io.

Thanks to [Taylor Cramer] and [Manish Goregaokar] for finding and correcting
this issue!

[`Instrumented::into_inner`]:
    https://docs.rs/tracing/latest/tracing/instrument/struct.Instrumented.html#method.into_inner
[`std::mem::forget`]: https://doc.rust-lang.org/std/mem/fn.forget.html
[`std::ptr::read`]:
    https://doc.rust-lang.org/std/primitive.pointer.html#method.read-1
[20a1762]:
    https://github.com/tokio-rs/tracing/commit/20a1762b3fd5f1fafead198fd18e469c68683721
[PR #2765]: https://github.com/tokio-rs/tracing/pull/2765
[v0.1.40]: https://crates.io/crates/tracing/0.1.40
[Taylor Cramer]: https://github.com/cramertj
[Manish Goregaokar]: https://github.com/manishearth
