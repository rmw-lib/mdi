#!/usr/bin/env bash
DIR=$(dirname $(realpath "$0"))
cd $DIR/..
cargo update
cargo upgrade
