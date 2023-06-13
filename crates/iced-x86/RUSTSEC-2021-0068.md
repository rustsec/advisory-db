```toml
[advisory]
id = "RUSTSEC-2021-0068"
package = "iced-x86"
date = "2021-05-19"
url = "https://github.com/icedland/iced/issues/168"
keywords = ["soundness"]
aliases = ["CVE-2021-38188", "GHSA-jjx5-3f36-6927"]

[affected]
functions = { "iced_x86::Decoder::new" = ["<= 1.10.3"] }

[versions]
patched = ["> 1.10.3"]
```

# Soundness issue in `iced-x86` versions <= 1.10.3 

Versions of iced-x86 <= 1.10.3 invoke undefined behavior which may cause soundness
issues in crates using the `iced_x86::Decoder` struct. The `Decoder::new()` function
made a call to `slice.get_unchecked(slice.length())` to get the end position of 
the input buffer. The flaw was fixed with safe logic that does not invoke undefined
behavior.

More details can be found at <https://github.com/icedland/iced/issues/168>.
