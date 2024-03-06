FROM node:20-slim AS build
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
RUN mkdir /app

COPY ./.eslintrc.js /app/.eslintrc.js
COPY ./.npmrc /app/.npmrc
COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock

COPY ./package.json /app/package.json
COPY ./pnpm-lock.yaml /app/pnpm-lock.yaml
COPY ./pnpm-workspace.yaml /app/pnpm-workspace.yaml
COPY ./turbo.json /app/turbo.json

COPY ./sdk /app/sdk
COPY ./apps /app/apps
COPY ./dapps /app/dapps

WORKDIR /app

RUN pnpm install
RUN pnpm explorer build

FROM nginx:alpine
COPY --from=build /app/apps/explorer/build /usr/share/nginx/html