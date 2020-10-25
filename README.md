[![CI](https://github.com/poliorcetics/growable-bitmap/workflows/ci/badge.svg)](https://github.com/poliorcetics/growable-bitmap/actions)
![crates.io](https://img.shields.io/crates/v/growable-bitmap)
![crates.io](https://img.shields.io/crates/l/growable-bitmap)

# Growable bitmap

`growable-bitmap` is a Rust crate providing a growable (and shrinkable) compact
boolean array that can be parameterized on its storage type.

**THIS CRATE IS NOT CONSIDERED PRODUCTION READY AT THE MOMENT.**

## TODO

 This crate is not feature-complete at all. Below are some features I want
 to add before marking it as `1.0`:

 - `BitOr` (with another `GrowableBitMap`).
 - `BitOrAssign` (with another `GrowableBitMap`).
 - `BitAnd` (with another `GrowableBitMap`).
 - `BitAndAssign` (with another `GrowableBitMap`).
 - `BitXor` (with another `GrowableBitMap`).
 - `BitXorAssign` (with another `GrowableBitMap`).

 - All fixed unsigned integers as storage (`u16`, `u32`, `u64` and `u128`
   are missing).
 - When `const-generics` become available, possibly use them as storage ?

 - [Rust 1.48.0+ / Intra-doc links]: Use intra-doc links in documentation.
   Right now there are no links because they're painful to write once you've
   been introduced to the wonder intra-doc links are.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
growable-bitmap = "0.1"
```

and, if you're using Rust Edition 2015, this to your crate root:

```rust
extern crate growable_bitmap;
```

## Similar crates

> But bitmaps are not a new problem, why a new crate ?

This is true, in fact there are two libraries on `crates.io` that provides
bitmaps already:

- [`bitmap`](https://crates.io/crates/bitmap): marked as *complete* since 2016,
  which means it does not leverage new APIs in the standard library. Not a bad
  thing if you want absolute stability though.
- [`bitmaps`](https://crates.io/crates/bitmaps): Only fixed-size arrays, which
  is an explicit non-goal of `growable-bitmap`. `bitmaps` and `growable-bitmap`
  complement each other and you should choose the correct one for you usage.

**And** I wanted to make a bitmap crate because I think bitmaps are a very cool
data structure and I love using Rust to build things.

## License

See the `LICENSE` file at the root of the repository.
