#!/bin/sh

printf %s"\n" "$1" > program.s
jwasm -elf64 -Fo .obj program.s >&2
ld -o program .obj
echo "$2" > .input
shift 2
./program "$@" < .input
