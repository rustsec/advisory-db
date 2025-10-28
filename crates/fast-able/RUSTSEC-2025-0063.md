```toml
[advisory]
id = "RUSTSEC-2025-0063"
package = "fast-able"
date = "2025-04-25"
categories = ["memory-corruption"]
aliases = ["GHSA-95hm-pr6q-298w"]

[affected.functions]
"fast_able::vec::SyncVec::get_unchecked" = ["< 1.13.7"] 

[versions]
patched = [">= 1.13.7"]
unaffected = []
```

# Possible unsound public API

The public accessible struct SyncVec has a public safe method get_unchecked. It accept a parameter index and used in the get_unchecked without sufficient checks as mentioned [here](https://doc.rust-lang.org/std/primitive.slice.html#method.get_unchecked). 

