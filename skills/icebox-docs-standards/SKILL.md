---
name: icebox-docs-standards
description: Enforce repository documentation standards for mdBook + rustdoc outputs, docs structure hygiene, and publish-ready Markdown updates. Apply on every task in this repository.
---

# Icebox Docs Standards

Use this skill on every task in this repository.

1. Documentation surfaces:
   - Human docs site: `mdbook` from `docs/` using `book.toml`.
   - API docs: `rustdoc` via `cargo doc` when `Cargo.toml` is present.
2. Keep docs navigation current:
   - Update `docs/SUMMARY.md` when adding/removing docs pages.
   - Keep `docs/README.md` aligned with canonical sections.
3. Preserve source-of-truth boundaries:
   - Architecture contracts in `docs/architecture/`.
   - Planning/execution docs in `docs/plan/`.
   - Machine-readable contracts in `docs/reference/`.
4. Keep Markdown footer policy consistent in touched docs:
   - `---`
   - `*Last updated: YYYY-MM-DD*`
5. CI/docs automation expectations:
   - Keep `.github/workflows/docs-site.yml` building mdBook and rustdoc artifacts.
   - If Rust code is introduced, ensure API docs generation remains enabled.
6. Publish readiness:
   - Prefer stable Markdown and relative links suitable for wiki/site reuse.
   - Avoid one-off docs islands; integrate into existing indexes.
7. Rust source documentation standards:
   - Document stable public API items (`pub` structs, enums, traits, fns, type aliases, constants) with `///`.
   - Document module/crate boundaries with `//!` where they define external behavior or usage.
   - Use comment layering consistently:
     - `///` and `//!` for API/module contracts and user-facing behavior.
     - `//` for selective implementation notes on non-obvious lines or blocks.
     - Avoid line-by-line comments for obvious code.
   - Keep `src/main.rs` thin; treat `src/lib.rs` as the primary API documentation surface.
   - Prefer examples on high-impact public APIs; examples should compile when practical.
8. Rustdoc validation expectations:
   - Run `cargo doc --workspace --all-features --no-deps` for local/API docs validation.
   - In CI, prefer strict rustdoc warnings once clean: `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps`.
   - If examples are intentionally non-runnable, mark them explicitly (`ignore`, `no_run`) rather than leaving ambiguous failures.
