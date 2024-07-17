#!/bin/sh

printf %s"\n" "$1" > program.cs
csc -nologo program.cs 2>/dev/null
echo "$2" > .input
shift
mono program.exe "$@" < .input
printf '%s' "$?"
