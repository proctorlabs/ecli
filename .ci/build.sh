#!/usr/bin/env bash
set -Eeou pipefail

# General build stuff
ROOT_DIR=$(git rev-parse --show-toplevel)
RUST_TRIPLE="x86_64-unknown-linux-musl"
PACKAGE_DIR="target/${RUST_TRIPLE}/release/"
PACKAGE_FILE="${ROOT_DIR}/${PACKAGE_DIR}ecli.tar.xz"
DOCKER_IMAGE="proctorlabs/rust-builder"

build() {
    (
        cd $ROOT_DIR
        cargo build --release --target $RUST_TRIPLE
    )
}

github_publish() {
    RELEASE_TAG="${RELEASE_TAG}"
    GITHUB_TOKEN="$GITHUB_TOKEN"
    hub release create -p -m "Automated Prerelease" -a "$PACKAGE_FILE" "$RELEASE_TAG"
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
        docker pull ${DOCKER_IMAGE}
        docker run -v $PWD:/src -w /src --rm -it -u $UID:$UID -e "RELEASE_TAG=$RELEASE_TAG" -e "GITHUB_TOKEN=$GITHUB_TOKEN" ${DOCKER_IMAGE} /src/.ci/build.sh -ba
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
