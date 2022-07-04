#!/bin/sh

printf %s"\n" "$1" > program.s
nasm program.s -f elf64 -o .obj
ld -o program .obj
echo "$2" > .input
shift 2
./program "$@" < .input
