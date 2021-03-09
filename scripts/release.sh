#!/bin/bash
NAME=$(grep ^name Cargo.toml | sed -r s/^[^\"]*\"\([^\"]*\)\"/\\1/)
VERSION=$(grep ^version Cargo.toml | sed -r s/^[^\"]*\"\([^\"]*\)\"/\\1/)
FULL_NAME="$NAME-$VERSION-$1"
DIR="target/full-release/$VERSION/$FULL_NAME"
mkdir -p $DIR

cargo build --release --target "$@"
cp assets/ $DIR -r
cp config/ $DIR -r
cp Cargo.toml $DIR
cp README.md $DIR
cp COPYING $DIR

if [[ $1 == *"windows"* ]]; then
  cp "target/$1/release/$NAME.exe" $DIR
  cd "$DIR/.."
  zip -r "$FULL_NAME.zip" $FULL_NAME
else
  cp "target/$1/release/$NAME" $DIR
  tar -czvf "target/full-release/$VERSION/$FULL_NAME.tar.gz" -C "$DIR/.." $FULL_NAME
fi
