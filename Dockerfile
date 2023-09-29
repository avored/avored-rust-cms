# Using the `rust-musl-builder` as base image, instead of 
# the official Rust toolchain
FROM rust:1.70 AS chef
USER root
RUN cargo install cargo-chef
RUN apt-get update && apt-get install -y libclang-dev librocksdb-dev llvm
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
COPY --from=planner /app/avored_better_query avored_better_query
# Notice that we are specifying the --target flag!
RUN cargo chef cook --release --target x86_64-unknown-linux-gnu --recipe-path recipe.json
COPY . .
RUN cp .env.example .env
RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM alpine AS runtime
RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/avored-rust-cms /usr/local/bin/
USER myuser
CMD ["/usr/local/bin/avored-rust-cms"]
