FROM node:20-slim AS build
ENV PNPM_HOME="/pnpm"
# Initially set PATH so corepack is available
ENV PATH="$PNPM_HOME:$PATH"

# Enable corepack and lock pnpm version
RUN corepack enable && corepack prepare pnpm@9.9.0 --activate

WORKDIR /app

COPY . /app

RUN apt-get update && apt-get install -y curl build-essential

# Install Rust non-interactively using rustup
# Install Rust non-interactively using rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
#    rustup set default-host x86_64-unknown-linux-gnu && \
# Update PATH to include Cargo binaries
ENV PATH="/root/.cargo/bin:$PATH"


RUN  rm -rf /root/.rustup/downloads/* && \
    rustup update stable && \
    rustup component add cargo


## Explicitly update the Rust stable toolchain. This helps ensure that all components (like Cargo)
## are properly downloaded and installed before they are used.
#RUN rustup update stable

# Install wasm-pack using its installer.
# Create a symlink to /usr/local/bin so it's found by subsequent commands.
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh && \
    ln -s /root/.cargo/bin/wasm-pack /usr/local/bin/wasm-pack

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

# Run your build command; wasm-pack should now be available.
RUN pnpm explorer build

FROM nginx:alpine as release
COPY --from=build /app/apps/explorer/build /usr/share/nginx/html
