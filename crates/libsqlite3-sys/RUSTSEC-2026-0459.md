```toml
[advisory]
id = "RUSTSEC-2026-0459"
package = "libsqlite3-sys"
date = "2026-10-30"
url = "https://nvd.nist.gov/vuln/detail/CVE-2026-119375"
references = [
    "https://sqlite.org/bugs/info/2026-08-17T15:54:41Z",
    "https://github.com/sqlite/sqlite/commit/e5bf119821f25be1d7f10ef0a2edbe175289166e",
    "https://sqlite.org/releaselog/3_54_0.html",
    "https://github.com/rusqlite/rusqlite/releases/tag/v0.41.0",
]
categories = ["memory-exposure"]
keywords = ["sqlite", "wal", "uninitialized-memory", "information-disclosure"]
aliases = ["CVE-2026-119375", "GHSA-72w2-rjgg-q729"]
cvss = "CVSS:4.0/AV:L/AC:L/AT:N/PR:N/UI:P/VC:H/VI:N/VA:N/SC:N/SI:N/SA:N"

[versions]
patched = [">= 0.39.0"]
unaffected = ["< 0.6.0"]
```

# `libsqlite3-sys` via C SQLite CVE-2026-119375

SQLite versions before 3.54.0 can return uninitialized page-cache memory when a process opens a database file together with a sibling write-ahead-log (`-wal`) file whose page size differs from the database's. A crafted pair of files can make ordinary queries return stale bytes from the process's memory. The attack vector is local and needs the victim to open the files.

As `libsqlite3-sys` bundles SQLite when it is built from its bundled source (the `bundled` feature, which `rusqlite`'s `bundled` feature enables), it is susceptible to the vulnerability. The releases `0.6.0` to `0.38.2` bundle SQLite `3.13.0` to `3.53.2` and are affected; the earlier releases contain no SQLite source. `libsqlite3-sys` was updated to bundle the patched version of SQLite in `0.39.0` (SQLite `3.54.0`) and `0.39.1` (SQLite `3.54.1`), released with [`rusqlite` 0.41](https://github.com/rusqlite/rusqlite/releases/tag/v0.41.0). `libsqlite3-sys` `0.39` declares Rust 1.88 and `rusqlite` `0.41` declares Rust 1.89.

Only a process that opens a database file, with its `-wal`, that a less trusted party could have written is exposed. A build that links a system SQLite instead of the bundled source is not affected by the copy in this crate; its exposure is that of the system library's version. The `bundled-sqlcipher` feature builds SQLCipher instead of SQLite; in `0.39.0` and `0.39.1` it depends on `sqlcipher-src =419.0.6`, which bundles SQLite `3.53.4` and is affected.
