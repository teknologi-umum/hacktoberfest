FROM node:18.9.0-bullseye

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
