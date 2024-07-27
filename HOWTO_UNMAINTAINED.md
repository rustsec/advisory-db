# HOWTO Guide: Unmaintained Crate Advisories

This document describes the policy for adding advisories for unmaintained
crates to the [RustSec Advisory Database].

These advisories serve to inform the Rust community about both the existence
of unmaintained crates within a particular project, and also serve to guide
switching to maintained alternatives.

When approaching a potentially unmaintained crate, do not behave rudely
towards open source maintainers. Submitting a RUSTSEC advisory for an
unmaintained crate should not be treated as a weapon to coerce open source
maintainers.

## Definition of an "unmaintained" crate

We consider crates unmaintained when they fall into either of the following
categories:

- Explicitly unmaintained: a crate's author has declared that they are no
  longer maintaining a particular crate.
- Implicitly unmaintained: the author is incommunicado for a prolonged period
  of time and cannot advise as to a crate's status.

## Creating an unmaintained crate advisory

### Policy

When in doubt, we always defer to the author of a crate's discretion as to
whether they would prefer an unmaintained crate advisory be filed, provided
we are able to make contact.

First and foremost: *ASK THE AUTHOR(S)*, preferably in a public issue on the
project's source code repository. If an author/maintainer of a particular crate
thinks filing an unmaintained crate advisory is a good idea, then great! Go ahead.

If the author is responsive and declares that the project *is* maintained, then
the RustSec organization considers it maintained and won't accept advisories about its maintenance status. Again, when in doubt, defer
to the author's discretion. So long as the author is responsive and avows that
a crate is maintained, we take them at their word. Repository metrics like
recent commits, open issues, latest release, etc are not reasons to go against
the direct word of a crate author.

However, if attempts have been made to contact a crate author have failed,
metrics like recent commits, open issues, time since last crate release etc
are important evidence to justify that a crate is unmaintained. An
incommunicado crate author is irrelevant if there is evidence that work is
continuing to happen on a crate.

To justify the "implicitly unmaintained" status, where a crate author is
unreachable, the following criteria must be met:

- Stale repository: no recent maintenance activity, including any of the
  following: recent commits, responses from the author on open issues,
  crate releases, or other publicly visible activity by the author.
  Inactivity over a period of 1 year or more is the preferred threshold.
- Contact attempts with the author made with no response. Ideally these
  attempts are made via a public GitHub issue, so that issue can be
  cited in an unmaintained crate advisory if need be. Unresponsiveness
  by the author over a period of 270 days (or 60 days of
  unresponsiveness after being notified of a vulnerability) is the
  minimum before a crate will be considered unmaintained.

### Process

Unmaintained crate advisories use the same structure as RustSec security
advisories, but include an `informational = "unmaintained"` attribute in
the TOML advisory.

When creating the advisory, please include a link to an open issue
on the upstream project repository where the maintenance status has been
discussed in the `url = "..."` field of the advisory.

For more information on adding an advisory to the RustSec DB, see:

<https://github.com/RustSec/advisory-db/blob/main/CONTRIBUTING.md>

### Questions

Please open a GitHub issue:

<https://github.com/rustsec/advisory-db/issues>

[//]: # (links)

[RustSec Advisory Database]: https://rustsec.org
