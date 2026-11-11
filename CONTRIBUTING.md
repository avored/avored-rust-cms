# Contributing to Avored Rust CMS

Thanks for being interested in contributing to Avored Rust CMS. This project is a Rust workspace with an Axum backend, a Leptos SSR frontend, and a Vite/Tailwind asset pipeline. The guidance below is intended to keep local development predictable and make review easier for maintainers.

## Before you start

- Open or reference the relevant GitHub issue before starting larger work.
- Keep changes small and scoped to the issue or feature you are addressing.
- If a change affects public behaviour, update the relevant docs and examples.

## Local setup

### Prerequisites

- Rust and Cargo
- `wasm32-unknown-unknown` target
- Node.js and npm
- `cargo-leptos`

Install the Rust target and Leptos tooling if needed:

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked
```

### Install dependencies

```bash
npm install
```

### Configure local environment

Create a `.env` file in the project root with the same values used by the app locally:

```dotenv
APP_ENV=local
AVORED_DATABASE_NAMESPACE=avored
AVORED_DATABASE_NAME=avored_cms
AVORED_DATABASE_FOLDER_NAME=rocksdb://data/avored.db
AVORED_PASSWORD_SALT=replace-with-a-random-secret
AVORED_JWT_SECRET=replace-with-another-random-secret
AVORED_CORS_ALLOWED_APP_URL=http://localhost:3000
```

Generate unique random values with:

```bash
openssl rand -hex 32
```

Do not commit `.env` files. Keep production secrets out of the repository and use environment-specific files (`.env.dev`, `.env.test`, etc.) only when needed.

### Run the app

```bash
npm run dev
```

This starts Vite and the Leptos development watcher. The application listens on port `3000` by default.

## Project layout

```text
crates/
  avored-api/      # Axum server, router registrations, HTTP handlers
  avored-core/     # domain, DTOs, repo traits, app use cases
  avored-web/      # Leptos UI and hydration-facing web components

docs/
  architecture/     # architecture and frontend hydration notes
  roadmap/         # planning and backlog documents
  tech-debt/        # debt and implementation follow-ups
```

## Development workflow

### Rust

Use the workspace default formatting and compile checks before submitting changes:

```bash
cargo fmt
cargo test
```

If you are working on backend or API behaviour, prefer targeted integration tests where possible rather than broad, noisy test runs.

### Frontend

Use the project's Vite/Tailwind workflow when modifying styles or frontend assets:

```bash
npm run build
```

When adjusting Leptos components, keep SSR and hydration safety in mind. The project already documents the hydration boundaries in [docs/architecture/leptos-ssr-hydration-boundaries.md](docs/architecture/leptos-ssr-hydration-boundaries.md).

## Coding standards

- Prefer small, readable functions and domain-specific types.
- Keep `Axum` handlers and `Leptos` views aligned with the existing project structure rather than introducing ad hoc patterns.
- Add or update docs when a user-facing or developer-facing behaviour changes.
- Make validation and error handling explicit, especially for HTTP and database failures.
- Avoid leaking implementation details into end-user-facing error messages.

## Pull request checklist

Before opening a PR, confirm the following:

- The change is scoped to a single issue or improvement.
- The relevant tests or checks have been run.
- The README or developer docs have been updated when needed.
- The code is formatted and consistent with the existing Rust workspace style.
- The description explains the user problem, the fix, and any follow-up work.

## Useful references

- [README](README.md)
- [Architecture overview](docs/architecture/leptos-ssr-hydration-boundaries.md)
- [API route/versioning reference](docs/api-route-versioning.md)
- [Roadmap](docs/roadmap/improvement-and-tech-debt-roadmap.md)

## Questions or help

If you are unsure how to proceed, start by checking the existing docs and issue thread for the feature area you are working in. A focused question tied to the exact behavior or bug is usually the fastest way to unblock progress.
