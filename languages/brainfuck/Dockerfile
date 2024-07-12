FROM alpine as build

COPY brainfuck.c Makefile ./
RUN apk add build-base --no-cache && \
    make brainfuck

FROM alpine

RUN apk add util-linux --no-cache

COPY --from=build brainfuck /usr/local/bin/
COPY run.sh /var/run/
