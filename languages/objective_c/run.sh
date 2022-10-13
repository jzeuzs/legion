#!/bin/sh

printf %s"\n" "$1" > program.m
gcc program.m -o program
echo "$2" > .input
shift 2
./program "$@" < .input
