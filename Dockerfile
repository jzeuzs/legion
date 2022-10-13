FROM rust as builder

# This is a dummy build to get the dependencies cached.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "// meow" > src/lib.rs && \
    cargo build --release && \
    rm -r src

# This is the actual build, copy in the rest of the sources.
COPY . .
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && \
	apt-get upgrade -y && \ 
	apt-get install -y ca-certificates && \
	rm -rf /var/lib/apt/lists/*

COPY --from=builder /target/release/legion /usr/local/bin/legion
COPY Cargo.lock /

CMD ["/usr/local/bin/legion"]
