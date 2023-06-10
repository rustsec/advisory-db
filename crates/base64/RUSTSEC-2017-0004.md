```toml
[advisory]
id = "RUSTSEC-2017-0004"
package = "base64"
aliases = ["CVE-2017-1000430", "GHSA-x67x-vg9m-65c3"]
cvss = "CVSS:3.0/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H"
date = "2017-05-03"
keywords = ["memory-corruption"]
url = "https://github.com/alicemaz/rust-base64/commit/24ead980daf11ba563e4fb2516187a56a71ad319"

[versions]
patched = [">= 0.5.2"]
```

# Integer overflow leads to heap-based buffer overflow in encode_config_buf

Affected versions of this crate suffered from an integer overflow bug when
calculating the size of a buffer to use when encoding base64 using the
`encode_config_buf` and `encode_config` functions.  If the input string
was large, this would cause a buffer to be allocated that was too small.
Since this function writes to the buffer using unsafe code, it would
allow an attacker to write beyond the buffer, causing memory corruption
and possibly the execution of arbitrary code.

This flaw was corrected by using checked arithmetic to calculate
the size of the buffer.
