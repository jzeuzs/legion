#!/bin/sh

printf %s"\n" "$1" > program.erl
echo "$2" > .input
shift 2
escript program.erl "$@" < .input
