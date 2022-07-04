FROM alpine:latest as build 

RUN apk add build-base git cmake --no-cache && \
    git clone https://github.com/justinmeza/lci.git && \
    cd lci && \
    mkdir build && \
    cd build && \
    cmake .. && \
    make && \
    make install

FROM alpine:latest

COPY --from=build /usr/local/bin/lci /usr/local/bin/
COPY run.sh /var/run/
