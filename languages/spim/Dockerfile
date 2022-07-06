FROM alpine as build

RUN apk add --no-cache build-base bison flex git && \
    git clone https://github.com/TryItOnline/spim.git /opt/spim && \
    cd /opt/spim && \
    make && \
    chmod -R 755 /opt/spim

COPY run.sh /var/run
