# ImageCombiner

A Rust CLI tool (for now) that combines two images by alternating pixels. Built as a portfolio project for learning systems programming with Rust.

## Features

- Combines two images using pixel alternation
- Automatic image format validation
- Professional CLI with `clap`
- Structured logging with `log` + `env_logger`
- Unit tests and integration tests
- Production-ready project structure

## Installation

```bash
# Clone the repository
git clone <repo-url>
cd ImageCombiner

# Build
cargo build
```

## Usage

```bash
cargo run -- --image-1 images/image1.png --image-2 images/image2.png -o output.png
```

### Command Line Options

| Option | Description |
|--------|------------|
| `--image-1` | Path to first input image |
| `--image-2` | Path to second input image |
| `-o, --output` | Path to output image |

### Help

```bash
cargo run -- --help
```

## Development

### Run Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --test unit_test

# Integration tests only
cargo test --test integration_test
```

### Linting

```bash
# Run clippy
cargo clippy

# Format code
cargo fmt
```

### Build

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## Tech Stack

- **Rust** - Systems programming language
- **clap** - CLI argument parsing
- **log** + **env_logger** - Logging
- **image** - Image processing
- **criterion** - Benchmarking (future)
