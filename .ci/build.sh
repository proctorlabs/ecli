#!/usr/bin/env bash
set -Eeou pipefail

# General build stuff
ROOT_DIR=$(git rev-parse --show-toplevel)
RUST_TRIPLE="x86_64-unknown-linux-musl"
PACKAGE_DIR="target/${RUST_TRIPLE}/release/"
PACKAGE_FILE="${ROOT_DIR}/${PACKAGE_DIR}ecli.tar.xz"
HUB_VERSION="2.12.3"

build() {
    (
        cd $ROOT_DIR
        rustup target add $RUST_TRIPLE
        cargo build --release --target $RUST_TRIPLE
    )
}

github_publish() {
    RELEASE_TAG="${RELEASE_TAG}"
    GITHUB_TOKEN="$GITHUB_TOKEN"
    curl -L "https://github.com/github/hub/releases/download/v${HUB_VERSION}/hub-linux-amd64-${HUB_VERSION}.tgz" | tar xvz --strip=2 --wildcards *bin/hub
    ./hub release create -p -m "Automated Prerelease" -a "$PACKAGE_FILE" "$RELEASE_TAG"
}

create_archive() {
    (
        cd $ROOT_DIR/$PACKAGE_DIR
        strip ecli
        tar -cvJf ecli.tar.xz ecli
    )
}

build_in_docker() {
    (
        cd $ROOT_DIR
        docker pull rust:1.36-buster
        docker run -v $PWD:/app -w /app --rm -it -u $UID:$UID -e "RELEASE_TAG=$RELEASE_TAG" -e "GITHUB_TOKEN=$GITHUB_TOKEN" rust:1.36-buster /app/.ci/build.sh -ba
    )
}

while getopts "badg" opt; do
    case ${opt} in
    d)
        build_in_docker
        exit 0
        ;;
    b)
        build
        ;;
    a)
        create_archive
        ;;
    g)
        github_publish
        ;;
    *)
        echo "Invalid parameters specified" 1>&2
        exit 1
        ;;
    esac
done

exit 0
