# Contributing to EchoLedger

Thank you for your interest in contributing to EchoLedger! We welcome contributions from the community to help improve this project.

## 🚀 Getting Started

1. **Fork** the repository on GitHub
2. **Clone** your fork locally
   ```bash
   git clone https://github.com/your-username/echoledger.git
   cd echoledger
   ```
3. **Set up** the development environment (see [README.md](README.md))
4. Create a **new branch** for your changes
   ```bash
   git checkout -b feature/your-feature-name
   ```

## 🔧 Development Workflow

### Rust Canisters

```bash
# Build all canisters
dfx build

# Test a specific canister
cargo test -p emergency_bridge

# Format code
cargo fmt

# Check for clippy warnings
cargo clippy --all-targets --all-features -- -D warnings
```

### Frontend

```bash
# Install dependencies
cd frontend
yarn install

# Start development server
yarn start

# Run tests
yarn test

# Format code
yarn format

# Lint code
yarn lint
```

## 📝 Pull Request Process

1. Ensure your code follows the project's coding standards
2. Update the documentation as needed
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request with a clear description of your changes

## 🛠️ Code Style

### Rust
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for consistent formatting
- Document all public APIs with `///` doc comments

### TypeScript/React
- Use TypeScript for all new code
- Follow the [Airbnb JavaScript Style Guide](https://github.com/airbnb/javascript)
- Use functional components with hooks
- Prefer named exports over default exports

## 📜 License

By contributing to EchoLedger, you agree that your contributions will be licensed under the [MIT License](LICENSE).
