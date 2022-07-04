#!/bin/sh

printf %s"\n" "$1" > program.ts
echo "$2" > .input
shift 2
tsc --lib DOM,ESNext --target ES2020 --strict --skipLibCheck \
    --types /usr/local/share/.config/yarn/global/node_modules/@types/node \
    program.ts

node program.js "$@" < .input
