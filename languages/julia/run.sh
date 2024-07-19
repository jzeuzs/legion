#!/bin/sh

printf %s"\n" "$1" > program.jl
echo "$2" > .input
shift 2
julia program.jl "$@" < .input
printf '%s' "$?"
