#!/usr/bin/env sh
set -Eeou pipefail

install_ecli() {
    (
        cd /usr/local/bin
        curl -L -s 'https://github.com/proctorlabs/ecli/releases/download/v0.1-alpha1/ecli.tar.xz' | sudo tar --xz -x
    )
}

install_ecli
