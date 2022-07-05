#!/bin/sh

printf %s"\n" "$1" > program.sql
echo "$2" > .input
shift 2
sqlite3 -init program.sql "$@" < .input
