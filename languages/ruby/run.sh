#!/bin/sh

printf %s"\n" "$1" > program.rb
echo "$2" > .input
shift 2
ruby program.rb "$@" < .input
