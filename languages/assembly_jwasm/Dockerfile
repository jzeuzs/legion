FROM alpine as build

RUN apk add --no-cache build-base cmake git && \
    git clone https://github.com/JWasm/JWasm.git && \
    cd JWasm && \
    mkdir build && \
    cd build && \
    cmake .. && \
    make

FROM alpine

RUN apk add --no-cache binutils
COPY --from=build /JWasm/build/jwasm /usr/local/bin
COPY run.sh /var/run
