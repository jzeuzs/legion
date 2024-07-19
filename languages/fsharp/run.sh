#!/bin/sh

printf %s"\n" "$1" > program.fs
fsharpc --nologo --optimize- program.fs 2>/dev/null
echo "$2" > .input
shift 2
mono program.exe "$@" < .input
printf '%s' "$?"
