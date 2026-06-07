```toml
[advisory]
id = "RUSTSEC-2026-0173"
package = "proc-macro-error2"
date = "2026-06-07"
informational = "unmaintained"
url = "https://github.com/GnomedDev/proc-macro-error-2/issues/17"

[versions]
patched = []
```

# proc-macro-error2 is unmaintained

The author of `proc-macro-error2` has [confirmed](https://github.com/GnomedDev/proc-macro-error-2/issues/17#issuecomment-4643215473) that the crate is no longer maintained and recommends that users migrate away from it.

`proc-macro-error2` was originally created as a maintained fork of [`proc-macro-error`](https://crates.io/crates/proc-macro-error) (see [RUSTSEC-2024-0370](https://rustsec.org/advisories/RUSTSEC-2024-0370)). Both the original crate and this fork are now unmaintained.

## Possible Alternative(s)

- [manyhow](https://crates.io/crates/manyhow)
- [proc-macro2-diagnostics](https://github.com/SergioBenitez/proc-macro2-diagnostics)
