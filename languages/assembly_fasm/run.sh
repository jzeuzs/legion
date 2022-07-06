#!/bin/sh

printf %s"\n" "$1" > program.s
fasm.x64 program.s program >&2
echo "$2" > .input
shift 2
./program "$@" < .input
