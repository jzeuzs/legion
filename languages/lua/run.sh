#!/bin/sh

printf %s"\n" "$1" > program.lua
echo "$2" > .input
shift 2
lua5.4 program.lua "$@" < .input
printf '%s' "$?"
