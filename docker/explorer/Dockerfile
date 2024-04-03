FROM node:20-slim AS build
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
WORKDIR /app

COPY /sdk /app/sdk
COPY /apps /app/apps
COPY /dapps /app/dapps
COPY Cargo.toml Cargo.lock /app/
COPY package.json /app/package.json
COPY pnpm-lock.yaml /app/pnpm-lock.yaml
COPY pnpm-workspace.yaml /app/pnpm-workspace.yaml
COPY .eslintrc.js .npmrc turbo.json /app/

# --mount=[...] will ensure a persistent cache volume is mounted at /pnpm_store.
# This is like running docker run -v /tmp/cache:/pnpm_store ... except that mount is
# only available at build time. Our resulting Docker image won't have this mount available after build.
RUN --mount=type=cache,id=pnmcache,target=/pnpm_store \
  # pnpm config set store-dir /pnpm_store instructs pnpm to use /pnpm_store directory (mounted as cache volume)
  # to save downloaded packages and re-use existing one if possible.
  pnpm config set store-dir /pnpm_store && \
  # `pnpm config set package-import-method copy` instructs pnpm to copy packages directly in `node_modules/`
  # instead of symlinking to store. Otherwise pnpm would create symlinks from `node_modules` to
  # `/pnpm_store` which wouldn't exist after build time.
  # By copying we ensure that packages are effectively saved in resulting Docker image.
  pnpm config set package-import-method copy && \
  pnpm install --prefer-offline --ignore-scripts --frozen-lockfile
RUN pnpm explorer build

FROM nginx:alpine as release
COPY --from=build /app/apps/explorer/build /usr/share/nginx/html
