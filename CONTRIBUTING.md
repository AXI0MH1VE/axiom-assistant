# Contributing to Axiom Assistant

Thank you for your interest in contributing to Axiom Assistant! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Code Style](#code-style)
- [Documentation](#documentation)

## Code of Conduct

This project follows a standard code of conduct:

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on what is best for the community
- Show empathy towards other community members

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/axiom-assistant.git
   cd axiom-assistant
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/AXI0MH1VE/axiom-assistant.git
   ```

## Development Setup

### Prerequisites

- Rust 1.75+ with cargo
- Node.js 18+ and npm (for UI)
- System dependencies (see README.md)

### Install Dependencies

```bash
# Rust dependencies (automatically installed by cargo)
cargo build

# UI dependencies
cd ui
npm install
```

### Run in Development Mode

```bash
# Backend CLI
cargo run

# Tauri application
cd ui
npm run tauri dev
```

## Making Changes

### Branch Strategy

- `main` - Production-ready code
- `develop` - Integration branch for features
- `feature/*` - Individual features
- `bugfix/*` - Bug fixes
- `hotfix/*` - Critical production fixes

### Creating a Feature Branch

```bash
git checkout develop
git pull upstream develop
git checkout -b feature/your-feature-name
```

### Commit Messages

Follow conventional commits format:

```
type(scope): subject

body (optional)

footer (optional)
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat(probabilistic): add support for GGUF model loading
fix(orchestrator): correct hybrid query verification logic
docs(readme): update installation instructions
```

## Testing

### Run Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in specific module
cargo test modules::probabilistic
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Arrange
        let input = "test input";
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

### Integration Tests

Place integration tests in `tests/` directory:

```rust
// tests/integration_test.rs
use axiom_assistant::modules::*;

#[tokio::test]
async fn test_full_workflow() {
    // Test complete workflow
}
```

## Submitting Changes

### Before Submitting

1. **Run tests**: `cargo test`
2. **Run linter**: `cargo clippy`
3. **Format code**: `cargo fmt`
4. **Update documentation** if needed
5. **Add tests** for new features

### Create Pull Request

1. Push your branch to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. Go to GitHub and create a Pull Request from your fork to `upstream/develop`

3. Fill in the PR template:
   - Describe the changes
   - Reference any related issues
   - Add screenshots for UI changes
   - List any breaking changes

### PR Review Process

- Maintainers will review your PR
- Address any requested changes
- Once approved, your PR will be merged

## Code Style

### Rust Style Guidelines

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Maximum line length: 100 characters

### Naming Conventions

```rust
// Modules: lowercase with underscores
mod neural_network;

// Types: PascalCase
struct ProbabilisticModule;
enum Intent { Creative, Logical }

// Functions and variables: snake_case
fn process_query(input: &str) -> Result<String>;
let user_input = "example";

// Constants: SCREAMING_SNAKE_CASE
const MAX_TOKENS: usize = 1000;
```

### Error Handling

Always use `Result` for functions that can fail:

```rust
pub fn process_data(input: &str) -> anyhow::Result<Output> {
    let parsed = parse_input(input)?;
    let processed = transform(parsed)?;
    Ok(processed)
}
```

### Documentation

Document public APIs:

```rust
/// Process a user query and return a response stream.
/// 
/// # Arguments
/// 
/// * `query` - The user's input text
/// 
/// # Returns
/// 
/// A stream of response tokens
/// 
/// # Examples
/// 
/// ```
/// let stream = orchestrator.process_query("Hello").await;
/// ```
pub async fn process_query(&self, query: &str) -> Stream<String> {
    // Implementation
}
```

## Documentation

### Code Documentation

- Use `///` for public API documentation
- Use `//!` for module-level documentation
- Include examples in documentation
- Document error cases

### README Updates

Update README.md if you:
- Add new features
- Change installation steps
- Modify configuration options
- Update system requirements

### Changelog

Update CHANGELOG.md (if present) with:
- New features
- Bug fixes
- Breaking changes
- Deprecations

## Questions?

- **Issues**: Open an issue on GitHub
- **Discussions**: Use GitHub Discussions
- **Security**: See SECURITY.md for reporting vulnerabilities

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (see LICENSE file).

## Thank You!

Your contributions help make Axiom Assistant better for everyone. We appreciate your time and effort!
