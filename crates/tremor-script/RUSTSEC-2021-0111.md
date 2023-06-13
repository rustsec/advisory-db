```toml
[advisory]
id = "RUSTSEC-2021-0111"
package = "tremor-script"
date = "2021-09-16"
url = "https://github.com/tremor-rs/tremor-runtime/pull/1217"
categories = ["memory-corruption", "memory-exposure"]
aliases = ["CVE-2021-45701", "CVE-2021-45702", "GHSA-3pp4-64mp-9cg9", "GHSA-9qvw-46gf-4fv8", "GHSA-q2x5-6q7q-r872"]

[versions]
patched = [">= 0.11.6"]
unaffected = ["<= 0.7.2"]
```

# Memory Safety Issue when using `patch` or `merge` on `state` and assign the result back to `state`

Affected versions of this crate maintains references to memory that might have been freed already.
If affects the following two `tremor-script` language constructs:

* A [Merge](https://www.tremor.rs/docs/tremor-script/index#merge) where we assign the result back to the target expression
  and the expression to be merged needs to reference the `event`:

```
let state = merge state of event end;
```

* A [Patch](https://www.tremor.rs/docs/tremor-script/index#patch) where we assign the result back to the target expression
  and the patch operations used need to reference the `event`:

```
let state = patch state of insert event.key => event.value end;
```

For constructs like this (it doesnt matter what is references in the expression to be merged or the patch operations) an optimization
was applied to manipulate the target value in-place, instead of cloning it.

Our `Value` struct which underpins all event data in `tremor-script`, is representing as borrowed strings `beef::Cow<'lifetime, str>` 
that reference the actual `Vec<u8>` the event is based upon. We keep the raw byte-array next to the event structure inside our `Event` struct as a self-referential struct,
so we make sure that the structured `Value` and its references are valid across its whole lifetime.

The optimization was considered safe as long as it was only possible to merge or patche `event` data or static data.
When `state` was introduced to `tremor-script` a new possibility existed, to keep `Value` data around for longer than the lifetime of an event.
If `event` data is merged or patched into `state` without cloning `state` first, it can still reference keys or values from
the previous event, which will now be invalid. This allows access to those already freed regions of memory and to get their content out over the wire.


## Workaround

If an upgrade is not possible, a possible workaround is to avoid the optimization
by introducing a temporary variable and not immediately reassigning to `state`:

```
let tmp = merge state of event end;
let state = tmp
```

## Fix

The flaw was corrected in `tremor-script` version 0.11.6 via commit [1a2efcd](https://github.com/tremor-rs/tremor-runtime/commit/1a2efcdbe68e5e7fd0a05836ac32d2cde78a0b2e) by removing the optimization
and always clone the target expression of a [Merge](https://www.tremor.rs/docs/tremor-script/index#merge) or [Patch](https://www.tremor.rs/docs/tremor-script/index#patch.

