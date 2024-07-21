#!/bin/bash

version=$(awk -F ' = ' '$1 ~ /version/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml)

# TODO: also push a latest image
podman build --platform linux/arm64,linux/amd64 \
    -t fzamperin/cron-cloudflare:$version \
    -t fzamperin/cron-cloudflare:latest \
    --jobs=2 \
    -f Dockerfile \
    --manifest fzamperin/cron-cloudflare \
    .