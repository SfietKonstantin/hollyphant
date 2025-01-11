#!/bin/bash
#Similar to https://github.com/CODeRUS/github-sfos-build/blob/master/build.sh

set -ex
export SFOS_ARCH=$1

docker run --rm --privileged \
  -v $GITHUB_WORKSPACE:/workspace \
  -e SFOS_RELEASE \
  -e SFOS_ARCH \
  coderus/sailfishos-platform-sdk:$SFOS_RELEASE \
  "/bin/bash" "/workspace/.github/workflows/build-sfos-package-docker.sh"

