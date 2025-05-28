# Contributing to Ultra Blazing WC

Thank you for your interest in contributing to Ultra Blazing WC! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Performance Considerations](#performance-considerations)
- [Submitting Changes](#submitting-changes)
- [Release Process](#release-process)

## Code of Conduct

This project adheres to a code of conduct that we expect all contributors to follow:

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Maintain a professional environment

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/your-username/blazing-wc.git
   cd blazing-wc
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/original-owner/blazing-wc.git
   ```

## Development Setup

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Git
- A Unix-like environment (Linux, macOS, WSL on Windows)

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Run tests
cargo test

# Run benchmarks
./scripts/benchmark.sh
```

### Development Tools

We recommend installing these tools for development:

```bash
# Code formatting
rustup component add rustfmt

# Linting
rustup component add clippy

# Code coverage (optional)
cargo install cargo-tarpaulin
```

## Making Changes

### Branch Naming

Use descriptive branch names:
- `feature/add-new-optimization`
- `fix/memory-leak-in-mmap`
- `docs/update-readme`
- `perf/improve-simd-performance`

### Commit Messages

Follow conventional commit format:
```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New features
- `fix`: Bug fixes
- `perf`: Performance improvements
- `docs`: Documentation changes
- `test`: Test additions/modifications
- `refactor`: Code refactoring
- `style`: Code style changes

Examples:
```
feat(core): add AVX-512 support for ultra-fast counting
fix(mmap): handle edge case with zero-byte files
perf(simd): optimize whitespace detection with branchless ops
docs(readme): add performance comparison table
```

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Address all Clippy warnings (`cargo clippy`)
- Use meaningful variable names
- Add comments for complex algorithms
- Prefer safe Rust unless unsafe provides significant performance benefits

### Performance Guidelines

This project prioritizes performance. When contributing:

1. **Benchmark your changes**: Use `./scripts/benchmark.sh`
2. **Profile critical paths**: Use tools like `perf` or `cargo flamegraph`
3. **Consider memory access patterns**: Cache-friendly code is crucial
4. **Test on multiple architectures**: x86_64, ARM64, etc.
5. **Measure, don't guess**: Always validate performance claims

## Testing

### Running Tests

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration_tests

# Generate test data (if needed)
cd tests/data && ./generate_large.sh
```

### Test Requirements

- All new features must include tests
- Performance-critical code should include benchmarks
- Tests should cover edge cases (empty files, large files, etc.)
- Integration tests should verify compatibility with system `wc`

### Test Data

- Small test files are committed to the repository
- Large test files (>1MB) are generated via scripts
- Use realistic data that represents actual use cases

## Performance Considerations

### Optimization Priorities

1. **Algorithmic efficiency**: O(n) vs O(n²) matters more than micro-optimizations
2. **Memory access patterns**: Sequential > random access
3. **SIMD utilization**: Vectorized operations when beneficial
4. **Branch prediction**: Minimize unpredictable branches
5. **Cache efficiency**: Consider L1/L2/L3 cache sizes

### Unsafe Code Guidelines

When using `unsafe`:
- Document why it's necessary
- Explain safety invariants
- Provide safe alternatives when possible
- Test thoroughly on multiple platforms
- Consider using `#[cfg(feature = "unsafe-optimizations")]`

### Platform Considerations

- Test on Linux, macOS, and Windows
- Consider different CPU architectures (x86_64, ARM64)
- Account for different page sizes and memory models
- Validate SIMD instruction availability

## Submitting Changes

### Pull Request Process

1. **Update your branch**:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run the full test suite**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ./scripts/benchmark.sh
   ```

3. **Create a pull request** with:
   - Clear description of changes
   - Performance impact (if applicable)
   - Test results
   - Breaking changes (if any)

### Pull Request Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Performance improvement
- [ ] Documentation update
- [ ] Breaking change

## Performance Impact
- Benchmark results (if applicable)
- Memory usage changes
- CPU utilization changes

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Benchmarks run successfully
- [ ] Tested on multiple platforms

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No new warnings
```

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):
- `MAJOR.MINOR.PATCH`
- Major: Breaking changes
- Minor: New features (backward compatible)
- Patch: Bug fixes (backward compatible)

### Release Checklist

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run full test suite
4. Create release PR
5. Tag release after merge
6. GitHub Actions will build and publish binaries

## Questions?

- Open an issue for bugs or feature requests
- Start a discussion for questions or ideas
- Check existing issues before creating new ones

Thank you for contributing to Ultra Blazing WC! 🚀 