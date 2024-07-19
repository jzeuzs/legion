#!/bin/sh

printf %s"\n" "$1" > program.js
echo "$2" > .input
shift 2
bun run program.js "$@" < .input
printf '%s' "$?"
