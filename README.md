# RustSec Advisory Database

[![Build Status][build-image]][build-link]
![Maintained as of July 2018][maintained-image]
[![Gitter Chat][gitter-image]][gitter-link]

[build-image]: https://travis-ci.org/RustSec/advisory-db.svg?branch=master
[build-link]: https://travis-ci.org/RustSec/advisory-db
[maintained-image]: https://img.shields.io/maintenance/yes/2018.svg
[gitter-image]: https://badges.gitter.im/badge.svg
[gitter-link]: https://gitter.im/RustSec/Lobby

The RustSec Advisory Database is a repository of security advisories filed
against Rust crates published via https://crates.io

Advisory metadata is stored in [TOML] format (see below). The following tools
consume the data and can be used for auditing and reporing (send PRs to add yours):

* [cargo-audit]: Audit `Cargo.lock` files for crates with security vulnerabilities

[TOML]: https://github.com/toml-lang/toml
[cargo-audit]: https://github.com/rustsec/cargo-audit

## Reporting Vulnerabilities

To report a new vulnerability, open a pull request using the template below.
See [CONTRIBUTING.md] for more information.

<a href="https://github.com/RustSec/advisory-db/blob/master/CONTRIBUTING.md">
  <img alt="Report Vulnerability" width="250px" height="60px" src="https://rustsec.org/assets/img/report-vuln-button.svg">
</a>

[CONTRIBUTING.md]: https://github.com/RustSec/advisory-db/blob/master/CONTRIBUTING.md

## Advisory Format

Each advisory contains information in [TOML] format:

```toml
[advisory]
# Identifier for the advisory (mandatory). Will be assigned a "RUSTSEC-YYYY-NNNN"
# identifier e.g. RUSTSEC-2018-0001. Please use "RUSTSEC-0000-0000" in PRs.
id = "RUSTSEC-0000-0000"

# Name of the affected crate (mandatory)
package = "mycrate"

# Disclosure date of the advisory as an RFC 3339 date (mandatory)
date = "2017-02-25"

# Versions which include fixes for this vulnerability (mandatory)
patched_versions = [">= 1.2.0"]

# Versions which were never vulnerable (optional)
unaffected_versions = ["< 1.1.0"]

# Vulnerability aliases, e.g. CVE IDs (optional but recommended)
# Request a CVE for your RustSec vulns: https://iwantacve.org/
aliases = ["CVE-2018-XXXX"]

# References to related vulnerabilities (optional)
# e.g. CVE for a C library wrapped by a -sys crate)
references = ["CVE-2018-YYYY", "CVE-2018-ZZZZ"]

# URL to a long-form description of this issue, e.g. a GitHub issue/PR,
# a change log entry, or a blogpost announcing the release (optional)
url = "https://github.com/mystuff/mycrate/issues/123"

# Single-line description of a vulnerability (mandatory)
title = "Flaw in X allows Y"

# Enter a short-form description of the vulnerability here (mandatory)
description = """
Affected versions of this crate did not properly X.

This allows an attacker to Y.
 
The flaw was corrected by Z.
"""
```

## License

All content in this repository is placed in the public domain.

[![Public Domain](http://i.creativecommons.org/p/zero/1.0/88x31.png)](https://github.com/RustSec/advisory-db/blob/master/LICENSE.txt)
