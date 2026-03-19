# API Documentation (Rustdoc)

Icebox API docs are generated from Rust doc comments (`///` and `//!`) using `rustdoc`.

This page belongs under `docs/reference/` because it documents the generated API-doc surface and how it is built, rather than product architecture or execution planning.

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
- Keep `src/main.rs` thin and prefer crate/module contract docs in `src/lib.rs` and leaf modules.

---

*Last updated: 2026-03-18*
