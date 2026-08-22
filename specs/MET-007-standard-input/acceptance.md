# MET-007 Acceptance Evidence

- **Feature**: MET-007
- **Azure work item**: 247
- **Pinned Radon reference**: `54b88e5878b2724bf4d77f97349588b811abdff2`
- **Scope**: Python standard-input discovery

## Observed scenarios

| Supplied paths | Reader observation | Public seam observation |
| --- | --- | --- |
| `[-]` | Consumed to completion once logically | One complete input named `-` |
| `[-, -]` and `[-, -, -]` | Consumed to completion once logically per request | One complete input named `-` per request |
| `[-, sample.py]` | Not accessed | Exact ordered collection delegated once |
| `[first.py, -, second.py, -]` | Not accessed | Exact order and multiplicity delegated once |
| `[sample.py]` | Not accessed | Exact collection delegated once |
| `[]` | Not accessed; no implicit stdin | Exact empty collection delegated once |
| Standard-input read failure | Failure propagated | No partial consumer handoff |

The focused integration tests observe behavior only through the public input-discovery,
`MetricInputConsumer`, and non-standard-input delegate seams. They do not open delegated
paths, parse Python, calculate metrics, or assert a low-level `Read::read` call count.

## Verification

The following commands completed successfully in `rusty-kode-dev` at
`/workspaces/rusty-kode`:

```text
cargo test --test standard_input
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```
