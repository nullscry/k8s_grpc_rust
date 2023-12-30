FROM rust:slim-buster AS builder

WORKDIR /smart_model

COPY Cargo.toml Cargo.lock build.rs diesel.toml setupdb_server.sh ./
COPY src src
COPY proto proto
COPY migrations migrations

RUN apt-get update && apt-get install -y libprotoc17 libprotobuf-dev protobuf-compiler libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo test

RUN cargo build --release

FROM debian:bullseye-slim AS runtime

WORKDIR /smart_model
COPY --from=builder /smart_model/target/release/db_api /usr/local/bin
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin
COPY --from=builder /smart_model/setupdb_server.sh /usr/local/bin
COPY migrations migrations

RUN apt-get update && apt-get install -y libpq-dev

ENV DATABASE_URL=postgres://postgres:trux@postgres/trux_task

RUN chmod +x /usr/local/bin/setupdb_server.sh

EXPOSE 8888

CMD ["/usr/local/bin/setupdb_server.sh"]
