#!/bin/sh

printf %s"\n" "$1" > program.hs
echo "$2" > .input
shift 2
runghc -- -funfolding-use-threshold=16 -optc-O3 program.hs "$@" < .input
printf '%s' "$?"
