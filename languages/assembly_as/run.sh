#!/bin/sh

printf %s"\n" "$1" > program.s
as program.s -o .obj
ld -o program .obj
echo "$2" > .input
shift 2
./program "$@" < .input
