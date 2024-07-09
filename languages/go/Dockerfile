FROM golang:alpine

ENV CGO_ENABLED=0
RUN apk add --no-cache libc6-compat
COPY run.sh /var/run
