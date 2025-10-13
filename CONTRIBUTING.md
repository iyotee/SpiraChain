# Contributing to SpiraChain

Thank you for your interest in contributing to SpiraChain! This document provides guidelines for contributing to the project.

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help create a welcoming environment for all contributors

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/iyotee/Qbitum/issues)
2. Create a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - System information (OS, Rust version)
   - Relevant logs or screenshots

### Suggesting Features

1. Check existing feature requests
2. Create a new issue with:
   - Clear description of the feature
   - Use cases and benefits
   - Potential implementation approach

### Pull Requests

1. **Fork** the repository
2. **Create a branch** from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes**:
   - Write clear, documented code
   - Follow Rust best practices
   - Add tests for new functionality
   - Update documentation as needed
4. **Test your changes**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```
5. **Commit** with clear messages:
   ```bash
   git commit -m "Add feature: brief description"
   ```
6. **Push** to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```
7. **Create a Pull Request**:
   - Provide clear description of changes
   - Reference any related issues
   - Explain testing performed

## Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/Qbitum.git
cd Qbitum

# Install dependencies
./install.sh  # Linux/macOS
# or
install.bat   # Windows

# Build the project
cargo build

# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt
```

## Code Style

- Follow standard Rust conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write comprehensive tests
- Document public APIs with doc comments
- Keep functions focused and concise

## Testing

- Write unit tests for all new functionality
- Add integration tests for complex features
- Ensure all tests pass before submitting PR
- Test on multiple platforms when possible

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## Documentation

- Update README.md for user-facing changes
- Add doc comments for public APIs
- Update whitepaper for protocol changes
- Include code examples in documentation

## Commit Message Guidelines

Format: `<type>: <description>`

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Test additions or changes
- `chore`: Build process or tooling changes

Examples:
```
feat: Add Fibonacci spiral generation
fix: Correct π calculation precision
docs: Update installation instructions
test: Add tests for XMSS signatures
```

## Review Process

1. Maintainers will review your PR
2. Address any requested changes
3. Once approved, your PR will be merged
4. Your contribution will be acknowledged

## Getting Help

- Join our Discord (coming soon)
- Open a discussion issue
- Email: hello@spirachain.network

---

## CI/CD Requirements

All pull requests must pass:

1. **Formatting:** `cargo fmt --all -- --check`
2. **Linting:** `cargo clippy` with zero warnings
3. **Tests:** All unit and integration tests
4. **Security:** `cargo audit` with no vulnerabilities
5. **Benchmarks:** No performance regressions > 10%
6. **Testnet:** 3-node simulation producing blocks

### Pre-commit Checks

Install pre-commit hook:

```bash
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
EOF
chmod +x .git/hooks/pre-commit
```

On Windows (PowerShell):

```powershell
@'
cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) { exit 1 }
cargo clippy --all-targets --all-features -- -D warnings
if ($LASTEXITCODE -ne 0) { exit 1 }
cargo test --all
exit $LASTEXITCODE
'@ | Out-File -FilePath .git\hooks\pre-commit -Encoding ASCII
```

---

## License

By contributing, you agree that your contributions will be licensed under the GNU General Public License v3.0.

---

Thank you for helping make SpiraChain better! 🌀

