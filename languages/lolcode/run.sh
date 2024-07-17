#!/bin/sh

printf %s"\n" "$1" > program.lol
echo "$2" > .input
shift 2
lci program.lol "$@" < .input
printf '%s' "$?"
