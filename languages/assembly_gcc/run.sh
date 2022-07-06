#!/bin/sh

printf %s"\n" "$1" > program.s
gcc -no-pie -o program program.s
echo "$2" > .input
shift 2
./program "$@" < .input
