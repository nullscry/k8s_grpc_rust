FROM rust:alpine3.17 AS builder

WORKDIR /smart_model

COPY Cargo.toml Cargo.lock build.rs ./
COPY src src
COPY proto proto

ENV CARGO_UNSTABLE_SPARSE_REGISTRY=true

RUN apk add --no-cache protoc protobuf-dev musl-dev

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo test

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.19.0 AS runtime

WORKDIR /smart_model
COPY --from=builder /smart_model/target/x86_64-unknown-linux-musl/release/numbers /usr/local/bin

EXPOSE 8888

CMD ["/usr/local/bin/numbers"]
