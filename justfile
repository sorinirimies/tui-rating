# tui-rating - A customizable rating widget library for ratatui
#
# Prerequisites: cargo install just && cargo install nu
# Usage: just <task> or just --list
# Patterns: See docs/JUSTFILE_PATTERNS.md for best practices

# Default task - show available commands
default:
    @just --list

# Install required tools (just, git-cliff)
install-tools:
    @echo "Installing required tools..."
    @command -v just >/dev/null 2>&1 || cargo install just
    @command -v git-cliff >/dev/null 2>&1 || cargo install git-cliff
    @echo "✅ All tools installed!"

# Build the project
build:
    cargo build

# Build release version
build-release:
    cargo build --release

# Run the interactive example
run:
    cargo run --example interactive

# Run basic example
example-basic:
    cargo run --example basic

# Run interactive example
example-interactive:
    cargo run --example interactive

# Run half_stars example
example-half-stars:
    cargo run --example half_stars

# Run symbols example
example-symbols:
    cargo run --example symbols

# Run custom example
example-custom:
    cargo run --example custom

# Run alignment example
example-alignment:
    cargo run --example alignment

# Run all examples
examples: example-basic example-interactive example-half-stars example-symbols example-custom example-alignment

# Run tests
test:
    cargo test

# Test the nushell scripts
test-scripts:
    nu scripts/test_scripts.nu

# Run tests with coverage
test-coverage:
    cargo tarpaulin --out Html --output-dir coverage

# Check code without building
check:
    cargo check

# Format code
fmt:
    cargo fmt

# Check if code is formatted
fmt-check:
    cargo fmt --check

# Run clippy linter
clippy:
    cargo clippy -- -D warnings

# Run all checks (fmt, clippy, test)
check-all: fmt-check clippy test test-scripts
    @echo "✅ All checks passed!"

# Clean build artifacts
clean:
    cargo clean

# Check if git-cliff is installed
check-git-cliff:
    @command -v git-cliff >/dev/null 2>&1 || { echo "❌ git-cliff not found. Install with: cargo install git-cliff"; exit 1; }

# Generate full changelog from all tags
changelog: check-git-cliff
    @echo "Generating full changelog..."
    git-cliff -o CHANGELOG.md
    @echo "✅ Changelog generated!"

# Generate changelog for unreleased commits only
changelog-unreleased: check-git-cliff
    @echo "Generating unreleased changelog..."
    git-cliff --unreleased --prepend CHANGELOG.md
    @echo "✅ Unreleased changelog generated!"

# Generate changelog for specific version tag
changelog-version version: check-git-cliff
    @echo "Generating changelog for version {{version}}..."
    git-cliff --tag v{{version}} -o CHANGELOG.md
    @echo "✅ Changelog generated for version {{version}}!"

# Preview changelog without writing to file
changelog-preview: check-git-cliff
    @git-cliff

# Preview unreleased changes
changelog-preview-unreleased: check-git-cliff
    @git-cliff --unreleased

# Generate changelog for latest tag only
changelog-latest: check-git-cliff
    @echo "Generating changelog for latest tag..."
    git-cliff --latest -o CHANGELOG.md
    @echo "✅ Latest changelog generated!"

# Update changelog with all commits (force regenerate)
changelog-update: check-git-cliff
    @echo "Regenerating complete changelog from all tags..."
    git-cliff --output CHANGELOG.md
    @echo "✅ Changelog updated from all git history!"

# Bump version (usage: just bump 0.2.0)
# Note: Runs check-all first to ensure code quality before version bump (fail early)
bump version: check-all check-git-cliff
    @echo "Bumping version to {{version}}..."
    nu scripts/bump_version.nu {{version}}

# Quick release: format, check, test, and build
release-check: fmt clippy test build-release
    @echo "✅ Ready for release!"

# Publish to crates.io (dry run)
publish-dry:
    cargo publish --dry-run

# Publish to crates.io
publish:
    cargo publish

# Run pre-publish readiness checks
check-publish:
    nu scripts/check_publish.nu

