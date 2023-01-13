#!/usr/bin/env bash

# Use https://hub.docker.com/r/robertodr/maturin to deploy

docker run \
       --env MATURIN_PASSWORD="$MATURIN_PASSWORD" \
       --rm \
       -v "$(pwd)":/io \
       robertodr/maturin \
       publish \
       --interpreter python3.8 python3.9 python3.10 \
       --username nielstron \
       --password "$MATURIN_PASSWORD"