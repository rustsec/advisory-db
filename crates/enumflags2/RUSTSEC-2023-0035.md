```toml
[advisory]
id = "RUSTSEC-2023-0035"
package = "enumflags2"
date = "2023-04-17"
url = "https://github.com/meithecatte/enumflags2/releases/tag/v0.7.7"
informational = "unsound"
aliases = ["GHSA-qvc4-78gw-pv8p"]

# [affected.macros]
# "enumflags2::make_bitflags" = ["< 0.7.7, >= 0.7.0"]

[versions]
patched = [">= 0.7.7"]

unaffected = ["< 0.7.0"]
```

# Adverserial use of `make_bitflags!` macro can cause undefined behavior

The macro relied on an expression of the form `Enum::Variant` always being a
variant of the enum. However, it may also be an associated integer constant, in
which case there's no guarantee that the value of said constant consists only of
bits valid for this bitflag type.

Thus, code like this could create an invalid `BitFlags<Test>`, which would cause
iterating over it to trigger undefined behavior. As the debug formatter
internally iterates over the value, it is also affected.

```rust
use enumflags2::{bitflags, make_bitflags};

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum Test {
    A = 1,
    B = 2,
}

impl Test {
    const C: u8 = 69;
}

fn main() {
    let x = make_bitflags!(Test::{C});
    // printing or iterating over x is UB
}
```
