FROM rust:1.66-bullseye as builder

ARG TARGETARCH
ARG PKG_VERSION

WORKDIR /usr/src/cron-cloudflare

COPY . .

RUN cargo install --path --locked .

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y libssl1.1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/cron-cloudflare /usr/local/bin/cron-cloudflare

ENV CONFIG_FILE_PATH=/config/config.yaml

VOLUME [ "/config" ]

CMD [ "cron-cloudflare" ]