#!/bin/bash

if ! hash ghp-import 2>/dev/null; then
    echo "This script requires ghp-import from pip"
    exit 1
fi
if ! hash cargo 2>/dev/null; then
    echo "This script requires cargo"
    exit 1
fi

# Generate docs, add redirect, use ghp-import to write gh-pages branch, and push.
echo "Generating docs, prepare for push"
cargo doc \
    && echo '<meta http-equiv=refresh content=0;url=dns_lookup/index.html>' > target/doc/index.html && \
    ghp-import -np target/doc
