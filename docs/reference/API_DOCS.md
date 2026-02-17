# API Documentation (Rustdoc)

Icebox API docs are generated from Rust doc comments (`///`) using `rustdoc`.

## Build Locally

```bash
cargo doc --workspace --all-features --no-deps
```

Generated output:

- `target/doc/index.html`

## CI Behavior

- `.github/workflows/docs-site.yml` builds rustdoc when `Cargo.toml` is present.
- The artifact publishes API docs under `site/api/`.

## Authoring Guidance

- Prefer module-level docs with `//!` for crate/module boundaries.
- Add public API docs to exported items before release.
- Keep examples compilable where practical.

---

*Last updated: 2026-02-17*
