```toml
[advisory]
id = "RUSTSEC-2022-0079"
package = "elf_rs"
date = "2022-10-31"
url = "https://github.com/vincenthouyi/elf_rs/issues/11"
categories = ["memory-corruption"]
keywords = ["elf", "header"]
aliases = ["GHSA-g6pw-999w-j75m"]

[versions]
patched = [">= 0.3.0"]

[affected]
```

# ELF header parsing library doesn't check for valid offset

The crate has several unsafe sections that don't perform proper pointer validation.

An example can be found in the following function:

```
fn section_header_raw(&self) -> &[ET::SectionHeader] {
    let sh_off = self.elf_header().section_header_offset() as usize;
    let sh_num = self.elf_header().section_header_entry_num() as usize;
    unsafe {
        let sh_ptr = self.content().as_ptr().add(sh_off);
        from_raw_parts(sh_ptr as *const ET::SectionHeader, sh_num)
    }
}
```

While this will work perfectly fine *if* the ELF header is valid, malicious or
malformed input can contain a section header offset of an arbitrary size, meaning
that the resultant pointer in the unsafe block can point to an artibrary address
in the address space of the process.

This can result in unpredictable behaviour, and in our fuzz testing, we discovered
that it's trivial to cause SIGABRT (signal 6), or SEGV (signal 11).

The function should either be marked as unsafe, with a note that the caller is responsible
for providing only valid inputs, or it should ideally do the due diligence to ensure that the
offset doesn't exceed the bounds of the header (and add additional checks as necessary).

