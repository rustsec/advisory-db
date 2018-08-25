# Reporting Vulnerabilities

To add an advisory to the RustSec database, open a [Pull Request] against
this repository containing the new advisory:

1. Create a file named `RUSTSEC-0000-0000.toml` in the `crates/<yourcratename>`
   subdirectory of this repository (you may need to create it if it doesn't exist)
2. Copy and paste the [TOML advisory template] from the README.md file in this repo.
   Delete the comments and additional whitespace, and fill it out with the
   details of the advisory.
3. Open a [Pull Request]. After being reviewed your advisory will be assigned
   a `RUSTSEC-*` advisory identifier and be published to the database.
4. (Optional, but recommended) Request a CVE for your vulnerability:
   https://iwantacve.org/

[Pull Request]: https://github.com/RustSec/advisory-db/pulls
[TOML advisory template]: https://github.com/RustSec/advisory-db#advisory-format

## Criteria

RustSec is a database of security vulnerabilities. The following are
examples of qualifying vulnerabilities:

* Code Execution (i.e. RCE)
* Memory Corruption
* Privilege Escalation (either at OS level or inside of an app/library)
* File Disclosure / Directory Traversal
* Web Security (e.g. XSS, CSRF)
* Format Injection, e.g. shell escaping, SQL injection (and also XSS)
* Cryptography Failure (e.g. confidentiality breakage, integrity breakage, key leakage)
* Covert Channels (e.g. Spectre, Meltdown)
* Panics in code advertised as "panic-free" (particularly if useful for network DoS attacks)

When in doubt, please open a PR.

## FAQ

**Q: Do I need to be owner of a crate to file an advisory?**

A:  No, anyone can file an advisory against any crate. The legitimacy of
    vulnerabilities will be determined prior to merging. If a vulnerability
    turns out to be fake it will be removed from the database.
    
**Q: Can I file an advisory without creating a pull request?**

A: Yes, instead of creating a full advisory yourself you can also
   [open an issue on the advisory-db repo](https://github.com/RustSec/advisory-db/issues)
   or email information about the vulnerability to
   [rustsec@googlegroups.com](mailto:rustsec@googlegroups.com).

**Q: Does this project have a GPG key or other means of handling embargoed vulnerabilities?**

A: We do not presently handle embargoed vulnerabilities. Please ensure embargoes
   have been lifted and details have been disclosed to the public prior to filing
   them against RustSec.
