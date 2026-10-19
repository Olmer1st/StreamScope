#!/bin/sh
# Repository-local Git helper. Never stores or logs credentials.
case "$1" in
  get)
    host=
    protocol=
    while IFS= read -r line && [ -n "$line" ]; do
      case "$line" in
        host=*) host=${line#host=} ;;
        protocol=*) protocol=${line#protocol=} ;;
      esac
    done
    [ "$host" = github.com ] && [ "$protocol" = https ] || exit 0
    token=$(gh auth token --hostname github.com --user Olmer1st) || exit 1
    printf 'username=Olmer1st\npassword=%s\n\n' "$token"
    ;;
  store|erase) : ;;
esac
