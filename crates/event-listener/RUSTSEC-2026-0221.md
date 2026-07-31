```toml
[advisory]
id = "RUSTSEC-2026-0221"
package = "event-listener"
date = "2026-07-13"
url = "https://github.com/smol-rs/event-listener/pull/163"
informational = "unsound"
categories = ["memory-corruption", "thread-safety"]
keywords = ["concurrency"]

[versions]
patched = [">= 5.4.2"]
unaffected = ["< 5.1.0"]
```

# `event-listener` allows `!Send` tags to cross thread boundaries via `StackSlot`

Affected versions of `event-listener` unconditionally implement `Send` and
`Sync` for `StackSlot<'_, T>`, the stack-allocated listener type created
by the `listener!` macro.

This allows a `!Send` tag type set via `Event::with_tag` to be moved to
another thread and accessed via `StackSlot::wait`, causing a data race in safe
code.
