# Version 0.2.0 - Parameterized storage

Date: 2020-10-29

## Changes

- Added the `BitStorage` trait. Types implementing this trait can be used as
  storage for a `GrowableBitMap`.
- Implement `BitStorage` for `u8`, `u16`, `u32`, `u64` and `u128`. `usize` is
  platform-dependant and I don't think is it appropriate for the crate to
  decide whether that is okay or not for your program.
- **BREAKING CHANGE** Use `BitStorage` as a type parameter in `GrowableBitMap`.
- **BREAKING CHANGE** `GrowableBitMap::new()` is not `const` anymore because of
  the change above.

## Migrating from 0.1.0

- Import `growable_bitmap::BitStorage`.
- Replace any use of `GrowableBitMap` by `GrowableBitMap<u8>` or
  `GrowableBitMap::<u8>`. You can then change `u8` to an integer type more
  suited to your needs if necessary.

# Version 0.1.0

A very basic working `GrowableBitMap` type with only `u8`s as backing storage.
