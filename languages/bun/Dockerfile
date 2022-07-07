FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y curl unzip && \
    curl -fsSL https://bun.sh/install | BUN_INSTALL=/opt/.bun bash

ENV PATH=/opt/.bun/bin:${PATH}
COPY run.sh /var/run
