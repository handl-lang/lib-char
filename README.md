# lib-char

`char` — the character-library contract (axioms).

Part of the stdlib → `lib-*` split (ADR-0128); moved from
`stdlib/handl-packages/char/`.

`src/index.hnl` declares the 18 `char.*` axioms that every backend
(`char-native`, ASCII-only, JS, ICU, …) must satisfy. There is no Rust
implementation here — the embedding host provides the native `char`
module. All algorithms (`str-handl`) are written against this contract.

## Documentation

Published site (Diátaxis): <https://handl-lang.github.io/docs/lib-char/>

- [Reference](https://handl-lang.github.io/docs/lib-char/reference/) — public API surface
- [How-to](https://handl-lang.github.io/docs/lib-char/how-to/) — task recipes
- [Explanation](https://handl-lang.github.io/docs/lib-char/explanation/) — design and rationale

Decision records live on the `project` branch under `project/adrs/`.
