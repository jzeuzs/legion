#!/bin/sh

printf %s"\n" "$1" > program.ts
echo "$2" > .input
shift 2
deno run -A program.ts "$@" < .input
