#!/bin/sh
# Run GitHub CLI as Olmer1st without changing the globally active account.
token=$(gh auth token --hostname github.com --user Olmer1st) || exit 1
GH_TOKEN="$token" exec gh "$@"
