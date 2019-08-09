#!/usr/bin/env sh

docker build -t builder .
docker run -v $PWD:/app -w /app --rm -it builder cargo build
