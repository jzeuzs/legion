#!/bin/sh

export GOCACHE=/tmp/go-cache
printf %s"\n" "$1" > program.go
echo "$2" > .input
shift 2
CGO_ENABLED=0 GOOS=linux go build program.go -o ./program
./program "$@" < .input
