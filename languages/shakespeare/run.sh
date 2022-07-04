#!/bin/sh

printf %s"\n" "$1" > program.spl
echo "$2" > .input
shift 2
shakespeare run program.spl "$@" < .input