# Upgrade dependencies (nightly-style: upgrade pins, sync lock, quality gate)
upgrade-deps:
    nu scripts/upgrade_deps.nu --dry-run

# Update dependencies
update:
    cargo update

# Show outdated dependencies
outdated:
    cargo outdated

# Generate documentation
doc:
    cargo doc --no-deps --open

# Watch and auto-run on file changes (requires cargo-watch)
watch:
    cargo watch -x "run --example interactive"

# Git: commit current changes
commit message:
    git add .
    git commit -m "{{message}}"

# Git: pull from GitHub (origin)
pull:
    git pull origin main

# Git: pull from Gitea Microlab
pull-gitea-microlab:
    git pull gitea-microlab main

# Git: pull from Gitea (nexus-lab instance)
pull-gitea-nexus-lab:
    git pull gitea-nexus-lab main

# Git: pull from Gitea Starscream
pull-gitea-starscream:
    git pull gitea-starscream main

# Git: pull from all remotes (Gitea first, then GitHub)
pull-all:
    git pull gitea-microlab main
    git pull gitea-starscream main
    git pull gitea-nexus-lab main
    git pull origin main
    @echo "✅ Pulled from Gitea Microlab, Gitea Starscream, Gitea (nexus-lab), and GitHub!"

# Git: push to GitHub (origin)
push:
    git push origin main

# Git: push to Gitea Microlab
push-gitea-microlab:
    git push gitea-microlab main

# Git: push to Gitea (nexus-lab instance)
push-gitea-nexus-lab:
    git push gitea-nexus-lab main

# Git: push to Gitea Starscream
push-gitea-starscream:
    git push gitea-starscream main

# Git: push to GitHub, Gitea, and Gitea (nexus-lab)
push-all:
    git push origin main
    git push gitea-microlab main
    git push gitea-starscream main
    git push gitea-nexus-lab main
    @echo "✅ Pushed to GitHub, Gitea Microlab, Gitea Starscream, and Gitea (nexus-lab)!"

# Git: push tags to GitHub
push-tags:
    git push origin --tags

# Git: push tags to all remotes
push-tags-all:
    git push origin --tags
    git push gitea-microlab --tags
    git push gitea-starscream --tags
    git push gitea-nexus-lab --tags
    @echo "✅ Tags pushed to GitHub, Gitea Microlab, Gitea Starscream, and Gitea (nexus-lab)!"

# Full release workflow: bump version and push to GitHub
release version: (bump version)
    @echo "Pushing to GitHub..."
    git push origin main
    git push origin v{{version}}
    @echo "✅ Release v{{version}} complete on GitHub!"

# Full release workflow: bump version and push to Gitea Microlab
release-gitea-microlab version: (bump version)
    @echo "Pushing to Gitea Microlab..."
    git push gitea-microlab main
    git push gitea-microlab v{{version}}
    @echo "✅ Release v{{version}} complete on Gitea Microlab!"

# Full release workflow: bump version and push to Gitea (nexus-lab instance)
release-gitea-nexus-lab version: (bump version)
    @echo "Pushing to Gitea (nexus-lab)..."
    git push gitea-nexus-lab main
    git push gitea-nexus-lab v{{version}}
    @echo "✅ Release v{{version}} complete on Gitea (nexus-lab)!"

# Full release workflow: bump version and push to Gitea Starscream
release-gitea-starscream version: (bump version)
    @echo "Pushing to Gitea Starscream..."
    git push gitea-starscream main
    git push gitea-starscream v{{version}}
    @echo "✅ Release v{{version}} complete on Gitea Starscream!"

# Full release workflow: bump version and push to GitHub, Gitea, and Gitea (nexus-lab)
release-all version: (bump version)
    @echo "Pushing to all remotes..."
    git push origin main
    git push gitea-microlab main
    git push gitea-starscream main
    git push gitea-nexus-lab main
    git push origin v{{version}}
    git push gitea-microlab v{{version}}
    git push gitea-starscream v{{version}}
    git push gitea-nexus-lab v{{version}}
    @echo "✅ Release v{{version}} complete on all remotes!"

