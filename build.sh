#!/usr/bin/env bash
THIS_DIR="$(cd "$(dirname "${BASH_SOURCE}")"; pwd)"

INDEX_FILE_PATH="${THIS_DIR}/src/index.ts"
DIST_DIR="${THIS_DIR}/dist"

mkdir -p "${DIST_DIR}"
deno compile \
     --unstable \
     --allow-net \
     --allow-env \
     --target "x86_64-apple-darwin" \
     --output "${DIST_DIR}/update-issue" \
     "${INDEX_FILE_PATH}"
