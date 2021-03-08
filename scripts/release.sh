#!/bin/bash
NAME=$(grep ^name Cargo.toml | sed -r s/^[^\"]*\"\([^\"]*\)\"/\\1/)
VERSION=$(grep ^version Cargo.toml | sed -r s/^[^\"]*\"\([^\"]*\)\"/\\1/)
DIR="target/full-release/$VERSION/$1"
mkdir -p $DIR

cargo build --release --target "$@"
cp assets/ $DIR -r
cp config/ $DIR -r
cp Cargo.toml $DIR
cp README.md $DIR
cp COPYING $DIR

if [[ $1 == *"windows"* ]]; then
  cp "target/$1/release/$NAME.exe" $DIR
  zip -r "target/full-release/$VERSION/$NAME-$VERSION-$1.zip" $DIR
else
  cp "target/$1/release/$NAME" $DIR
  tar -czvf "target/full-release/$VERSION/$NAME-$VERSION-$1.tar.gz" $DIR
fi
