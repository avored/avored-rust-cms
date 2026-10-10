# Avored Rust CMS

Avored Rust CMS is a self-hosted content management system built with Rust, Leptos, Axum, and SurrealDB. It includes tools for managing entities, collections, email templates, and workflows, as well as a REST API.

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

## Useful commands

| Command | Description |
| --- | --- |
| `npm run build` | Build the Vite frontend assets |
| `npm run vite` | Start the Vite development server |
| `npm run leptos` | Start the Leptos development watcher |
| `npm run dev` | Run Vite and the Leptos watcher together |
| `cargo test` | Run the Rust test suite |

## Production

Do not use development secrets in production. Set `APP_ENV=prod` in `.env` and provide the required production settings in a protected `.env.prod` file. Use strong, unique values for `AVORED_PASSWORD_SALT` and `AVORED_JWT_SECRET`, restrict `AVORED_CORS_ALLOWED_APP_URL` to trusted origins, and serve the application behind HTTPS.
