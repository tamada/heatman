#! /bin/sh

VERSION=$1

if [[ "$VERSION" == "" ]]; then
    echo "no version specified"
    exit 1
fi

result=0

PREV_VERSION=$(grep "^version = " Cargo.toml | sed -e 's/version = "\(.*\)"/\1/g')
if [[ "$PREV_VERSION" != "" && $VERSION == $PREV_VERSION ]]; then
    echo "already updated to $VERSION"
    exit 1
fi

echo "$PREV_VERSION -> $VERSION  (${PREV_VERSION//-/--} -> ${VERSION//-/--})"

for i in README.md docs/content/_index.md ; do 
    sed -e "s#Version-v${PREV_VERSION//-/--}-information#Version-v${VERSION//-/--}-information#g" \
        -e "s#tag/v${PREV_VERSION}#tag/v${VERSION}#g" \
        -e "s#heatman:${PREV_VERSION}#heatman:${VERSION}#g" \
        -e "s#crates.io-v${PREV_VERSION}#crates.io-v${VERSION}#g" \
        $i > a; mv a $i
done
# sed -e "s#version = \".*\"#version = \"${VERSION}\"#g" docs/config.toml > a ; mv a docs/config.toml
sed "s/^version = \".*\"/version = \"${VERSION}\"/g" Cargo.toml > a && mv a Cargo.toml

echo "Replace version from \"${PREV_VERSION}\" to \"${VERSION}\""
