FROM node:alpine

RUN yarn global add typescript @types/node && \
	apk add --no-cache util-linux

COPY run.sh /var/run/
