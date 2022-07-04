#!/bin/sh

printf %s"\n" "$1" > program.cc
g++ program.cc -o program
echo "$2" > .input
shift 2
./program "$@" < .input
