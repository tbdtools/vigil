# Vigil

Vigil is an open-source Endpoint Detection and Response (EDR) system that leverages eBPF for efficient system monitoring and natively supports Sigma rules for threat detection. Built in Rust, it provides high-performance, low-overhead monitoring with a focus on extensibility and community-driven rule development.

## 🚀 Features

- **eBPF-powered Monitoring**: Efficient system monitoring using eBPF technology for minimal performance impact
- **Native Sigma Support**: First-class support for Sigma rules, enabling vendor-neutral threat detection
- **High Performance**: Built in Rust with a focus on efficiency and low resource usage
- **Extensible Architecture**: Plugin system for custom detectors and rule extensions
- **Real-time Alerting**: Immediate notification of suspicious activities
- **Community-Driven**: Open source with a focus on community contributions and rule sharing

## 🏗️ Project Status

> ⚠️ **Early Development**: This project is in active development and not yet ready for production use.

Current focus areas:

- Core eBPF monitoring infrastructure
- Sigma rule parsing and evaluation engine
- Basic event collection and alerting

## 🛠️ Installation

### Prerequisites

- Linux kernel 5.8+ (for eBPF features)
- Rust 1.75+
- LLVM and clang for eBPF program compilation
- libelf-dev and zlib1g-dev

### Building from Source

```bash
# Install dependencies (Ubuntu/Debian)
sudo apt install llvm clang libelf-dev zlib1g-dev

# Clone repository
git clone https://github.com/tbdtools/vigil.git
cd vigil

# Build
cargo build --release

# Install
cargo install --path .
```

## 🚦 Quick Start

```bash
# Start Vigil daemon
sudo vigil daemon start

# Load Sigma rules
vigil rules load path/to/rules

# View live events
vigil events watch

# Check status
vigil status
```

## 🏛️ Architecture

Vigil consists of several key components:

- **Core Engine**: eBPF program management and event collection
- **Rule Engine**: Sigma rule parsing and evaluation
- **Event Pipeline**: Collection, processing, and storage of system events
- **Alert Manager**: Alert generation and notification
- **CLI Interface**: User interaction and system management

## 📚 Documentation

- [User Guide](docs/user-guide.md)
- [Architecture Overview](docs/architecture.md)
- [Rule Development](docs/rule-development.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Security](SECURITY.md)

## 🤝 Contributing

We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md) for details on:

- Code style and guidelines
- Development setup
- Testing requirements
- Pull request process

## 🔒 Security

- For security issues, please refer to our [Security Policy](SECURITY.md)
- All rules undergo security review before acceptance
- Regular security audits of dependencies
- Minimal privilege requirements by design

## 📈 Performance

Vigil is designed with performance in mind:

- Efficient eBPF programs with minimal overhead
- Optimized Rust implementation
- Configurable resource limits
- Performance impact monitoring

## 🌟 Related Projects

- [Sigma](https://github.com/SigmaHQ/sigma): Generic Signature Format
- [aya](https://github.com/aya-rs/aya): eBPF development in Rust
- [osquery](https://github.com/osquery/osquery): SQL-powered monitoring

## ⚖️ License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- The Sigma project for their work on standardizing detection rules
- The Rust and eBPF communities
- All contributors and supporters

## 📞 Contact

- GitHub Issues: For bug reports and feature requests
- Security Issues: See [SECURITY.md](SECURITY.md)
- Community: [Discord](#) | [Matrix](#)

---

**Note**: Vigil is under active development. Star/watch the repository for updates!
