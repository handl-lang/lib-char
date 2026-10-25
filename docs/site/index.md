# lib-char

The character-library contract: the irreducible floor every backend satisfies.

## What it is

`char` is an **abstraction contract**, not an implementation: the axioms and
laws that every character backend must satisfy. All `str-handl` algorithms are
written on top of it, so swapping the backend (`char-native` on Rust
`std::char`, an ASCII-only one, a JS one, a future ICU one) changes semantics —
Unicode vs ASCII vs locale — without touching a single string algorithm.

## Modules
- `char` — the contract module: predicates, transforms, indexing, code points


## Start here

- **Reference** — [public API](/lib-char/reference/)
- **How-to** — [task recipes](/lib-char/how-to/)
- **Explanation** — [design and rationale](/lib-char/explanation/)

No tutorials: this repo is a building block used from Rust. End-to-end
walkthroughs live with the product that composes it — see the
[Handl platform](/handl/).
