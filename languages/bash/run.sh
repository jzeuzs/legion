#!/bin/sh

printf %s"\n" "$1" > program.sh
echo "$2" > .input
shift 2
bash program.sh "$@" < .input
