---
name: icebox-rust-standards
description: Enforce Icebox Rust code organization, visibility, crate-boundary, and module-layout conventions. Use for Rust file structure, `mod`/`pub mod`, `main.rs`/`lib.rs`, and repo style decisions.
---

# Icebox Rust Standards

Use this skill for Rust code structure and style decisions. Do not duplicate mdBook or general rustdoc policy from `icebox-docs-standards`; use this skill for code organization, API exposure, and module layout choices.

1. Crate boundaries:
   - For this CLI, keep `src/main.rs` thin; it should start the program and pass through the final exit code so core behavior stays in `src/lib.rs`, where it is easier to test and reuse.
   - Prefer `src/lib.rs` for reusable CLI logic, orchestration, parsing, and behavior that should be testable outside the binary entrypoint.
   - Treat `src/main.rs` as the binary crate root and `src/lib.rs` as the primary reusable crate surface.
2. Module visibility:
   - Default to `mod` unless a module is intentionally part of the crate's public surface.
   - Use `pub mod` only when external callers should navigate through that module path.
   - Keep the public surface narrow; expose the smallest API needed for current use.
   - Public modules should contain meaningful public items; avoid exporting empty or incidental namespaces.
3. Module layout:
   - Prefer `foo.rs` over `foo/mod.rs` for parent modules in modern Rust code.
   - When a module grows child modules, prefer `src/foo.rs` plus `src/foo/*.rs`.
   - Keep layout consistent with the surrounding crate if an area already uses a clear pattern.
4. Comment and doc placement:
   - Put local implementation notes next to the relevant code.
   - Put item docs on the item they describe, and module or crate docs at the module boundary.
   - Prefer comments that match the scope of the behavior they explain.
5. Naming and structure:
   - Use module names that describe responsibility, not implementation trivia.
   - Prefer small, responsibility-focused modules over large grab-bag files.
   - Split by domain or behavior when a file starts mixing unrelated concerns.
6. Error and utility boundaries:
   - Keep helpers in `util` private unless there is a clear reuse boundary.
   - Avoid turning internal error plumbing into public API unless downstream callers need it.
7. Review rubric:
   - Ask whether a new module needs to be public at all.
   - Ask whether a new file belongs in `lib.rs`, `main.rs`, or a feature module.
   - Ask whether the chosen layout improves discoverability in editors, search, and rustdoc.
