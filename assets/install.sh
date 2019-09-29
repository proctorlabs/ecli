#!/usr/bin/env sh
set -Eeou pipefail

install_ecli() {
    (
        cd /usr/local/bin
        curl -L -s 'https://github.com/proctorlabs/ecli/releases/download/v0.1.0/ecli-0.1.0-amd64-musl.tar.xz' | sudo tar --xz -x
    )
}

install_ecli
