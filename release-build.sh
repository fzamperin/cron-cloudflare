#!/bin/bash

version=$(awk -F ' = ' '$1 ~ /version/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml)

# TODO: also push a latest image
podman build --platform linux/amd64,linux/arm64 \
    -f Dockerfile \
    --manifest docker.io/fzamperin/cron-cloudflare:$version \
    .