# lib-char

`char` — the character-library **contract**: `src/index.hnl` declares
the 18 `char.*` axioms every backend (`char-native`, ASCII-only, JS,
ICU, …) must satisfy. No Rust implementation lives here — the embedding
host provides the native `char` module, and all algorithms (`str-handl`)
are written against this contract.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the contract — axiom declarations |
| `tests/contract.rs` | Rust-side contract checks |
| `docs/` | published docs source |
| `pkg.hnl`, `scripts/hnl-gate.sh` | manifest + `.hnl` gate |
| `workflows/gate.yaml` | CI — shared `handl-gates` campaign |

## Contract

- This repo is axioms only — do not add a `char` implementation here;
  backends are sibling repos (`lib-char-native`, …).
- Arrow/contract-dialect `.hnl` is expected; `hnl-gate.sh` detects and
  skips non-canonical files.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
