# Contributing to tui-rating

Thank you for your interest in contributing to tui-rating! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful, inclusive, and constructive in all interactions.

## How to Contribute

### Reporting Bugs

Before creating a bug report:
- Check the issue tracker to avoid duplicates
- Collect relevant information (OS, terminal, Rust version)
- Create a minimal reproducible example if possible

When filing a bug report, include:
- Clear description of the issue
- Steps to reproduce
- Expected vs actual behavior
- Environment details
- Code samples or screenshots

### Suggesting Features

For feature requests:
- Check existing issues and discussions
- Clearly describe the use case
- Provide examples of how it would work
- Consider backwards compatibility

### Pull Requests

All contributions must be submitted through GitHub using the forking workflow:

1. **Fork the Repository**
   - Visit https://github.com/sorinirimies/tui-rating
   - Click the "Fork" button to create your own copy of the repository

2. **Clone Your Fork**
   ```bash
   git clone https://github.com/YOUR-USERNAME/tui-rating.git
   cd tui-rating
   ```

3. **Add Upstream Remote**
   ```bash
   git remote add upstream https://github.com/sorinirimies/tui-rating.git
   ```

4. **Create a Branch**
   ```bash
   git checkout -b feature/my-new-feature
   ```

5. **Make Changes**
   - Write clear, idiomatic Rust code
   - Follow existing code style
   - Add tests for new functionality
   - Update documentation as needed

6. **Run Tests**
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

7. **Commit Changes**
   - Use conventional commit messages
   - Examples: `feat: add gradient color support`, `fix: handle edge case in animation`

8. **Push to Your Fork**
   ```bash
   git push origin feature/my-new-feature
   ```

9. **Create Pull Request**
   - Go to your fork on GitHub
   - Click "Compare & pull request"
   - Fill in the PR description with details about your changes
   - Submit the PR to the main repository

**Note**: Do not submit patches or contributions through email or other channels. All contributions must go through GitHub's pull request system on your forked repository.

## Development Setup

### Prerequisites

- Rust 1.88.0 or later
- A terminal that supports Unicode (for full visual testing)

### Building

```bash
# Build the library
cargo build

# Build with all features
cargo build --all-features

# Build examples
cargo build --examples
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Running Examples

```bash
# Run specific example
cargo run --example horizontal

# Or use just
just example-horizontal
```

### Using Just

This project includes a `justfile` for common tasks:

```bash
# List all commands
just

# Run checks (format, clippy, test)
just check

# Format code
just fmt

# Run clippy
just clippy

# Build documentation
just doc
```

## Code Style

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` to catch common mistakes
- Write idiomatic Rust code

### Documentation

- Add doc comments for all public items
- Include examples in doc comments
- Use `///` for item documentation
- Use `//!` for module documentation
- Run `cargo doc --open` to preview

Example:
```rust
/// Creates a rating with `value` out of `max` items (value is clamped).
///
/// # Example
///
/// ```
/// use tui_rating::Rating;
/// let rating = Rating::new(3.0, 5);
/// assert_eq!(rating.display_value(), 3.0);
/// ```
pub fn new(value: f64, max: usize) -> Self {
    // ...
}
```

### Testing

- Write unit tests for new functionality
- Add integration tests (`tests/`) where appropriate
- Test edge cases (0, max, NaN, out-of-range)
- Nushell scripts are tested with `just test-scripts`

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_value() {
        assert_eq!(RatingState::new(9.0, 5).value(), 5.0);
    }
}
```

## Project Structure

```
tui-rating/
├── src/
│   ├── lib.rs        # Public exports
│   ├── rating.rs     # Rating widget
│   ├── state.rs      # RatingState
│   └── symbols.rs    # RatingSymbols presets
├── examples/         # Example apps + VHS tapes
├── tests/            # Integration tests
└── scripts/          # Nushell automation
```

## Adding New Features

### New Symbol Set

1. Add a preset constructor in `src/symbols.rs`
2. Add it to the preset tests and `tests/integration.rs`
3. Show it in `examples/symbols.rs`
4. Update the README

### New Widget Option

1. Add a builder method on `Rating` in `src/rating.rs`
2. Add unit tests and docs
3. Demonstrate in an example

## Examples

When adding examples:
- Place in `examples/` directory
- Include clear comments explaining functionality
- Add keyboard controls help text
- Register in `Cargo.toml` under `[[example]]`
- Update README with description and run command

## Documentation

### README Updates

When adding significant features:
- Update feature list
- Add examples showing usage
- Update API sections
- Add to appropriate sections

### CHANGELOG

Follow [Keep a Changelog](https://keepachangelog.com/):
- Add entries under `[Unreleased]`
- Use categories: Added, Changed, Deprecated, Removed, Fixed, Security
- Write clear, user-focused descriptions

## Release Process

Maintainers only:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag: `git tag -a v0.x.x -m "Release v0.x.x"`
4. Push tag: `git push origin v0.x.x`
5. CI will build and publish to crates.io

## Questions?

- Open an issue for questions
- Check existing documentation
- Review examples for usage patterns

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to tui-rating! ⭐