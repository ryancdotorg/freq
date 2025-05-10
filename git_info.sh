#!/bin/bash
set -uo pipefail
trap 's=$?; echo ": Error on line "$LINENO": $BASH_COMMAND"; exit $s' ERR
IFS=$'\n\t'

if [ -f "Cargo.toml" ]; then
  PKG_VERSION="$(cargo pkgid | awk -F '[#@]' '{print $NF}')"
else
  PKG_VERSION=
fi

TAG_VERSION=$(git describe --abbrev=0 --tags 2> /dev/null || printf '')
VERSION_EXTRA=
GIT_TAGGED=$(git tag --points-at HEAD 2> /dev/null | grep . || printf '')
GIT_BRANCH=$(git rev-parse --abbrev-ref HEAD 2> /dev/null | tr -c '\n0-9A-Za-z' - || printf '')
GIT_COMMIT=$(git log -1 --format=.%h 2> /dev/null || printf '')
GIT_STATUS=$(git status --porcelain -uno 2> /dev/null | grep -q . && printf '%s' '-dirty' || printf '')

if [ -z "$VERSION_EXTRA" ]; then
  if [ -n "$GIT_BRANCH" ]; then
    GIT_INFO=$GIT_BRANCH$GIT_COMMIT$GIT_STATUS
    if [ -z "$GIT_TAGGED" ]; then
      VERSION_EXTRA=+$GIT_INFO
    elif [ -n "$GIT_STATUS" ]; then
      VERSION_EXTRA=+$GIT_INFO
    fi
  fi
fi

echo PKG_VERSION $PKG_VERSION
echo TAG_VERSION $TAG_VERSION
echo VERSION_EXTRA $VERSION_EXTRA
echo GIT_TAGGED $GIT_TAGGED
echo GIT_BRANCH $GIT_BRANCH
echo GIT_COMMIT $GIT_COMMIT
echo GIT_STATUS $GIT_STATUS


#ifeq ($(VERSION_EXTRA),)
#        ifneq ($(GIT_BRANCH),)
#                GIT_INFO := $(GIT_BRANCH)$(GIT_COMMIT)$(GIT_STATUS)
#                ifeq ($(GIT_TAGGED),)
#                        VERSION_EXTRA := +$(GIT_INFO)
#                else
#                        ifneq ($(GIT_STATUS),)
#                                VERSION_EXTRA := +$(GIT_INFO)
#                        endif
#                endif
#        endif
#endif
