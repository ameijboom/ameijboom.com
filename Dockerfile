FROM oven/bun:1.1.28 AS build
WORKDIR /build

COPY package.json bun.lockb . /build/

RUN bun install && bun run build

FROM oven/bun:1.1.28-distroless
WORKDIR /app

COPY --from=build /build/.output /app

CMD ["./server/index.mjs"]