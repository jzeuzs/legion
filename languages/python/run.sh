#!/bin/sh

printf %s"\n" "$1" > program.py
echo "$2" > .input
shift 2
python program.py "$@" < .input
printf '%s' "$?"
