#!/bin/sh

printf %s"\n" "$1" > program.csx
echo "$2" > .input
shift
dotnet script program.csx < .input
