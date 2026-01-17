# Project Plan

## Goals
- Ship a fast markdown-powered blog with a clean reading experience.
- Keep the Rust service small, testable, and easy to extend.
- Support local authoring and deployment with minimal ceremony.

## Proposed Stack
- Runtime: Rust + Axum for HTTP routing and middleware.
- Rendering: generate HTML directly in Rust (no template engine).
- Markdown: `pulldown-cmark` (with front matter via `serde_yaml`).
- Static assets: plain CSS with `postcss` optional; keep it minimal to start.
- Content: Markdown files in `posts/` with `YYYY-MM-DD-slug_words.md` (e.g., `2025-01-07-changes_is_important.md`).
- Data model: in-memory index at startup; add `sled` or SQLite if search/tags grow.
- Config: `.env` via `dotenvy`, plus `config` crate for typed settings.
- Logging/Tracing: `tracing` + `tracing-subscriber`.
- Testing: Rust test harness + `reqwest` for integration tests.

## Milestones
1. Scaffold Axum app with `/posts/:slug` route only (no index yet).
2. Build markdown loader + front matter parser for `posts/`.
3. Implement slug mapping (strip `YYYY-MM-DD-`, replace `_` with `-`).
4. Render templates and ship basic styling.
5. Add tests and CI (fmt, clippy, tests).

## Open Decisions
- Do we want a full static export mode (pre-render to `public/`)?
- Choose a CSS approach: handcrafted or minimal utility set?
- Confirm the date prefix format is `YYYY-MM-DD` (with a dash separator to the slug).
