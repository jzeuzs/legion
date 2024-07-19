#!/bin/sh

printf %s"\n" "$1" > program.pl
echo "$2" > .input
shift 2
perl program.pl "$@" < .input
printf '%s' "$?"
