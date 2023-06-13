```toml
[advisory]
id = "RUSTSEC-2021-0027"
package = "bam"
aliases = ["CVE-2021-28027", "GHSA-cpqj-r29q-chrh"]
cvss = "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H"
date = "2021-01-07"
url = "https://gitlab.com/tprodanov/bam/-/issues/4"
categories = ["memory-corruption"]

[versions]
patched = [">= 0.1.3"]

[affected]
functions = { "bam::bgzip::Block::load" = ["< 0.1.3"] }
```

# Loading a bgzip block can write out of bounds if size overflows.

Affected versions of `bam` set the length of an internal buffer using
`self.compressed.set_len(block_size - HEADER_SIZE - MIN_EXTRA_SIZE)` and then
wrote into it. While `block_size` was constrained to a proper maximum, when it
was too small the subtraction could overflow negatively to a large number past
the capacity of `self.compressed`.

This can result in memory corruption in the form of writing out of bounds when
loading a `bgzip` file with a small `block_size`.

Commit `061eee38d4` fixed this issue by checking for the underflow when setting
the buffer size.
