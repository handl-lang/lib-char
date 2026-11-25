# lib-char

`char` — the character-library contract (axioms).

`src/index.hnl` declares the 18 `char.*` axioms that every backend
(`char-native`, ASCII-only, JS, ICU, …) must satisfy. There is no Rust
implementation here — the embedding host provides the native `char`
module. All algorithms (`str-handl`) are written against this contract.

## Layout

```text
src/index.hnl            the contract (axiom declarations)
pkg.hnl                  package manifest
scripts/hnl-gate.sh      the gate
workflows/gate.yaml                    bupha entry (handl-gates)
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.

## Documentation

<https://handl-lang.github.io/docs/lib-char/>
