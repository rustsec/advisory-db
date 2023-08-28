# Reporting Vulnerabilities

To add an advisory to the RustSec database, open a [Pull Request] against
[this](https://github.com/RustSec/advisory-db) repository containing the new advisory:

### Required Steps

1. Create a file named `RUSTSEC-0000-0000.md` in the `crates/<yourcratename>`
   subdirectory of the repository (you may need to create it if it doesn't exist)
2. Copy and paste the [TOML advisory template] from the README.md file in this repo.
   Delete the comments and additional whitespace, and fill it out with the
   details of the advisory. Surround the TOML data with <code>\```toml</code> and <code>\```</code> markers.
3. Write a human-readable Markdown description in the same file, after the <code>\```</code> marker and a newline. Use [this example advisory][example] as a reference.
4. Open a [Pull Request]. After being reviewed your advisory will be assigned
   a `RUSTSEC-*` advisory identifier and be published to the database.
   
### Optional Steps

Feel free to do either or both of these as you see fit (we recommend you do both):

4. [Yank] the affected versions of the crate.
5. Request a CVE for your vulnerability. See for details:
   https://cve.mitre.org/cve/request_id.html and https://cveform.mitre.org .
   Alternatively, you can create a GitHub Security Advisory (GHSA) and let them request
   a CVE for you. In this case, you can add the GHSA ID to the RustSec advisory via the
   `aliases` field.

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

Moreover, RustSec also tracks [soundness] issues as informational advisories, independent of whether they are vulnerabilities or not.
A soundness issue arises when using a crate from safe code can cause [Undefined Behavior].

[soundness]: https://rust-lang.github.io/unsafe-code-guidelines/glossary.html#soundness-of-code--of-a-library
[Undefined Behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html

When in doubt, please open a PR.

## FAQ

**Q: Do I need to be owner of a crate to file an advisory?**

A: No, anyone can file an advisory against any crate. The legitimacy of
    vulnerabilities will be determined prior to merging. If a vulnerability
    turns out to be fake, it will be removed from the database.
    
**Q: Can I file an advisory without creating a pull request?**

A: Yes, instead of creating a full advisory yourself, you can also
   [open an issue on the advisory-db repo](https://github.com/RustSec/advisory-db/issues)
   or email information about the vulnerability to
   [rustsec@googlegroups.com](mailto:rustsec@googlegroups.com).

**Q: Does this project have a GPG key or other means of handling embargoed vulnerabilities?**

A: We do not presently handle embargoed vulnerabilities. Please ensure embargoes
   have been lifted and details have been disclosed to the public prior to filing
   them against RustSec.

[Pull Request]: https://github.com/RustSec/advisory-db/pulls
[TOML advisory template]: https://github.com/RustSec/advisory-db#advisory-format
[Yank]: https://doc.rust-lang.org/cargo/commands/cargo-yank.html
[example]: https://raw.githubusercontent.com/rustsec/advisory-db/main/EXAMPLE_ADVISORY.md
