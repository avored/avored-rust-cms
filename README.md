# Avored Rust CMS

> A modern, self-hosted Content Management System built entirely in Rust — fast, type-safe, and batteries included.

Avored Rust CMS is an open-source headless CMS and admin platform built with **Rust**, **Leptos**, **Axum**, and **SurrealDB**. It combines a reactive server-side-rendered admin UI with a clean REST API, giving you full control over your content without sacrificing developer experience.

---

## 💡 Idea

Most CMS platforms are either too heavy (monolithic PHP stacks), too limiting (SaaS-only), or require a patchwork of services to get started. Avored Rust CMS aims to be **a single, self-contained binary** that you can drop on a server and have a production-grade content management system running in minutes — with no external database server, no container orchestration, and no JavaScript runtime required on the server.

Built in Rust from the ground up, it targets:

- **Performance** — near-zero overhead, handling thousands of requests with minimal memory footprint.
- **Safety** — the Rust type system and Leptos's reactive model eliminate entire classes of runtime errors.
- **Developer productivity** — a single codebase for both the admin UI (Leptos/WASM) and the backend API (Axum), with shared types across the boundary.
- **True self-hosting** — embedded SurrealDB means no external dependencies. Clone, configure, run.

---

## ✨ Features

### 🗂 Content Modelling
- **Entities** — define custom content types (e.g. Blog Post, Product, FAQ) with flexible attribute schemas.
- **Attributes** — attach typed fields (text, number, date, boolean, relation) to any entity.
- **Collections** — group and organise content records across entities.
- **Entity Identifier Management** — rename and re-slug entity identifiers without losing data.

### 🔄 Workflows
- **Custom Workflows** — create multi-step publishing workflows (e.g. Draft → Review → Published).
- **Workflow Executions** — track the full execution history of every workflow run.
- **Activate / Deactivate** — toggle workflows on or off at any time via the API.

### 📧 Email Templates
- **Managed Templates** — create, update, and delete reusable email templates from the admin or API.
- **Paginated Listing** — browse all templates with server-side pagination.

### 🔐 Authentication & Security
- **JWT-based Authentication** — stateless, secure token authentication for every API request.
- **Argon2 Password Hashing** — industry-standard password hashing with configurable salt.
- **Role-based Access Control** — user model with granular permission support.
- **CORS Configuration** — restrict API access to trusted origins via environment variable.
- **Multi-environment Secrets** — separate `.env` files per environment (`local`, `dev`, `test`, `stag`, `prod`).

### 🌐 REST API
- **Full CRUD** for Entities, Collections, Email Templates, and Workflows.
- **Versioned API** (`/api/v1/`) for stable, public-facing content access.
- **Auth endpoint** — `/api/auth/login`
- **Setup endpoint** — `/api/misc/setup` for first-run administrator bootstrapping.
- **OpenAPI-friendly** route structure (GET / POST / PUT / PATCH / DELETE per resource).

### 🖥 Admin UI (Leptos + WASM)
- **Server-Side Rendered** admin panel powered by Leptos with client-side hydration.
- **Reactive components** — real-time UI updates without full page reloads.
- **i18n / Internationalisation** — built-in multi-language support via `rust-i18n`.
- **Local browser storage** — preferences and session data persisted via `gloo-storage`.

### 🛢 Database
- **Embedded SurrealDB** — no external database required; data lives in `rocksdb://data/avored.db`.
- **Schema migrations** — applied automatically on startup.
- **In-memory mode** — available for testing (`kv-mem` feature).

### 📦 Build & Deployment
- **Single binary** — compile once, run anywhere.
- **Vite frontend pipeline** — fast asset bundling with hot-module replacement in development.
- **WASM-optimised release profile** — size-optimised (`opt-level = 'z'`, LTO) WASM bundle for production.
- **Static asset serving** — built-in file serving for `/public` and `/assets` routes.

---

## 🗺 Roadmap

- [ ] API Key management (create, rotate, revoke)
- [ ] Media / file upload support
- [ ] Webhook triggers on content events
- [ ] GraphQL API layer
- [ ] Plugin / extension system
- [ ] CLI tooling (`avored-cli`) for migrations and seeding
- [ ] Docker & Docker Compose support
- [ ] Audit log / activity history

---

## 🧰 Tech Stack

| Layer | Technology |
|---|---|
| Language | Rust (2021 edition) |
| Web Framework | Axum 0.8 |
| Frontend UI | Leptos 0.8 (SSR + WASM hydration) |
| Database | SurrealDB 3 (embedded RocksDB) |
| Auth | JWT (`jsonwebtoken`) + Argon2 |
| Asset Bundling | Vite + `cargo-leptos` |
| i18n | `rust-i18n` |
| Async Runtime | Tokio |

---

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) and Cargo
- The `wasm32-unknown-unknown` Rust target
- [Node.js](https://nodejs.org/) and npm
- The `cargo-leptos` command-line tool

Install the Rust target and `cargo-leptos` if you don't already have them:

```sh
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked
```

---

## Get started

1. Clone the repository and enter the project directory:

   ```sh
   git clone https://github.com/avored/avored-rust-cms.git
   cd avored-rust-cms
   ```

2. Install the JavaScript dependencies:

   ```sh
   npm install
   ```

3. Create a `.env` file in the project root with the following local-development settings:

   ```dotenv
   APP_ENV=local
   AVORED_DATABASE_NAMESPACE=avored
   AVORED_DATABASE_NAME=avored_cms
   AVORED_DATABASE_FOLDER_NAME=rocksdb://data/avored.db
   AVORED_PASSWORD_SALT=replace-with-a-random-secret
   AVORED_JWT_SECRET=replace-with-another-random-secret
   AVORED_CORS_ALLOWED_APP_URL=http://localhost:3000
   ```

   Generate a separate random value for each secret, then replace the placeholders:

   ```sh
   openssl rand -hex 32
   ```

   Running the command twice produces independent values. Keep `.env` private and do not commit it. For other environments, the application loads `.env.dev`, `.env.test`, `.env.stag`, or `.env.prod` when `APP_ENV` is set to `dev`, `test`, `stag`, or `prod`, respectively.

4. Build the frontend assets:

   ```sh
   npm run build
   ```

5. Start the development servers:

   ```sh
   npm run dev
   ```

   This runs Vite and `cargo leptos watch`. The CMS server listens on port **3000**.

6. Open [http://localhost:3000/setup](http://localhost:3000/setup) and create your administrator account. After setup, sign in at [http://localhost:3000/auth/login](http://localhost:3000/auth/login).

The application uses embedded SurrealDB storage, so no separate database server is needed. It creates the database and applies schema migrations on startup. With the settings above, the database is stored at `data/avored.db`.

---

## Useful commands

| Command | Description |
| --- | --- |
| `npm run build` | Build the Vite frontend assets |
| `npm run vite` | Start the Vite development server |
| `npm run leptos` | Start the Leptos development watcher |
| `npm run dev` | Run Vite and the Leptos watcher together |
| `cargo test` | Run the Rust test suite |

---

## Production

Do not use development secrets in production. Set `APP_ENV=prod` in `.env` and provide the required production settings in a protected `.env.prod` file. Use strong, unique values for `AVORED_PASSWORD_SALT` and `AVORED_JWT_SECRET`, restrict `AVORED_CORS_ALLOWED_APP_URL` to trusted origins, and serve the application behind HTTPS.

---

## Contributing

Contributions are welcome! Please open an issue to discuss the change you'd like to make before submitting a pull request. All code must pass `cargo test` and be formatted with `cargo fmt`.

---

## License

This project is open-source. See the [LICENSE](LICENSE) file for details.
