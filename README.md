# RustSec Advisory Database [![Build Status][build-image]][build-link] ![Maintained as of July 2018][maintained-image]

[build-image]: https://travis-ci.org/RustSec/advisory-db.svg?branch=master
[build-link]: https://travis-ci.org/RustSec/advisory-db
[maintained-image]: https://img.shields.io/maintenance/yes/2018.svg

The RustSec Advisory Database is a repository of security advisories filed
against Rust crates published via https://crates.io

Advisory metadata is stored in [TOML] format for [cargo-audit] and other
automated tools to consume.

## Format

Each advisory contains information in [TOML] format:

```toml
[advisory]
package = "mypackage"

# Versions which were never vulnerable
unaffected_versions = ["< 1.1.0"]

# Versions which include fixes for this vulnerability
patched_versions = [">= 1.2.0"]

# Vulnerability aliases (e.g. CVE IDs). Optional but recommended.
# Request a CVE for your RustSec vulns: https://iwantacve.org/
aliases = ["CVE-2018-XXXX"]

# References to related vulnerabilities (Optional)
# e.g. CVE for a C library wrapped by a -sys crate)
references = ["CVE-2018-YYYY", "CVE-2018-ZZZZ"]

# URL to a long-form description of this issue, e.g. a blogpost announcing
# the release or a changelog entry (optional)
url = false

# Single-line description of a vulnerability
title = "Flaw in X allows Y"

# Disclosure date of the advisory (RFC 3339)
date = "2017-02-25"

# Enter a short-form description of the vulnerability here (required)
description = """
Affected versions of this crate did not properly X.

This allows an attacker to Y.
 
The flaw was corrected by Z.
"""
```

[TOML]: https://github.com/toml-lang/toml
[cargo-audit]: https://github.com/rustsec/cargo-audit

## License

All content in this repository is placed in the public domain.

[![Public Domain](http://i.creativecommons.org/p/zero/1.0/88x31.png)](https://github.com/RustSec/advisory-db/blob/master/LICENSE.txt)
