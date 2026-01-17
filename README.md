# Fragments

Fragments is a small Rust + Axum blog engine that renders Markdown posts into HTML and serves a minimal, typographic UI.

## Features
- Markdown posts with front-matter style metadata.
- Draft support via `/drafts/:slug`.
- Simple export mode to build a static site.
- Asset serving from `/assets`.

## Project structure
- `src/`: Rust application code (Axum server, routing, handlers).
- `posts/`: Markdown blog posts (one file per post).
- `assets/`: Static files (images, CSS, JS).
- `tests/`: Integration and end-to-end tests.

## Running locally
```sh
cargo run
```

Set a custom port with:
```sh
PORT=3000 cargo run
```

## Exporting a static site
```sh
cargo run -- --export
```

This writes static HTML to `./public`.

## Writing posts
- File naming: `posts/YYYY-MM-DD-slug_words.md`
- URL mapping: `/posts/slug-words` (strip `YYYY-MM-DD-`, replace `_` with `-`)

## Development tools
- `cargo test`: run tests.
- `cargo fmt`: format code.
- `cargo clippy`: lint.

## Deployment
Railway builds with the provided `Dockerfile` and runs the Axum server.

