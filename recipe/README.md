# Conda-Forge Staged Recipe for Kubeincus

This directory contains the staged recipe for packaging Kubeincus for conda-forge.

## Overview

This recipe follows conda-forge best practices for packaging Rust applications managed with pixi.

## Files

- `meta.yaml`: Package metadata, dependencies, and build configuration
- `build.sh`: Build script for Linux platforms

## Building Locally

To test the recipe locally, you can use `conda-build`:

```bash
conda build recipe/
```

## Submitting to conda-forge

This recipe is intended to be submitted to the [conda-forge/staged-recipes](https://github.com/conda-forge/staged-recipes) repository via a pull request.

### Steps to Submit

1. Fork the `conda-forge/staged-recipes` repository
2. Copy the contents of this `recipe/` directory to `staged-recipes/recipes/kubeincus/`
3. Update the `sha256` hash in `meta.yaml` with the actual hash from the release tarball
4. Submit a pull request

## Dependencies

### Build Dependencies

- Rust compiler (via `{{ compiler('rust') }}`)
- C compiler (via `{{ compiler('c') }}`)
- `cargo-bundle-licenses`: For generating third-party license information
- `pkg-config`: For finding system libraries

### Host Dependencies

- `openssl`: Required for network operations

### Runtime Dependencies

- `openssl`: Required at runtime

## Platform Support

Currently, this recipe only supports Linux platforms (`linux-64` and `linux-aarch64`). This is intentional as Incus/LXD are primarily Linux technologies.

## Testing

The recipe includes basic tests to verify:
- The binary is accessible and executable
- `--help` and `--version` flags work correctly

## License

The package is licensed under Apache-2.0. The build process includes bundling third-party licenses using `cargo-bundle-licenses`.

## Maintainers

- babeloff

## Notes for Reviewers

- This is a new package submission (version 0.1.0)
- The project uses Rust and is managed with pixi for reproducible environments
- The implementation bridges KubeVirt with Incus/LXD for system-container management
- This is an RFC-stage project gathering initial feedback
