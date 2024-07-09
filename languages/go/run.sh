#!/bin/sh

export GOCACHE=/tmp/go-cache
printf %s"\n" "$1" > program.go
echo "$2" > .input
shift 2
go build program.go
./program "$@" < .input
