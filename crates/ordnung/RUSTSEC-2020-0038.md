```toml
[advisory]
id = "RUSTSEC-2020-0038"
package = "ordnung"
aliases = ["CVE-2020-35890", "CVE-2020-35891", "GHSA-4wj3-p7hj-cvx8", "GHSA-qrwc-jxf5-g8x6"]
cvss = "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:N/I:N/A:H"
date = "2020-09-03"
url = "https://github.com/maciejhirsz/ordnung/issues/8"

[versions]
patched = []
```

# Memory safety issues in `compact::Vec`

`compact::Vec` contains multiple memory safety issues.

1. It mishandles large capacity and causes out-of-bound access in 32-bit / allocator layout mismatch in 64-bit.
2. `remove()` is not panic-safe and causes double-free when an index larger than the length is provided.
