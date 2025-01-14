FROM node:20-slim AS base

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

FROM base AS build

WORKDIR /build

COPY package.json pnpm-lock.yaml /build/
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
COPY . /build/
RUN pnpm run build

FROM denoland/deno:2.1.5
WORKDIR /app

COPY --from=build /build/.next/standalone /app
COPY --from=build /build/.next/static /app/.next/static

CMD ["run", "-A", "/app/server.js"]
