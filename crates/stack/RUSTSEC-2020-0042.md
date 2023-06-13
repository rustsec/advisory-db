```toml
[advisory]
id = "RUSTSEC-2020-0042"
package = "stack"
aliases = ["CVE-2020-35895", "GHSA-h45v-vgvp-3h5v"]
cvss = "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H"
date = "2020-09-24"
url = "https://github.com/arcnmx/stack-rs/issues/4"

[versions]
patched = [">= 0.3.1"]
```

# Missing check in ArrayVec leads to out-of-bounds write.

ArrayVec::insert allows insertion of an element into the array object into the
specified index. Due to a missing check on the upperbound of this index, it is
possible to write out of bounds.
