# Valinoreth - GURPS Game Mechanics Implementation
# Just command runner recipes

# Default recipe - show available commands
default:
    @just --list

# ── Development ───────────────────────────────────────────────────────────────

# Basic compilation check
check:
    cargo check --all-targets

# Run all tests
test:
    cargo test --all-targets

# Run specific test by name
test-one TEST:
    cargo test --all-targets {{TEST}}

# Run tests with output
test-verbose:
    cargo test --all-targets -- --nocapture

# Run clippy lints
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Check code formatting
fmt-check:
    cargo fmt --all -- --check

# Format code
fmt:
    cargo fmt --all

# Run all checks (clippy, fmt, test)
check-all: clippy fmt-check test

# Build in release mode
build:
    cargo build --release

# Clean build artifacts
clean:
    cargo clean

# ── Feature Testing ───────────────────────────────────────────────────────────

# Check with no default features
check-no-default:
    cargo check --no-default-features

# Check with all features
check-all-features:
    cargo check --all-features

# Test all feature combinations
check-features: check-no-default check-all-features check

# ── Documentation ─────────────────────────────────────────────────────────────

# Build documentation
doc:
    cargo doc --all-features --no-deps

# Build and open documentation
doc-open:
    cargo doc --all-features --no-deps --open

# Check documentation for broken links
doc-check:
    cargo doc --all-features --no-deps --document-private-items

# ── Dependencies ──────────────────────────────────────────────────────────────

# Update dependencies
update-deps:
    cargo update

# Show dependency tree
deps-tree:
    cargo tree

# Show outdated dependencies
deps-outdated:
    cargo outdated

# ── Security ──────────────────────────────────────────────────────────────────

# Run security audit
audit:
    cargo audit

# Generate OmniBOR artifact tree
omnibor:
    cargo omnibor

# Run all security checks
security: audit

# ── Release Management ────────────────────────────────────────────────────────

# Preview unreleased changelog entries
changelog-preview:
    git cliff --unreleased

# Update changelog with unreleased entries
changelog-update:
    git cliff --unreleased --prepend CHANGELOG.md

# Regenerate entire changelog
changelog-full:
    git cliff --output CHANGELOG.md

# Dry-run release (patch|minor|major)
release-dry-run LEVEL:
    cargo release {{LEVEL}} --dry-run

# Execute release (patch|minor|major)
release LEVEL:
    cargo release {{LEVEL}}

# Complete pre-release workflow
pre-release: check-all security changelog-preview
    @echo "Pre-release checks complete!"
    @echo "Next steps:"
    @echo "  1. Review changelog: just changelog-preview"
    @echo "  2. Update changelog: just changelog-update"
    @echo "  3. Commit changelog updates"
    @echo "  4. Execute release: just release [patch|minor|major]"

# ── Distribution ──────────────────────────────────────────────────────────────

# Build release artifacts
dist-build:
    cargo dist build

# Verify distribution without upload
dist-check:
    cargo dist plan

# Preview release plan
dist-plan:
    cargo dist plan

# Update CI workflow from dist config
dist-generate:
    cargo dist init --yes

# ── Setup ─────────────────────────────────────────────────────────────────────

# Install development tools
setup:
    @echo "Installing development tools..."
    cargo install cargo-audit
    cargo install cargo-outdated
    cargo install cargo-tree
    cargo install git-cliff
    cargo install cargo-release
    cargo install cargo-dist
    @echo "Development tools installed!"

# ── Markdown ──────────────────────────────────────────────────────────────────

# Check markdown formatting
markdown-check:
    markdownlint-cli2 "**/*.md"

# Fix markdown formatting
markdown-fix:
    markdownlint-cli2 "**/*.md" --fix
