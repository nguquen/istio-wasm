FROM node:16-alpine
WORKDIR /app

ADD . /app

ADD ./authz_filter.wasm /app/filters/authz_filter.wasm

CMD  ["node", "index.js"]
