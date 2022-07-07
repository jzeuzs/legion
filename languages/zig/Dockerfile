FROM alpine

RUN apk add --no-cache zig --repository=http://dl-cdn.alpinelinux.org/alpine/edge/testing/
ENV XDG_CACHE_HOME=/tmp/xdg_cache
COPY run.sh /var/run
