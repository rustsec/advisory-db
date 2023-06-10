```toml
[advisory]
id = "RUSTSEC-2017-0003"
package = "security-framework"
aliases = ["CVE-2017-18588", "GHSA-jqqr-c2r2-9cvr"]
cvss = "CVSS:3.0/AV:N/AC:L/PR:N/UI:N/S:U/C:N/I:L/A:N"
date = "2017-03-15"
keywords = ["mitm"]
url = "https://github.com/sfackler/rust-security-framework/pull/27"

[versions]
patched = [">= 0.1.12"]
```

# Hostname verification skipped when custom root certs used

If custom root certificates were registered with a `ClientBuilder`, the
hostname of the target server would not be validated against its presented leaf
certificate.

This issue was fixed by properly configuring the trust evaluation logic to
perform that check.
