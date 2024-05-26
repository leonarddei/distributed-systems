FROM rust:1.78.0 as build

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src

RUN cargo build --release

FROM rust:1.78.0

COPY --from=build ./target/release/todo-list .

CMD ["./todo-list"]