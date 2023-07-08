```toml
[advisory]
id = "RUSTSEC-2023-0042"
package = "ouroboros"
date = "2023-06-11"
url = "https://github.com/joshua-maros/ouroboros/issues/88"
informational = "unsound"
aliases = ["GHSA-87mf-9wg6-ppf8"]

[versions]
patched = [">=0.16.0"]
```

# Ouroboros is Unsound

## Summary

Ouroboros has a soundness problem, but a fix has been implemented in 0.16.0.

## More details:

In 0.15.0, Ouroboros works internally by creating a struct where all 
uses of 'this are replaced by 'static. However, a recent addition to 
Miri checks that references passed to functions are valid during the 
entire execution of the function, even when those references are 
passed inside a struct. This poses an issue for dropping 
self-referencing values, as the reference becomes invalid during the 
dropping process. Effectively, since self-referencing structs are 
not allowed in vanilla Rust, there is no allowance for dropping data
during a function that has also been given a reference to that data. 
There's usually no way to pass a and &a to the same function.

A fix was attempted, where the struct would be turned in to a raw 
byte array and only transformed back into the underlying data type 
inside each function. This is allowable as a reference can be 
created and die over the body of a function, as long as the 
reference was created inside the function. However, this is also 
not sound if the original struct contains padding. There is no 
way to initialize padding bytes to a known value that Miri will 
accept, so when it is cast to an array some of the bytes contain 
uninitialized values. This is not acceptable (despite the fact 
that these bytes are never read) due to the potential for 
optimizations that may read from the uninitialized bytes. Besides 
which, this fix does not allow for template or constant parameters 
as there is no way to check the size of a templated type without 
giving specific, concrete values for the template parameters.

The final solution was to use this fix while wrapping the raw byte
array in `MaybeUninit<>`.
