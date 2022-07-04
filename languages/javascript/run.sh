#!/bin/sh

printf %s"\n" "$1" > program.js
echo "$2" > .input
shift 2
node program.js "$@" < .input
