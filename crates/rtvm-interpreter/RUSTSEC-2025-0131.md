```toml
[advisory]
id = "RUSTSEC-2025-0131"
package = "rtvm-interpreter"
date = "2025-10-31"
references = [
    "https://crates.io/crates/rtvm-interpreter/4.0.0",
    "https://docs.rs/rtvm-interpreter/latest/src/rtvm_interpreter/interpreter.rs.html",
]
informational = "unsound"
categories = ["memory-corruption"]
aliases = ["GHSA-pq5v-rwp8-p7gm"]

[affected.functions]
"rtvm_interpreter::Interpreter::program_counter" = ["<= 4.0.0"]

[versions]
patched = []
unaffected = []
```

# Lack of sufficient checks in public API

The affected function is unsound due to insufficient checks on public struct field.
