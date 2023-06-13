```toml
[advisory]
id = "RUSTSEC-2021-0121"
package = "crypto2"
date = "2021-10-08"
url = "https://github.com/shadowsocks/crypto2/issues/27"
informational = "unsound"
keywords = ["crypto", "alignment", "unsound"]
aliases = ["CVE-2021-45709", "GHSA-9hfg-pxr6-q4vp", "GHSA-pmcv-mgcf-rvxg"]

[affected.functions]
"crypto2::streamcipher::Chacha20::encrypt_slice" = ["*"]
"crypto2::streamcipher::Chacha20::decrypt_slice" = ["*"]
"crypto2::streamcipher::xor_si512_inplace" = ["*"]

[versions]
patched = []
```

# Non-aligned u32 read in Chacha20 encryption and decryption
The implementation does not enforce alignment requirements on input slices while incorrectly assuming 4-byte alignment through an unsafe call to `std::slice::from_raw_parts_mut`, which breaks the contract and introduces undefined behavior.

This affects Chacha20 encryption and decryption in crypto2.
