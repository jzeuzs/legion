#!/bin/sh

printf %s"\n" "$1" > program.cr
crystal build program.cr
echo "$2" > .input
shift 2
./program "$@" < .input
