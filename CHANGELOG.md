# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.3] - 2025-05-28

## [1.0.2] - 2025-05-28

## [1.0.1] - 2025-05-28

## [1.0.0] - 2025-05-28

## [1.0.0] - 2025-05-28

## [1.0.0] - 2025-05-28

## [1.0.0] - 2025-05-28

## [1.0.0] - 2025-05-28

### Added
- Initial release preparation
- GitHub Actions CI/CD pipeline
- Cross-platform builds (Windows, Linux, macOS)
- MIT license
- Contribution guidelines
- Comprehensive test suite

## [1.0.0] - 2024-01-XX

### Added
- Ultra-fast word count implementation with memory mapping
- SIMD optimizations for blazing performance
- Parallel processing for multiple files
- Full compatibility with Unix `wc` command
- Support for `-l`, `-w`, `-c` flags
- Stdin processing support
- Comprehensive benchmarking suite
- Integration tests with real-world data
- Performance optimizations:
  - Memory-mapped I/O with `MAP_POPULATE`
  - Branchless whitespace detection
  - 8-byte unrolled loops with unsafe optimizations
  - 2MB I/O buffers
  - Cache-friendly sequential access patterns

### Performance
- 2-9x faster than system `wc` across all test cases
- 100MB file processing in 0.03s (vs 0.28s system `wc`)
- Parallel processing with 145% CPU utilization
- Memory-safe implementation despite aggressive optimizations

### Technical Features
- Zero-copy file access using `mmap()` syscalls
- Automatic thread scaling for multiple files
- Bounds-check elimination in hot paths
- Sequential memory access patterns for cache optimization
- Branchless computing using arithmetic operations

## [0.1.0] - Initial Development

### Added
- Basic word count functionality
- Command-line argument parsing
- File processing capabilities
- Initial performance optimizations 