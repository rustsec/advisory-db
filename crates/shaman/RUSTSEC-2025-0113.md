```toml
[advisory]
id = "RUSTSEC-2025-0113"
package = "shaman"
date = "2025-05-06"
informational = "unsound"
url = "https://crates.io/crates/shaman"
categories = ["memory-corruption"]
aliases = ["GHSA-7vjm-6qgq-3mrq"]

[affected.functions]
"shaman::cryptoutil::write_u64v_le" = ["<= 0.1.0"]
"shaman::cryptoutil::write_u32v_le" = ["<= 0.1.0"]
"shaman::cryptoutil::read_u64v_be" = ["<= 0.1.0"]
"shaman::cryptoutil::read_u64v_le" = ["<= 0.1.0"]
"shaman::cryptoutil::read_u32v_be" = ["<= 0.1.0"]
"shaman::cryptoutil::read_u32v_le" = ["<= 0.1.0"]

[versions]
patched = []
unaffected = []
```

# soundness issue and unmaintained
`shaman::cryptoutil::write_u64v_le` and other functions mentioned above cannot garantee memory safety of get_unchecked later if both length are zero.

`shaman` is unmaintained.
