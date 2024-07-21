FROM rust:1.79-bullseye AS builder

WORKDIR /usr/src/cron-cloudflare

COPY . .

RUN cargo build --release --locked

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y libssl1.1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/cron-cloudflare/target/release/cron-cloudflare /usr/local/bin/cron-cloudflare

ENV CONFIG_FILE_PATH=/config/config.yaml

VOLUME [ "/config" ]

CMD [ "cron-cloudflare" ]