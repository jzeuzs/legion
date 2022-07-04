#!/bin/sh

printf %s"\n" "$1" > program.csx
echo "$2" > .input
shift 2
DOTNET_CLI_HOME="/tmp/DOTNET_CLI_HOME" dotnet script program.csx "$@" < .input
