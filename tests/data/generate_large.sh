#!/bin/bash

# Generate a large file with real words using Unix tools
OUTPUT="tests/data/large.txt"

# Create base text with common English words and sentences
cat > /tmp/base_text.txt << 'INNER_EOF'
The quick brown fox jumps over the lazy dog near the riverbank.
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
Documentation helps developers understand and maintain codebases effectively.
INNER_EOF

# Generate approximately 15MB of text by repeating and varying the content
echo "Generating large test file with real words..."

# Repeat base text multiple times with line numbers and variations
for i in {1..100000}; do
    # Add line numbers and vary the content
    echo "[$i] $(shuf -n 1 /tmp/base_text.txt)"
    
    # Occasionally add some variety
    if [ $((i % 1000)) -eq 0 ]; then
        echo "Section $i: Performance benchmarking and optimization techniques."
        echo "Data processing pipeline stage $i with error handling mechanisms."
        echo ""
    fi
    
    # Add some random word combinations every 500 lines
    if [ $((i % 500)) -eq 0 ]; then
        echo "Random words: $(shuf -n 5 /usr/share/dict/words 2>/dev/null | tr '\n' ' ' || echo 'alpha beta gamma delta epsilon')"
    fi
done > "$OUTPUT"

# Clean up
rm -f /tmp/base_text.txt

# Show file statistics
echo "Generated file: $OUTPUT"
ls -lh "$OUTPUT"
wc "$OUTPUT" 