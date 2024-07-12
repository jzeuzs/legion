FROM python:3.9-alpine

RUN python -m pip install shakespearelang && \
	apk add --no-cache util-linux

COPY run.sh /var/run
