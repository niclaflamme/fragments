# Repository Guidelines

## Project Structure & Module Organization
- `src/`: Rust application code (Axum server, routing, handlers).
- `posts/`: Markdown blog posts (one file per post).
- `assets/`: Static files (images, CSS, JS).
- `tests/`: Integration and end-to-end tests.

## Build, Test, and Development Commands
- `cargo run`: build and run the Axum server locally.
- `cargo test`: run the test suite.
- `cargo fmt`: format Rust code with rustfmt.
- `cargo clippy`: lint with Clippy; fix warnings before PRs.

## Coding Style & Naming Conventions
- Rust: 4-space indentation via rustfmt; `snake_case` for modules/functions, `CamelCase` for types.
- Axum handlers: keep request/response types in `src/handlers/` or `src/routes/` as the codebase grows.
- Content files: `posts/YYYY-MM-DD-slug_words.md` (example: `posts/2025-01-07-changes_is_important.md`).
- URL mapping: `/posts/changes-is-important` (strip `YYYY-MM-DD-`, replace `_` with `-`).

## Testing Guidelines
- Framework: Rust’s built-in test harness (`#[test]`).
- Name tests after behavior: `test_list_posts_returns_sorted`.
- Prefer integration tests for HTTP routes in `tests/` using `reqwest` or `axum::Router` test helpers.

## Commit & Pull Request Guidelines
- No established commit convention yet; use Conventional Commits (`feat:`, `fix:`, `docs:`).
- PRs should include a concise summary, testing notes, and screenshots for UI changes.

## Configuration & Local Setup
- Use `.env` for local config (e.g., `PORT=3000`); avoid committing secrets.
- Document new config keys in `README.md` when added.
