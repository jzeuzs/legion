FROM alpine

RUN apk add --no-cache build-base cmake git && \
    git clone https://github.com/JWasm/JWasm.git && \
    cd JWasm && \
    mkdir build && \
    cd build && \
    cmake .. && \
    make && \
    ln -s /JWasm/build/jwasm /usr/bin

COPY run.sh /var/run
