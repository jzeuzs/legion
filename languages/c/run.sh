#!/bin/sh

printf %s"\n" "$1" > program.c
gcc program.c -o program
echo "$2" > .input
shift 2
./program "$@" < .input
printf '%s' "$?"
