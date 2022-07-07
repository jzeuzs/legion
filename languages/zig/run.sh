#!/bin/sh

printf %s"\n" "$1" > program.zig
echo "$2" > .input
zig build-exe --name program program.zig
shift 2
./program "$@" < .input
