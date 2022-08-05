# RustSec advisory-db maintainer's guide

Reviewing a pull request, step by step:

## Check Triage Labels

Categorise by what the PR Author intent is - see: Propose-Close/Merge for proposed.

- `Unsound` - The PR author suggests the advisory is informational/unsound
- `Unmaintained` - The PR author suggests the advisory is informational/unmaintained

It is important to ensure we know what is going on with each PR so we can efficiently check and follow-up.

- `Waiting-Maintainer` - We are waiting for the Maintainer to provide any response if any.
- `Waiting-Author` - We are waiting for the Author of PR

It is also useful to radiate the intent when dealing with PRs - Especially controversial ones to allow feedback.

- `Propose-Close` - PR may be Closed soon if nobody objects.
- `Propose-Merge` - PR may be Merged soon if nobody objects.

Objection to either Merge or Close can be short period of time, typically two days.

## See if CI passes

This is something first-time submitters may struggle with.
 
You can usually make changes directly to the sumbitter's branch. It's a great way to make CI pass and help out first-timers, but avoid making substantial changes to content this way without consuling the submitter.

## Make sure the developers of the crate in question are aware of the issue

If no upstream issue has been filed, ask the reporter to file one first.

## Check if there are any fixed versions
We don't want to carry a non-actionable advisory if a fix is forthcoming. It's alright to delay by a day or two and then publish it once the fix ships.

If a fix has been applied in git but not released to crates.io, ask the upstream for a new point release.

If the upstream is unresponsive or is not interested in fixing the issue, we can carry the advisory anyway, but this can be a delicate matter. See [here](https://github.com/rustsec/advisory-db/issues/1092) for guidance, and feel free to consult the [Rust Moderation team](https://www.rust-lang.org/governance/teams/moderation) in case of doubt.

## Make sure the advisory text is clear and actionable
If you don't understand what's going on, most users won't either. Ask the submitter specific questions to clarify the advisory text, if needed.

Avoid editing the text directly without clearing it with the submitter first; use the "Suggest changes" feature instead, or just plain comments on the pull request. The experience of other databases shows that the database maintainers may be missing some context, and unilateral changes may lead to incorrect advisories, so make sure the submitter confirms that the changes are correct.

## Check that the metadata is correct
The date should be set to the original disclosure of the issue, not the date of the pull request.

`informational = "unsound"` is used for [soundness issues](https://rust-lang.github.io/unsafe-code-guidelines/glossary.html#soundness-of-code--of-a-library) that can only be triggered by a programmer (as opposed to e.g. a malicious input), and/or require very contrived code to trigger.

`informational = "unmaintained"` has a [policy](https://github.com/rustsec/advisory-db/blob/main/HOWTO_UNMAINTAINED.md) associated with it; make sure it is followed. If any items are missing, advise the submitter how to proceed.

## Check if the advisory came from GHSA

GHSA advisory info is licensed under [CC-BY-4.0](https://docs.github.com/en/site-policy/github-terms/github-terms-for-additional-products-and-features#12-advisory-database), while most other databases (including RustSec) are in the public domain.

If the advisory came from GHSA, we have two options:

- Either use only data published in the associated CVE, since the CVE database is in the public domain
- Or ask the submitter of the GHSA advisory (not the RustSec pull request author) to release the advisory contents into the public domain


## Thank the submitter and any other participants

Let them know they did a nice and useful thing. That's how you get repeat submitters ;)
