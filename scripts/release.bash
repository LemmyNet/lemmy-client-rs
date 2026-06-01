#!/bin/sh
set -e

# Creating the new tag
new_tag="$1"

# Goto the upper route
CWD="$(cd -P -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
cd "$CWD/../"

# Update crate versions
old_tag=$(grep version Cargo.toml | head -1 | cut -d'"' -f 2)
sed -i "s/{ version = \"=$old_tag\", path/{ version = \"=$new_tag\", path/g" Cargo.toml
sed -i "s/version = \"$old_tag\"/version = \"$new_tag\"/g" Cargo.toml

# Run check to ensure translations are valid and lockfile is updated
cargo check

# The commit
# git add Cargo.toml Cargo.lock
# git commit -m"Version $new_tag"
# git tag $new_tag

# Push
# git push origin $new_tag
# git push
