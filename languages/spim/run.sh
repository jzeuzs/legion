#!/bin/sh

printf %s"\n" "$1" > program.s
echo "$2" > .input
shift 2
/opt/spim/spim -file program.s "$@" < .input
printf '%s' "$?"
