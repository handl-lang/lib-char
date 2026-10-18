# lib-char

`char` — the character-library contract (axioms).

Part of the stdlib → `lib-*` split (ADR-0128); moved from
`stdlib/handl-packages/char/`.

`src/index.hnl` declares the 18 `char.*` axioms that every backend
(`char-native`, ASCII-only, JS, ICU, …) must satisfy. There is no Rust
implementation here — the embedding host provides the native `char`
module. All algorithms (`str-handl`) are written against this contract.
