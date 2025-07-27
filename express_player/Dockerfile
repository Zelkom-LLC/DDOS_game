FROM oven/bun:1.2.19-debian

WORKDIR /app
COPY index.ts .
COPY strategy.ts .
COPY package.json .
COPY bun.lock .
COPY tsconfig.json .

RUN bun install

CMD [ "bun", "run", "dev" ]