# cushion-treemap -- AGENTS.md

Library crate: UI-agnostic squarified hierarchical cushion-treemap layout.
Computes rectangles; renderers draw them. No I/O, no renderer deps (only `serde`).

## Layout

- `src/lib.rs` (docs + re-exports), `src/model.rs` (`LayoutRect`, `Tile`, `TreemapTree`),
  `src/layout.rs` (squarify + hierarchical), `src/color.rs` (WizTree-style palette).
- `examples/synthetic_svg.rs`: std-only demo, writes hover-tooltip SVG to the temp dir.
- Lints (`Cargo.toml`): `unwrap_used`, `expect_used`, `panic`, `unimplemented`, `todo` denied.

## Gates

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run --example synthetic_svg
```

Keep it dependency-free (serde only). Colors are optional helpers; layout never looks at color.
