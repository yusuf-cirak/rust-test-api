FROM rust:bookworm as builder

WORKDIR /app

RUN apt-get update
RUN apt-get install -y pkg-config openssl libssl-dev curl

COPY . .

RUN cargo build --release -vv

FROM debian:bookworm-slim as runtime

RUN apt-get update && apt-get install -y openssl 

WORKDIR /app

COPY --from=builder /app/target/release/rust_test_api .
COPY --from=builder /app/.env .

CMD [ "/app/rust_test_api" ]
