
FROM rust:1.70 as builder

WORKDIR /usr/src/orion-server

COPY Cargo.toml Cargo.lock ./

RUN echo "fn main() {}" > src/main.rs

RUN cargo fetch && cargo build --release

COPY . .

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/src/orion-server

COPY --from=builder /usr/src/orion-server/target/release/orion-server .

EXPOSE 3030

CMD ["./orion-server"]