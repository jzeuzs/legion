FROM python:2-alpine

RUN apk add git util-linux --no-cache && \
    git clone https://github.com/programble/befungee.git /opt/befungee

COPY run.sh /var/run/
