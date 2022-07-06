#!/bin/sh

printf %s"\n" "$1" > program.s
gcc -nostdlib -nostartfiles -nodefaultlibs -static -c -o .obj program.s
ld -o program -static .obj
echo "$2" > .input
shift 2
./program "$@" < .input
