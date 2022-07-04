#!/bin/sh

printf %s"\n" "$1" > program.rs
echo "$2" > .input
rustc -C opt-level=0 --color never program.rs
shift 2
./program "$@" < .input
