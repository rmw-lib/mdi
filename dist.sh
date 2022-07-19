#!/usr/bin/env bash

set -e

_DIR=$(dirname $(realpath "$0"))

cd $_DIR

git add -u
git commit -m dist
git pull

cargo set-version --bump patch

cat Cargo.toml|grep version|head -1|awk -F \" '{print $2}' > .version

mdi


git add -u
git commit -m dist
git push

cargo publish
