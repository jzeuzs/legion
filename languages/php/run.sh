#!/bin/sh

printf %s"\n" "$1" > program.php
echo "$2" > .input
shift 2
php program.php "$@" < .input
printf '%s' "$?"
