# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is the Rust gRPC client SDK for Senzing, providing a Rust language interface that communicates with a [Senzing gRPC server](https://github.com/senzing-garage/servegrpc). It implements the `sz-sdk-rust` trait interfaces over gRPC.

**Status**: Work-in-progress (0.n.x semantic versions) - not production ready.

## Build Commands

```bash
# Build the project
cargo build

# Run tests (requires gRPC server running)
cargo test

# Run tests with output visible
cargo test -- --nocapture

# Run a single test
cargo test test_name -- --nocapture

# Update dependencies
cargo update

# Lint (runs cspell)
make lint
```

## Test Environment Setup

Tests require a running Senzing gRPC server on `localhost:8261`:

```bash
# Start the gRPC server (Docker)
make setup

# Clean up (stops the server)
make clean
```

Or manually:
```bash
docker run --detach --env SENZING_TOOLS_ENABLE_ALL=true \
  --name senzing-serve-grpc --publish 8261:8261 --rm senzing/serve-grpc
```

## Architecture

### gRPC Code Generation

Proto files in `proto/` are compiled at build time via `build.rs` using `tonic-prost-build`. Generated code is included in modules with `tonic::include_proto!("package_name")`.

Proto files define services for: `szconfig`, `szconfigmanager`, `szdiagnostic`, `szengine`, `szproduct`.

### Module Structure

Each Senzing service follows the same pattern (e.g., `szproduct/mod.rs`):
- Inner module named `{service}_y` (e.g., `szproduct_y`) contains the implementation
- Includes proto-generated code via `tonic::include_proto!`
- Wraps a tonic gRPC client (`SzProductClient<Channel>`)
- Uses a global tokio runtime (`OnceLock<Runtime>`) for blocking on async gRPC calls
- Methods return `Result<T, Box<dyn std::error::Error>>`

### Abstract Factory Pattern

`SzAbstractFactory` in `szabstractfactory/mod.rs` uses type-state pattern with `Uninitialized`/`Initialized` marker types:
- `SzAbstractFactory::new(grpc_url)` returns `SzAbstractFactory<Initialized>`
- Factory methods (`create_product()`, `create_diagnostic()`) create service clients

### Traits

`src/traits.rs` defines trait interfaces (`SzProduct`, `SzDiagnostic`, `SzAbstractFactory`) matching the Senzing SDK API, enabling different implementations (gRPC, core, mock).

### Async/Sync Bridge

The SDK provides a synchronous API that wraps async tonic gRPC calls using `rt.block_on()` with a shared global tokio runtime per module.
