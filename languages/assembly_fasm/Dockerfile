FROM alpine as build

RUN apk add --no-cache curl && \
    curl -sL "https://flatassembler.net/fasm-1.73.30.tgz" | tar xz && \
    ln -s /fasm/fasm.x64 /usr/local/bin/fasm.x64

FROM alpine

COPY --from=build /fasm/fasm.x64 /usr/local/bin
COPY run.sh /var/run
