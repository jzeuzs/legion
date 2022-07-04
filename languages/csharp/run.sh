#!/bin/sh

printf %s"\n" "$1" > program.csx
echo "$2" > .input
shift 2
DOTNET_CLI_HOME="/opt/dotnet" dotnet script program.csx "$@" < .input
