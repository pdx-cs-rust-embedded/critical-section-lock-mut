![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![crates-io](https://img.shields.io/crates/v/critical-section-lock-mut.svg)](https://crates.io/crates/critical-section-lock-mut)
[![api-docs](https://docs.rs/critical-section-lock-mut/badge.svg)](https://docs.rs/critical-section-lock-mut)
[![dependency-status](https://deps.rs/repo/github/BartMassey/critical-section-lock-mut/status.svg)](https://deps.rs/repo/github/BartMassey/critical-section-lock-mut)

# critical-section-lock-mut: simple locked mutable globals for embedded
Copyright © Bart Massey 2024 (Version 0.1.2)

Lock data with mutable access on a single-core
processor.  The locking is provided by a
`critical-section::Mutex`.

## Example

```
use critical_section_lock_mut::LockMut;

static SHARED: LockMut<u8> = LockMut::new();

fn main() {
    SHARED.init(3);
    SHARED.with_lock(|u| *u += 1);
```

# License

This work is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.

# Acknowledgments

Thanks to the `cargo-readme` crate for generation of this `README`.

# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
