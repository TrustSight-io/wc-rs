#!/bin/bash

# Generate a large file with real words using Unix tools
# Use the script's directory as the base
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT="$SCRIPT_DIR/large.txt"

# Create base text with common English words and sentences
BASE_TEXT="The quick brown fox jumps over the lazy dog near the riverbank.
Software engineering requires careful planning and systematic implementation.
Machine learning algorithms process vast amounts of data efficiently.
Open source projects benefit from collaborative development practices.
Performance optimization involves profiling bottlenecks and improving algorithms.
Memory management is crucial for system stability and resource efficiency.
Network protocols enable communication between distributed computing systems.
Database transactions ensure data consistency and integrity constraints.
User interface design focuses on usability and accessibility principles.
Security measures protect against various types of cyber threats.
Cloud computing provides scalable infrastructure for modern applications.
Artificial intelligence transforms how we solve complex problems.
Version control systems track changes in software development workflows.
Testing methodologies validate functionality and prevent regression bugs.
Documentation helps developers understand and maintain codebases effectively."

# Generate approximately 8MB of text more efficiently
echo "Generating large test file with real words..."

# Create a temporary file with numbered lines
{
    # First create a chunk of text with line numbers
    for i in {1..100}; do
        echo "[$i] $BASE_TEXT" | tr '\n' ' '
        echo
    done
} > /tmp/chunk.txt

# Now replicate this chunk many times to reach ~8MB
{
    for j in {1..75}; do
        cat /tmp/chunk.txt
        echo "Section $j: Performance benchmarking and optimization techniques."
        echo "Data processing pipeline stage $j with error handling mechanisms."
        echo ""
    done
} > "$OUTPUT"

# Clean up
rm -f /tmp/chunk.txt

# Show file statistics
echo "Generated file: $OUTPUT"
ls -lh "$OUTPUT"
wc "$OUTPUT"