# Kubeincus

A bridge enabling KubeVirt to manage system-containers with Incus/LXD, using images built with distrobuilder.

## Overview

Kubeincus is a Rust-based bridge that enables KubeVirt to manage system-containers through Incus/LXD. It leverages distrobuilder for image creation and is designed for performance, safety, and reliability.

## Features

- **KubeVirt Integration**: Seamless integration with KubeVirt for container management
- **Incus/LXD Support**: Native support for Incus and LXD system containers
- **Distrobuilder Images**: Uses distrobuilder for creating container images
- **Rust Implementation**: Built in Rust for memory safety and performance
- **Pixi Management**: Reproducible development environment using pixi

## Development

This project uses [pixi](https://prefix.dev/docs/pixi) for environment management.

### Prerequisites

- Rust 1.70 or later
- pixi

### Building

```bash
# Using pixi
pixi run build

# Or using cargo directly
cargo build --release
```

### Testing

```bash
# Using pixi
pixi run test

# Or using cargo directly
cargo test
```

### Linting

```bash
# Format check
pixi run fmt

# Clippy
pixi run clippy
```

## Conda-Forge Recipe

This repository includes a staged recipe for conda-forge packaging in the `recipe/` directory. The recipe follows conda-forge best practices for Rust packages and includes:

- `meta.yaml`: Package metadata and dependencies
- `build.sh`: Build script for Linux platforms

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! This is an RFC stage project gathering initial feedback and requirements discussion.
