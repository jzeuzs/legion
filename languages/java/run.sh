#!/bin/sh

printf %s"\n" "$1" > Main.java
echo "$2" > .input
javac Main.java
shift 2
java Main "$@" < .input
printf '%s' "$?"
