FROM node:18.9.0-bullseye

RUN apt-get update && apt-get install -y curl

WORKDIR /home/app

RUN npm i -g pnpm

# Files required by pnpm install
COPY pnpm-lock.yaml ./

RUN pnpm fetch

ADD . ./

RUN pnpm install -r --offline

RUN pnpm build

EXPOSE 3000
CMD [ "node", "server/entry.express.mjs" ]
