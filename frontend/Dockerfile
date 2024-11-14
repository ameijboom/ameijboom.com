FROM oven/bun:1.1.28 AS build
WORKDIR /build

COPY package.json bun.lockb /build/

RUN bun install

COPY . /build/

ARG NEXT_PUBLIC_MUSIC_API_URL
ENV NEXT_PUBLIC_MUSIC_API_URL=$NEXT_PUBLIC_MUSIC_API_URL

RUN bun run build

FROM oven/bun:1.1.28-distroless
WORKDIR /app

COPY --from=build /build/.next/standalone /app
COPY --from=build /build/.next/static /app/.next/static

CMD ["run", "/app/server.js"]