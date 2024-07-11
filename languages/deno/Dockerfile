FROM denoland/deno:alpine

ENV DENO_DIR=/tmp/deno-dir/ \
    NO_COLOR=1

RUN apk add --no-cache util-linux

COPY run.sh /var/run/
