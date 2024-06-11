FROM rust:1.78.0 as build

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
COPY ./migrations ./migrations

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=build ./target/release/todo-list .

CMD ["./todo-list"]