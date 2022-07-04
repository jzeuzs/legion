#!/bin/sh

printf %s"\n" "$1" > program.bf
echo "$2" > .input
shift 2
/opt/befungee/befungee.py program.bf "$@" < .input
