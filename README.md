# RustSec Advisory Database

[![Build Status][build-image]][build-link]
![Maintained: Q4 2019][maintained-image]
[![Gitter Chat][gitter-image]][gitter-link]

The RustSec Advisory Database is a repository of security advisories filed
against Rust crates published via https://crates.io

Advisory metadata is stored in [TOML] format (see below). The following tools
consume the data and can be used for auditing and reporting (send PRs to add yours):

* [cargo-audit]: Audit `Cargo.lock` files for crates with security vulnerabilities

## Reporting Vulnerabilities

To report a new vulnerability, open a pull request using the template below.
See [CONTRIBUTING.md] for more information.

<a href="https://github.com/RustSec/advisory-db/blob/master/CONTRIBUTING.md">
  <img alt="Report Vulnerability" width="250px" height="60px" src="https://rustsec.org/assets/img/report-vuln-button.svg">
</a>

## Advisory Format

Each advisory contains information in [TOML] format:

```toml
# Before you submit a PR using this template, **please delete the comments**
# explaining each field, as well as any unused fields.

[advisory]
# Identifier for the advisory (mandatory). Will be assigned a "RUSTSEC-YYYY-NNNN"
# identifier e.g. RUSTSEC-2018-0001. Please use "RUSTSEC-0000-0000" in PRs.
id = "RUSTSEC-0000-0000"

# Name of the affected crate (mandatory)
package = "mycrate"

# Disclosure date of the advisory as an RFC 3339 date (mandatory)
date = "2019-10-01"

# Single-line description of a vulnerability (mandatory)
title = "Flaw in X allows Y"

# Enter a short-form description of the vulnerability here (mandatory)
description = """
Affected versions of this crate did not properly X.

This allows an attacker to Y.
 
The flaw was corrected by Z.
"""

# Versions which include fixes for this vulnerability (mandatory)
patched_versions = [">= 1.2.0"]

# Versions which were never vulnerable (optional)
#unaffected_versions = ["< 1.1.0"]

# URL to a long-form description of this issue, e.g. a GitHub issue/PR,
# a change log entry, or a blogpost announcing the release (optional)
url = "https://github.com/mystuff/mycrate/issues/123"

# Optional: Categories this advisory falls under. Valid categories are:
# "code-execution", "crypto-failure", "denial-of-service", "file-disclosure"
# "format-injection", "memory-corruption", "memory-exposure", "privilege-escalation"
categories = ["crypto-failure"]

# Freeform keywords which describe this vulnerability, similar to Cargo (optional)
keywords = ["ssl", "mitm"]

# Vulnerability aliases, e.g. CVE IDs (optional but recommended)
# Request a CVE for your RustSec vulns: https://iwantacve.org/
#aliases = ["CVE-2018-XXXX"]

# References to related vulnerabilities (optional)
# e.g. CVE for a C library wrapped by a -sys crate)
#references = ["CVE-2018-YYYY", "CVE-2018-ZZZZ"]

# Optional: metadata which narrows the scope of what this advisory affects
[affected]
# CPU architectures impacted by this vulnerability (optional).
# Only use this if the vulnerability is specific to a particular CPU architecture,
# e.g. the vulnerability is in x86 assembly.
# For a list of CPU architecture strings, see the "platforms" crate:
# <https://docs.rs/platforms/latest/platforms/target/enum.Arch.html>
#arch = ["x86", "x86_64"]

# Operating systems impacted by this vulnerability (optional)
# Only use this if the vulnerable is specific to a particular OS, e.g. it was
# located in a binding to a Windows-specific API.
# For a list of OS strings, see the "platforms" crate:
# <https://docs.rs/platforms/latest/platforms/target/enum.OS.html>
#os = ["windows"]

# Table of canonical paths to vulnerable functions (optional)
# mapping to which versions impacted by this advisory used that particular
# name (e.g. if the function was renamed between versions). 
# The path syntax is `cratename::path::to::function`, without any
# parameters or additional information, followed by a list of version reqs.
functions = { "mycrate::MyType::vulnerable_function" = ["< 1.2.0, >= 1.1.0"] }
```

## License

All content in this repository is placed in the public domain.

[![Public Domain](http://i.creativecommons.org/p/zero/1.0/88x31.png)](https://github.com/RustSec/advisory-db/blob/master/LICENSE.txt)

[//]: # (badges)

[build-image]: https://github.com/rustsec/advisory-db/workflows/Validate/badge.svg
[build-link]: https://github.com/rustsec/advisory-db/actions
[maintained-image]: https://img.shields.io/maintenance/yes/2019.svg
[gitter-image]: https://badges.gitter.im/badge.svg
[gitter-link]: https://gitter.im/RustSec/Lobby

[//]: # (general links)

[TOML]: https://github.com/toml-lang/toml
[cargo-audit]: https://github.com/rustsec/cargo-audit
[CONTRIBUTING.md]: https://github.com/RustSec/advisory-db/blob/master/CONTRIBUTING.md
