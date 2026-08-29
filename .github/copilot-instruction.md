# AvoRed CMS Instructions

## Stack & Architecture
- **Backend:** Rust, Tokio, Axum (RESTful API), SurrealDB, Tracing (`tracing`).
- **Frontend:** Leptos (Admin HTML rendering), Alpine.js (DOM manipulation), Tailwind CSS.
- **Architecture:** DDD & Onion Architecture. Soft deletes preferred across models.
- **Localization:** `rust-i18n` with locale files in `resources/locales`.
- **Workflow Note:** Verification, tests, `cargo check`, and builds are handled manually by the user.

## Rust Guidelines
- **Safety & Quality:** Idiomatic Rust (RFC 430), borrow checker first, no `unsafe`. Pass `cargo clippy`.
- **Errors:** No `.unwrap()` or `.expect()` in production code. Prefer `Result<T, E>`, `?`, `thiserror`/`anyhow`.
- **Performance:** Prefer borrowing (`&T`, `&str`) and zero-copy/iterators over `.clone()`.
- **Types & Structure:** Expressive enums/structs with `serde`. Modular architecture (`async`/`await` on Tokio).

## UI & Styling (Tailwind CSS)
- **Design:** Modern, polished aesthetic with smooth borders (`rounded-lg`/`rounded-xl`, `shadow-sm`).
- **Dark Mode:** Always include dark mode variants (`dark:...` for bg, text, borders).
- **Interactions:** Responsive layouts (`flex`/`grid`) with fluid hover/focus transitions (`transition-all duration-200`).
