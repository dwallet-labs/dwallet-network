# ika-node-local Docker

This directory contains Docker configuration for running `ika-node` using a pre-built binary from `target/release`, rather than building from source within the Docker container.

## Overview

The `ika-node-local` Docker setup is designed for:
- **Faster builds**: Uses pre-compiled binaries instead of building from source
- **Local development**: Quick iteration when you already have the binary built
- **Testing**: Rapid deployment of locally built binaries
- **CI/CD**: When you want to build once and deploy multiple times

## Differences from `ika-node`

| Feature | `ika-node` | `ika-node-local` |
|---------|------------|------------------|
| Build method | Builds from source in container | Copies pre-built binary |
| Build time | ~10-20 minutes | ~30 seconds |
| Binary source | Compiled in container | Host `target/release/` |
| Use case | Production builds | Local development |
| Dependencies | Full Rust toolchain | Runtime only |

## Prerequisites

1. **Build the binary first**:
   ```bash
   cargo build --release --bin ika-node
   ```

2. **Verify the binary exists**:
   ```bash
   ls -la target/release/ika-node
   ```

## Usage

### Basic Build

```bash
cd docker/ika-node-local
./build.sh
```

### Custom Docker Tag

```bash
DOCKER_TAG="my-ika-node:v1.0.0" ./build.sh
```

### Build with Additional Docker Options

```bash
./build.sh --no-cache --progress=plain
```

## Running the Container

### Basic Run
```bash
docker run --rm ika-node-local:latest --help
```

### With Configuration File
```bash
docker run --rm \
  -v /path/to/your/config:/config \
  ika-node-local:latest \
  --config-path /config/node.yaml
```

### With Network and Ports
```bash
docker run --rm \
  -p 8080:8080 \
  -p 9184:9184 \
  --network ika-network \
  ika-node-local:latest \
  --config-path /config/node.yaml
```

### Interactive Mode
```bash
docker run --rm -it \
  ika-node-local:latest \
  bash
```

## Configuration

The container expects the same configuration as the regular `ika-node`. You can:

1. **Mount configuration files**:
   ```bash
   docker run --rm \
     -v $(pwd)/config:/config \
     ika-node-local:latest \
     --config-path /config/node.yaml
   ```

2. **Use environment variables**:
   ```bash
   docker run --rm \
     -e IKA_CONFIG_PATH=/config/node.yaml \
     ika-node-local:latest
   ```

3. **Pass command-line arguments**:
   ```bash
   docker run --rm \
     ika-node-local:latest \
     --network mainnet \
     --listen-address 0.0.0.0:8080
   ```

## Development Workflow

1. **Make code changes**
2. **Rebuild the binary**:
   ```bash
   cargo build --release --bin ika-node
   ```
3. **Rebuild the Docker image**:
   ```bash
   cd docker/ika-node-local
   ./build.sh
   ```
4. **Test the container**:
   ```bash
   docker run --rm ika-node-local:latest --version
   ```

## Troubleshooting

### Binary Not Found
```
Error: ika-node binary not found at target/release/ika-node
```
**Solution**: Build the binary first:
```bash
cargo build --release --bin ika-node
```

### Permission Denied
```
docker: permission denied while trying to connect to the Docker daemon socket
```
**Solution**: Ensure Docker is running and you have permissions:
```bash
sudo usermod -aG docker $USER
# Then log out and back in
```

### Container Won't Start
```
exec: "ika-node": executable file not found in $PATH
```
**Solution**: The binary might not be executable. Rebuild the image:
```bash
./build.sh --no-cache
```

## Environment Variables

The build script supports these environment variables:

- `DOCKER_TAG`: Custom tag for the built image (default: `ika-node-local:latest`)
- `GIT_REVISION`: Git revision for labeling (auto-detected)
- `BUILD_DATE`: Build date for labeling (auto-generated)

Example:
```bash
DOCKER_TAG="ika-node-local:dev" ./build.sh
```

## Comparison with Other Setups

### When to use `ika-node-local`:
- ✅ Local development and testing
- ✅ Quick iterations on code changes
- ✅ CI/CD pipelines where you build once, deploy many times
- ✅ When you need faster Docker builds

### When to use `ika-node`:
- ✅ Production deployments
- ✅ Reproducible builds
- ✅ When you don't have local build environment
- ✅ Multi-architecture builds

## Docker Image Details

- **Base Image**: `debian:bullseye-slim`
- **Runtime Dependencies**: `libjemalloc-dev`, `ca-certificates`, `curl`, `jq`
- **Memory Allocator**: jemalloc (for better performance)
- **Working Directory**: `/opt/ika`
- **Binary Locations**: 
  - `/opt/ika/bin/ika-node`
  - `/usr/local/bin/ika-node`

## Security Considerations

- The container runs as root by default
- Consider creating a non-root user for production use
- Mount configuration files as read-only when possible
- Use specific tags instead of `latest` for production

## Performance Notes

- Uses jemalloc for better memory allocation performance
- Optimized for x86_64 architecture
- Debug symbols are stripped from the binary for smaller size
- Consider using `--memory` and `--cpus` flags to limit resource usage 