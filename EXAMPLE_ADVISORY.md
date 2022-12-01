```toml
# Before you submit a PR using this template, **please delete the comments**
# explaining each field, as well as any unused fields.
# All optional fields are commented out.

[advisory]
# Identifier for the advisory (mandatory). Will be assigned a "RUSTSEC-YYYY-NNNN"
# identifier e.g. RUSTSEC-2018-0001. Please use "RUSTSEC-0000-0000" in PRs.
id = "RUSTSEC-0000-0000"

# Name of the affected crate (mandatory)
package = "mycrate"

# Disclosure date of the advisory as an RFC 3339 date (mandatory)
date = "2021-01-31"

# URL to a long-form description of this issue, e.g. a GitHub issue/PR,
# a change log entry, or a blogpost announcing the release (optional)
url = "https://github.com/mystuff/mycrate/issues/123"

# URL to additional helpful references regarding the advisory (optional)
#references = ["https://github.com/mystuff/mycrate/discussions/1"]

# Optional: Indicates the type of informational security  advisory
#  - "unsound" for soundness issues
#  - "unmaintained" for crates that are no longer maintained
#  - "notice" for other informational notices
#informational = "unmaintained"

# Categories this advisory falls under. Valid categories are:
# "code-execution", "crypto-failure", "denial-of-service", "file-disclosure"
# "format-injection", "memory-corruption", "memory-exposure", "privilege-escalation"
#categories = ["crypto-failure"]

# Common Vulnerability Scoring System score. More information
# can be found on the CVSS website, https://www.first.org/cvss/.
#cvss = "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H"

# Freeform keywords which describe this vulnerability, similar to Cargo (optional)
keywords = ["ssl", "mitm"]

# Vulnerability aliases, e.g. CVE IDs (optional but recommended)
# Request a CVE for your RustSec vulns: https://iwantacve.org/
#aliases = ["CVE-2018-XXXX"]

# Related vulnerabilities (optional)
# e.g. CVE for a C library wrapped by a -sys crate)
#related = ["CVE-2018-YYYY", "CVE-2018-ZZZZ"]

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

# Versions which include fixes for this vulnerability (mandatory)
# use patched = [] e.g. in case of unmaintained where there is no fix
[versions]
patched = [">= 1.2.0"]

# Versions which were never vulnerable (optional)
#unaffected = ["< 1.1.0"]
```

# RustSec Advisory Template - Advisory Title Goes Here

This is an example template for a RustSec advisory. Please copy this to
`crates/<crate-name>` and rename it to `RUSTSEC-0000-0000.md`.

In this section of the advisory you can write an extended description
of the vulnerability, will be converted into HTML and rendered at
<https://rustsec.org>.

- Markdown formatted
- TOML "front matter". See `README.md` for schema.
- Please include as much detail as you'd like.

A well structured advisory will include information like:

Affected versions of this crate did not properly check for integer overflow when allocating a buffer in `MyBuffer::with_capacity()` (bug description/location/root cause).

This can result in a memory corruption (consequence of the bug) when large integer is given to the parameter (trigger condition).

The flaw was corrected in commit abc123 by using `saturating_mul()` when calculating the buffer size (fix description).
