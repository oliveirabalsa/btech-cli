# btech

> A powerful CLI tool built with Rust for developers who need quick access to common utilities

<div align="center">

[![Rust Version](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

</div>

---

## 📋 Table of Contents

- [Overview](#-overview)
- [Key Features](#-key-features)
- [Quick Start](#-quick-start)
- [Installation](#-installation)
- [Usage](#-usage)
- [Commands](#-commands)
- [Examples](#-examples)
- [Configuration](#-configuration)
- [Development](#-development)
- [Contributing](#-contributing)
- [License](#-license)

---

## 🎯 Overview

**btech** is a lightweight, fast, and developer-friendly command-line tool written in Rust. It provides instant access to essential utilities that developers frequently need during development, testing, and prototyping.

Whether you need a quick UUID for your database records, placeholder text for your UI mockups, or just want a reliable tool that works across all platforms, **btech** has you covered.

---

## ✨ Key Features

### 🚀 Performance First

- **Blazing Fast**: Written in Rust for optimal performance
- **Low Resource Usage**: Minimal memory footprint and CPU usage
- **Instant Response**: Sub-millisecond execution times

### 🛠️ Developer Utilities

- **UUID Generation**: RFC 4122 compliant UUID v4 generation
- **Lorem Ipsum**: Customizable placeholder text generation
- **Extensible**: Easy to add new utilities and features

### 🌍 Cross-Platform

- **Universal Compatibility**: Works on Windows, macOS, and Linux
- **Consistent Experience**: Same interface and behavior across all platforms
- **No Dependencies**: Self-contained binary with no external requirements

### 🎨 User Experience

- **Intuitive CLI**: Clean and simple command-line interface
- **Rich Help System**: Comprehensive built-in documentation
- **Flexible Options**: Customizable parameters for all commands

---

## 🏃 Quick Start

Get up and running in under 60 seconds:

```bash
# Generate a UUID
btech uuid

# Generate placeholder text
btech lorem

# Get help
btech --help
```

---

## 📦 Installation

### Method 1: Install from Crates.io (Recommended)

```bash
cargo install btech
```

### Method 2: Build from Source

#### Prerequisites

- Rust 1.70 or higher ([Install Rust](https://rustup.rs/))
- Git

#### Build Steps

```bash
# Clone the repository
git clone https://github.com/yourusername/btech.git
cd btech

# Build in debug mode (for development)
cargo build

# Build optimized release version
cargo build --release

# Install globally
cargo install --path .
```

#### Verify Installation

```bash
btech --version
btech --help
```

### Method 3: Download Pre-built Binary

> _Coming soon_ - Pre-built binaries for popular platforms will be available for download.

---

## 📖 Usage

### Basic Syntax

```bash
btech [COMMAND] [OPTIONS]
```

### Global Options

```bash
btech --help          # Show help information
btech --version       # Show version information
```

---

## 🛠️ Commands

### UUID Generation

Generate RFC 4122 compliant UUID v4 strings:

```bash
btech uuid
```

**Options:** None

**Output:** A new UUID v4 string

### Lorem Ipsum Generation

Generate customizable placeholder text:

```bash
btech lorem [OPTIONS]
```

**Options:**

| Option    | Short | Default | Description                 |
| --------- | ----- | ------- | --------------------------- |
| `--count` | `-c`  | `10`    | Number of words to generate |

---

## 💡 Examples

### UUID Examples

```bash
# Generate a single UUID
$ btech uuid
550e8400-e29b-41d4-a716-446655440000

# Use in scripts
$ DATABASE_ID=$(btech uuid)
$ echo "New record ID: $DATABASE_ID"
```

### Lorem Ipsum Examples

```bash
# Generate default 10 words
$ btech lorem
Lorem ipsum dolor sit amet, consectetur adipiscing elit.

# Generate 25 words
$ btech lorem --count 25
$ btech lorem -c 25

# Generate 100 words for content testing
$ btech lorem -c 100

# Use in HTML templates
$ echo "<p>$(btech lorem -c 50)</p>" > sample.html

# Generate multiple paragraphs
$ for i in {1..3}; do echo "<p>$(btech lorem -c 30)</p>"; done
```

### Real-World Use Cases

```bash
# Database seeding
$ psql -c "INSERT INTO users (id, name, bio) VALUES ('$(btech uuid)', 'John Doe', '$(btech lorem -c 20)')"

# API testing
$ curl -X POST api.example.com/users \
  -H "Content-Type: application/json" \
  -d "{\"id\": \"$(btech uuid)\", \"description\": \"$(btech lorem -c 15)\"}"

# Content generation for prototyping
$ btech lorem -c 200 > mock_content.txt
```

---

## ⚙️ Configuration

**btech** is designed to work out of the box with sensible defaults. No configuration files are required.

### Environment Variables

| Variable      | Description                   | Default |
| ------------- | ----------------------------- | ------- |
| `BTECH_QUIET` | Suppress informational output | `false` |

---

## 🏗️ Development

### Prerequisites

- Rust 1.70+
- Cargo
- Git

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/btech.git
cd btech

# Run in development mode
cargo run -- uuid

# Run tests
cargo test

# Run with release optimizations
cargo run --release -- uuid
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Run comprehensive checks
cargo fmt --check && cargo clippy && cargo test
```

### Project Structure

```
src/
├── main.rs           # CLI entry point and command parsing
├── commands/         # Individual command implementations
│   ├── uuid.rs       # UUID generation logic
│   └── lorem.rs      # Lorem ipsum generation logic
└── utils/            # Shared utilities and helpers
    ├── formatting.rs # Output formatting functions
    └── validation.rs # Input validation helpers
```

---

## 🤝 Contributing

We welcome contributions! Here's how you can help:

### Ways to Contribute

- 🐛 **Bug Reports**: Found a bug? [Open an issue](https://github.com/yourusername/btech/issues)
- 💡 **Feature Requests**: Have an idea? [Start a discussion](https://github.com/yourusername/btech/discussions)
- 🔧 **Code Contributions**: Want to fix or improve something?
- 📚 **Documentation**: Help improve documentation and examples
- 🧪 **Testing**: Add tests or improve test coverage

### Development Workflow

1. **Fork** the repository
2. **Clone** your fork: `git clone https://github.com/yourusername/btech.git`
3. **Create** a feature branch: `git checkout -b feature/your-feature-name`
4. **Make** your changes
5. **Test** your changes: `cargo test`
6. **Format** your code: `cargo fmt`
7. **Commit** your changes: `git commit -m "Add your feature"`
8. **Push** to your branch: `git push origin feature/your-feature-name`
9. **Create** a Pull Request

### Guidelines

- Follow Rust best practices and idioms
- Add tests for new functionality
- Update documentation for any user-facing changes
- Keep the codebase clean and well-documented
- Use conventional commit messages

---

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

```text
MIT License

Copyright (c) 2025 Balsa Tech

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
```

---

## 🙏 Acknowledgments

**btech** stands on the shoulders of these amazing projects:

- [**clap**](https://github.com/clap-rs/clap) - Powerful command-line argument parsing
- [**uuid**](https://github.com/uuid-rs/uuid) - Robust UUID generation library
- [**lipsum**](https://github.com/mgeisler/lipsum) - High-quality lorem ipsum text generation

Special thanks to the Rust community for creating such an amazing ecosystem!

---

## 📞 Support

- 📧 **Email**: [support@btech.dev](mailto:support@btech.dev)
- 🐛 **Issues**: [GitHub Issues](https://github.com/yourusername/btech/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/yourusername/btech/discussions)
- 📖 **Documentation**: [GitHub Wiki](https://github.com/yourusername/btech/wiki)

---

<div align="center">

**Made with ❤️ by developers, for developers**

[⭐ Star us on GitHub](https://github.com/yourusername/btech) • [🐛 Report a bug](https://github.com/yourusername/btech/issues) • [💡 Request a feature](https://github.com/yourusername/btech/discussions)

</div>
