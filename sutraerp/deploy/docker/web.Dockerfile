# syntax=docker/dockerfile:1.7
# ---- build (static SPA artifact, ADR-L5-0015) ----
FROM node:22-bookworm-slim AS build
WORKDIR /src
RUN corepack enable
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml .npmrc tsconfig.base.json ./
COPY apps/web/package.json apps/web/
COPY packages/ui/package.json packages/ui/
COPY packages/api-client/package.json packages/api-client/
RUN pnpm install --frozen-lockfile
COPY catalog catalog
COPY openapi openapi
COPY packages packages
COPY apps/web apps/web
ARG VITE_AUTH_MODE=api
ENV VITE_AUTH_MODE=$VITE_AUTH_MODE
RUN pnpm --filter @sutra/web build

# ---- runtime: unprivileged nginx, SPA fallback + /api reverse proxy ----
FROM nginxinc/nginx-unprivileged:1.27-alpine
COPY deploy/docker/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /src/apps/web/build/client /usr/share/nginx/html
EXPOSE 8080
