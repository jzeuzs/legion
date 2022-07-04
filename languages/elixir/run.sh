#!/bin/sh

printf %s"\n" "$1" > program.exs
echo "$2" > .input
shift 2
elixir program.exs "$@" < .input
