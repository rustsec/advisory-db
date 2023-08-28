```toml
[advisory]
id = "RUSTSEC-0000-0000"
package = "crate-name"
date = "2020-01-31"
url = "https://example.com"
# Valid categories: "code-execution", "crypto-failure", "denial-of-service", "file-disclosure"
# "format-injection", "memory-corruption", "memory-exposure", "privilege-escalation"
categories = ["code-execution", "privilege-escalation"]
keywords = ["example", "freeform", "keywords"]
#aliases = ["CVE-YYYY-NNNN"]
#cvss = "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:C/C:H/I:H/A:H"

[versions]
patched = [">= 1.2.3, < 1.3.0", ">= 1.3.4"]
unaffected = ["<= 0.1.2"]

[affected]
#arch = ["x86"]
#os = ["windows"]
#functions = { "crate_name::MyStruct::vulnerable_fn" = [">= 1.3.0, < 1.3.4"] }
```

# RustSec Advisory Template - Advisory Title Goes Here

This is an example template for a RustSec advisory. Please copy this to
`crates/<crate-name>` and rename it to `RUSTSEC-0000-0000.md`.

In this section of the advisory, you can write an extended description
of the vulnerability, will be converted into HTML and rendered at
<https://rustsec.org>.

- Markdown formatted
- TOML "front matter". See `README.md` for schema.
- Please include as much detail as you'd like.

A well-structured advisory will include information like:

Affected versions of this crate did not properly check for integer overflow when allocating a buffer in `MyBuffer::with_capacity()` (bug description/location/root cause).

This can result in a memory corruption (consequence of the bug) when a large integer is given to the parameter (trigger condition).

The flaw was corrected in commit abc123 by using `saturating_mul()` when calculating the buffer size (fix description).
