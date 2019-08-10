#!/usr/bin/env sh
docker pull rust:1.36-buster
docker run -v $PWD:/app -w /app --rm -it rust:1.36-buster #/app/.ci/build.sh
