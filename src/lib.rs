//! `char` — the character-library contract.
//!
//! `src/index.hnl` is the abstraction contract: the irreducible floor that
//! every character backend (`char-native` on Rust `std::char`, ASCII-only,
//! JS, ICU, …) must satisfy. It contains only `axiom` declarations — there
//! is no Rust implementation in this crate; the embedding host supplies the
//! native `char` module.
//!
//! Extracted from `stdlib/handl-packages/char/` per the stdlib → `lib-*`
//! split (ADR-0128).

/// The handl source declaring the `char` contract.
pub const CHAR_HNL: &str = include_str!("index.hnl");
