FROM node:20.7.0-bookworm

RUN apt-get update && apt-get install -y curl && \
    apt-get clean autoclean && rm -rf /var/lib/apt/lists/*

WORKDIR /home/app

RUN npm i -g pnpm

# Files required by pnpm install
COPY package.json pnpm-lock.yaml ./

RUN pnpm fetch

ADD . ./

ENV VITE_API_BASE_URL=http://backend:9090

RUN pnpm install -r --frozen-lockfile && \
    pnpm build && \
    rm -rf node_modules && \
    pnpm install --frozen-lockfile --prod

EXPOSE 3000

CMD [ "node", "server/entry.polka.mjs" ]
