#!/bin/bash

docker buildx build --platform linux/arm/v7,linux/arm64,linux/amd64 \
    -t fzamperin/cron-cloudflare:latest \
    -t fzamperin/cron-cloudflare:$(awk -F ' = ' '$1 ~ /version/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml) \
    --push \
    -f Dockerfile \
    .