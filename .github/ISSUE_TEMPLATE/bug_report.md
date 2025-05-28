---
name: Bug Report
about: Create a report to help us improve
title: '[BUG] '
labels: 'bug'
assignees: ''
---

## Bug Description

A clear and concise description of what the bug is.

## To Reproduce

Steps to reproduce the behavior:
1. Run command: `ultra-blazing-wc [args]`
2. With input file: `[file description]`
3. Expected output: `[expected]`
4. Actual output: `[actual]`

## Expected Behavior

A clear and concise description of what you expected to happen.

## Actual Behavior

A clear and concise description of what actually happened.

## Environment

**System Information:**
- OS: [e.g., Ubuntu 22.04, macOS 13.0, Windows 11]
- Architecture: [e.g., x86_64, ARM64]
- Rust version: [output of `rustc --version`]
- Ultra Blazing WC version: [e.g., 1.0.0]

**Hardware:**
- CPU: [e.g., Intel i7-12700K, Apple M2]
- RAM: [e.g., 16GB]
- Storage: [e.g., NVMe SSD, HDD]

## Input Data

**File characteristics:**
- File size: [e.g., 100MB, 5KB]
- File type: [e.g., text, binary, UTF-8]
- Line count (approximate): [e.g., 1M lines]
- Special characters: [e.g., Unicode, null bytes]

**Sample input (if small):**
```
[paste small sample that reproduces the issue]
```

## Command and Output

**Command used:**
```bash
ultra-blazing-wc [your command here]
```

**Full output:**
```
[paste the complete output including any error messages]
```

**System `wc` output (for comparison):**
```bash
wc [same arguments]
# Output:
[paste system wc output]
```

## Performance Impact

- [ ] This is a performance regression
- [ ] This causes crashes or hangs
- [ ] This produces incorrect results
- [ ] This is a usability issue

**If performance regression:**
- Previous version performance: [e.g., 0.05s]
- Current version performance: [e.g., 0.15s]
- Performance degradation: [e.g., 3x slower]

## Additional Context

Add any other context about the problem here, such as:
- Does this happen with all files or specific types?
- Is this reproducible on different systems?
- Any workarounds you've found?
- Related issues or discussions?

## Logs and Debugging

**Debug output (if available):**
```bash
RUST_LOG=debug ultra-blazing-wc [args]
# Output:
[paste debug output]
```

**Stack trace (if crash):**
```
[paste stack trace if available]
```

## Checklist

- [ ] I have searched existing issues to ensure this is not a duplicate
- [ ] I have tested with the latest version
- [ ] I have provided all requested information
- [ ] I have tested with system `wc` for comparison
- [ ] I can reproduce this issue consistently 