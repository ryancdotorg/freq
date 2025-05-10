#!/bin/bash
set -uo pipefail
trap 's=$?; echo ": Error on line "$LINENO": $BASH_COMMAND"; exit $s' ERR
IFS=$'\n\t'

CROSS_ROOT="/opt/x-tools"
PKG_VERSION="$(cargo pkgid | awk -F '[#@]' '{print $NF}')"
BINARIES="freq"
PROFILE="lto"

GIT_IS_CLEAN=$(git status --porcelain -uno 2> /dev/null | grep -q . && printf 'no' || printf 'yes')
#GIT_IS_TAGGED=$(git rev-parse -q --verify "^v$PKG_VERSION" > /dev/null && printf 'yes' || printf 'no')
GIT_TAG=$(git describe --exact-match --tags 2>/dev/null || true)

mkdir -p dist
for target in aarch64-unknown-linux-musl armv7-unknown-linux-musleabihf x86_64-unknown-linux-musl
do
  PATH="${CROSS_ROOT}/${target}/bin:${PATH}" \
  RUSTFLAGS="-C linker=${CROSS_ROOT}/${target}/bin/${target}-ld" \
    cargo build --features decompress,regex --profile="${PROFILE}" --target="${target}"
  # if we're building from a clean tagged tree, generate release binaries
  if [ "$GIT_IS_CLEAN" = "yes" ] && [ "$GIT_TAG" = "v$PKG_VERSION" ]; then
    for bin in $BINARIES
    do
      arch="${target%%-*}"
      if [ ! -f "dist/${bin}-v${PKG_VERSION}-${arch}.tar.xz" ]; then
        tar \
          -C "target/${target}/${PROFILE}" \
          -cJf "dist/${bin}-v${PKG_VERSION}-${arch}.tar.xz" \
          --owner=root --group=root \
          "${bin}"
      fi
      if [ ! -f "dist/${bin}-v${PKG_VERSION}-${arch}.upx" ]; then
        upx \
          --ultra-brute \
          -o "dist/${bin}-v${PKG_VERSION}-${arch}.upx" \
          "target/${target}/${PROFILE}/${bin}"
      fi
    done
  fi
done