# Push release to all remotes (without bumping)
push-release-all:
    @echo "Pushing release to all remotes..."
    git push origin main
    git push gitea-microlab main
    git push gitea-starscream main
    git push gitea-nexus-lab main
    git push origin --tags
    git push gitea-microlab --tags
    git push gitea-starscream --tags
    git push gitea-nexus-lab --tags
    @echo "✅ Release pushed to all remotes!"

# Sync Gitea Microlab with GitHub (force)
sync-gitea-microlab:
    @echo "Syncing Gitea Microlab with GitHub..."
    git push gitea-microlab main --force
    git push gitea-microlab --tags --force
    @echo "✅ Gitea Microlab synced!"

# Sync Gitea Starscream with GitHub (force)
sync-gitea-starscream:
    @echo "Syncing Gitea Starscream with GitHub..."
    git push gitea-starscream main --force
    git push gitea-starscream --tags --force
    @echo "✅ Gitea Starscream synced!"

# Sync Gitea (nexus-lab instance) with GitHub (force)
sync-gitea-nexus-lab:
    @echo "Syncing Gitea (nexus-lab) with GitHub..."
    git push gitea-nexus-lab main --force
    git push gitea-nexus-lab --tags --force
    @echo "✅ Gitea (nexus-lab) synced!"

# Show configured remotes
remotes:
    @echo "Configured git remotes:"
    @git remote -v

# Setup Gitea remote (provide your Gitea URL)
setup-gitea url:
    @echo "Adding Gitea remote..."
    git remote add gitea-microlab {{url}}
    @echo "✅ Gitea remote added!"
    @echo "Test with: git push gitea-microlab main"

# Show current version
version:
    @nu scripts/version.nu

# Show git-cliff info
cliff-info:
    @echo "Git-cliff configuration:"
    @echo "  Config file: cliff.toml"
    @echo "  Installed: $(command -v git-cliff >/dev/null 2>&1 && echo '✅ Yes' || echo '❌ No (run: just install-tools)')"
    @command -v git-cliff >/dev/null 2>&1 && git-cliff --version || true

# Show project info
info:
    @echo "Project: tui-rating"
    @echo "Version: $(just version)"
    @echo "Author: Sorin Albu-Irimies"
    @echo "License: MIT"

# View changelog
view-changelog:
    @cat CHANGELOG.md

# Run the VHS tape for the basic demo
vhs-basic:
    @echo "Running VHS tape for basic..."
    vhs examples/vhs/basic.tape
    @echo "✅ Demo generated at examples/vhs/output/basic.gif"

# Run the VHS tape for the interactive demo
vhs-interactive:
    @echo "Running VHS tape for interactive..."
    vhs examples/vhs/interactive.tape
    @echo "✅ Demo generated at examples/vhs/output/interactive.gif"

# Run the VHS tape for the half_stars demo
vhs-half-stars:
    @echo "Running VHS tape for half_stars..."
    vhs examples/vhs/half_stars.tape
    @echo "✅ Demo generated at examples/vhs/output/half_stars.gif"

# Run the VHS tape for the symbols demo
vhs-symbols:
    @echo "Running VHS tape for symbols..."
    vhs examples/vhs/symbols.tape
    @echo "✅ Demo generated at examples/vhs/output/symbols.gif"

# Run the VHS tape for the custom demo
vhs-custom:
    @echo "Running VHS tape for custom..."
    vhs examples/vhs/custom.tape
    @echo "✅ Demo generated at examples/vhs/output/custom.gif"

# Run the VHS tape for the alignment demo
vhs-alignment:
    @echo "Running VHS tape for alignment..."
    vhs examples/vhs/alignment.tape
    @echo "✅ Demo generated at examples/vhs/output/alignment.gif"

# Run all VHS tapes (automatically discovers all .tape files)
vhs-all:
    @echo "🎬 Running automated VHS tape generation..."
    nu scripts/generate_all_tapes.nu

# Run all VHS tapes manually
vhs-all-manual: vhs-basic vhs-interactive vhs-half-stars vhs-symbols vhs-custom vhs-alignment
    @echo "✅ All demo GIFs generated!"
