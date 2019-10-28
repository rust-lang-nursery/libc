#!/usr/bin/env sh

# Small script to run tests for a target (or all targets) inside all the
# respective docker images.

set -ex

echo "${HOME}"
pwd

run() {
    echo "Building docker container for target ${1}"

    # use -f so we can use ci/ as build context
    docker build -t libc -f "ci/docker/${1}/Dockerfile" ci/
    mkdir -p target
    if [ -w /dev/kvm ]; then
        kvm="--volume /dev/kvm:/dev/kvm"
    else
        kvm=""
    fi

    # Emscripten targets require that the user is the same for building and
    # running the image
    if [[ "${1}" == *"emscripten" ]]; then
      user="0"
    else
      user="$(id -u):$(id -g)"
    fi

    docker run \
      --rm \
      --user "$user" \
      --env LIBC_CI \
      --env CARGO_HOME=/cargo \
      --env CARGO_TARGET_DIR=/checkout/target \
      --volume "$(dirname "$(dirname "$(command -v cargo)")")":/cargo \
      --volume "$(rustc --print sysroot)":/rust:ro \
      --volume "$(pwd)":/checkout:ro \
      --volume "$(pwd)"/target:/checkout/target \
      $kvm \
      --init \
      --workdir /checkout \
      libc \
      sh -c "HOME=/tmp PATH=\$PATH:/rust/bin exec ci/run.sh ${1}"
}

if [ -z "${1}" ]; then
  for d in ci/docker/*; do
    run "${d}"
  done
else
  run "${1}"
fi
