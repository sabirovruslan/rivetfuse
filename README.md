# RivetFuse

A high-performance LLM gateway built with Rust, designed to provide a unified interface for Large Language Model interactions.

## Overview

RivetFuse is an early-stage project that aims to create a robust gateway service for managing and routing requests to various LLM providers. The project is structured as a Rust workspace with multiple crates for modularity and maintainability.

## Features

- **HTTP Gateway**: Built with Axum for high-performance async HTTP handling
- **Configuration Management**: Flexible configuration system with environment-based profiles
- **Structured Logging**: Comprehensive logging with tracing and structured output
- **Tokenizer Support**: Built-in tokenization capabilities for LLM interactions
- **Modular Architecture**: Clean separation of concerns across multiple crates

## Getting Started

### Prerequisites

- Rust 1.70+ (2024 edition)
- Cargo

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd rivetfuse
```

2. Build the project:
```bash
cargo build
```

3. Run the gateway:
```bash
cargo run --bin gateway
```

### Configuration

The application uses a layered configuration system:

1. Base configuration from `settings/base.toml`
2. Environment-specific overrides (dev/test/prod)
3. Environment variables with `APP_` prefix

Copy the example configuration files:
```bash
cp settings/base.toml.example settings/base.toml
cp settings/dev.toml.example settings/dev.toml
```

## Development

This project is in early development. The current focus is on establishing the core architecture and basic functionality.

### Running in Development

```bash
# Set development profile
export APP_PROFILE=dev

# Run the gateway
cargo run --bin gateway
```

The gateway will start on `http://localhost:8080` by default.

## Contributing

This project is in early development. Contributions and feedback are welcome as we build out the core functionality.

## License

[License information to be added]
