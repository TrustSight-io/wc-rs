#!/bin/bash

# Ultra Blazing WC Release Script
# This script automates the release process with proper checks and tagging

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]] || [[ ! -f "src/main.rs" ]]; then
    log_error "This script must be run from the project root directory"
    exit 1
fi

# Parse command line arguments
VERSION=""
DRY_RUN=false
SKIP_TESTS=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --skip-tests)
            SKIP_TESTS=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -v, --version VERSION    Version to release (e.g., 1.0.0)"
            echo "  --dry-run               Show what would be done without making changes"
            echo "  --skip-tests            Skip running tests (not recommended)"
            echo "  -h, --help              Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0 -v 1.0.0             Release version 1.0.0"
            echo "  $0 -v 1.1.0 --dry-run   Preview release 1.1.0"
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Validate version format
if [[ -z "$VERSION" ]]; then
    log_error "Version is required. Use -v or --version to specify it."
    echo "Example: $0 -v 1.0.0"
    exit 1
fi

if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    log_error "Version must be in format X.Y.Z (e.g., 1.0.0)"
    exit 1
fi

log_info "🚀 Starting release process for version $VERSION"

if [[ "$DRY_RUN" == true ]]; then
    log_warning "DRY RUN MODE - No changes will be made"
fi

# Check if git is clean
if [[ -n $(git status --porcelain) ]]; then
    log_error "Git working directory is not clean. Please commit or stash changes."
    git status --short
    exit 1
fi

# Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [[ "$CURRENT_BRANCH" != "main" ]]; then
    log_warning "You're not on the main branch (current: $CURRENT_BRANCH)"
    read -p "Continue anyway? [y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "Aborted by user"
        exit 1
    fi
fi

# Check if tag already exists
if git tag -l | grep -q "^v$VERSION$"; then
    log_error "Tag v$VERSION already exists"
    exit 1
fi

# Update version in Cargo.toml
log_info "📝 Updating version in Cargo.toml"
if [[ "$DRY_RUN" == false ]]; then
    sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
    rm Cargo.toml.bak
    log_success "Updated Cargo.toml version to $VERSION"
else
    log_info "Would update Cargo.toml version to $VERSION"
fi

# Update CHANGELOG.md
log_info "📝 Updating CHANGELOG.md"
CURRENT_DATE=$(date +%Y-%m-%d)
if [[ "$DRY_RUN" == false ]]; then
    sed -i.bak "s/## \[Unreleased\]/## [Unreleased]\n\n## [$VERSION] - $CURRENT_DATE/" CHANGELOG.md
    rm CHANGELOG.md.bak
    log_success "Updated CHANGELOG.md with release date"
else
    log_info "Would update CHANGELOG.md with release date $CURRENT_DATE"
fi

# Build release binary first (needed for integration tests)
log_info "🔨 Building release binary"
RUSTFLAGS="-C target-cpu=native" cargo build --release
log_success "Release binary built successfully"

# Run tests
if [[ "$SKIP_TESTS" == false ]]; then
    log_info "🧪 Running test suite"
    
    # Format check
    log_info "Checking code formatting..."
    cargo fmt --all -- --check
    
    # Clippy check
    log_info "Running clippy..."
    cargo clippy --all-targets --all-features -- -D warnings
    
    # Unit tests
    log_info "Running unit tests..."
    cargo test --verbose
    
    # Generate test data for integration tests
    if [[ -f "tests/data/generate_large.sh" ]]; then
        log_info "Generating test data..."
        cd tests/data
        chmod +x generate_large.sh
        ./generate_large.sh
        cd ../..
    fi
    
    # Integration tests
    log_info "Running integration tests..."
    cargo test --test integration_tests
    
    # Benchmarks
    if [[ -f "scripts/benchmark.sh" ]]; then
        log_info "Running benchmarks..."
        chmod +x scripts/benchmark.sh
        ./scripts/benchmark.sh
    fi
    
    log_success "All tests passed!"
else
    log_warning "Skipping tests as requested"
fi

# Commit changes
if [[ "$DRY_RUN" == false ]]; then
    log_info "📝 Committing release changes"
    git add Cargo.toml CHANGELOG.md
    git commit -m "chore: release v$VERSION"
    log_success "Committed release changes"
else
    log_info "Would commit release changes"
fi

# Create and push tag
if [[ "$DRY_RUN" == false ]]; then
    log_info "🏷️  Creating and pushing tag v$VERSION"
    git tag -a "v$VERSION" -m "Release v$VERSION"
    git push origin main
    git push origin "v$VERSION"
    log_success "Tag v$VERSION created and pushed"
else
    log_info "Would create and push tag v$VERSION"
fi

# Final instructions
log_success "🎉 Release process completed!"
echo ""
log_info "Next steps:"
echo "1. 🤖 GitHub Actions will automatically build binaries for all platforms"
echo "2. 📦 Binaries will be attached to the GitHub release"
echo "3. 📚 The package will be published to crates.io (if configured)"
echo "4. 🔍 Monitor the GitHub Actions workflow for any issues"
echo ""
log_info "GitHub Release URL: https://github.com/TrustSight-io/wc-rs/releases/tag/v$VERSION"
log_info "GitHub Actions: https://github.com/TrustSight-io/wc-rs/actions"

if [[ "$DRY_RUN" == true ]]; then
    echo ""
    log_warning "This was a dry run. No changes were made."
    log_info "Run without --dry-run to perform the actual release."
fi 