# Contributing to ApexCode

Thank you for your interest in contributing to ApexCode! This document provides guidelines for
contributing to the project.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
3. Create a new branch for your feature or bugfix
4. Make your changes
5. Submit a pull request

## Development Setup

```bash
# Clone the repository
git clone https://github.com/your-username/apexcode.git
cd apexcode

# Install dependencies
cargo build

# Run tests
cargo test

# Run with debug output
cargo run -- --help
```

## Code Style

- Follow Rust naming conventions (snake_case for variables/functions, PascalCase for types)
- Keep functions focused and small
- Add doc comments for public APIs
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for lints

## Testing

- Write unit tests for new functionality
- Ensure all tests pass before submitting a PR
- Test edge cases and error conditions

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Commit Messages

Follow conventional commits format:

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks
- `perf:` - Performance improvements

Example:

```
feat: add support for TypeScript detection
```

## Pull Request Process

1. Update the README if needed
2. Add tests for new functionality
3. Ensure all tests pass
4. Update documentation
5. Submit a clear description of your changes

## Questions?

Feel free to open an issue for questions or discussion.
