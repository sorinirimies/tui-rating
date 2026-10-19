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

# Audit dependencies for advisories (cargo install cargo-audit)
audit:
    cargo audit

# Check advisories, licenses, bans and sources (cargo install cargo-deny)
deny:
    cargo deny check

# List fuzz targets (needs nightly + cargo install cargo-fuzz)
fuzz-list:
    cargo +nightly fuzz list

# Fuzz one target (see `just fuzz-list`); SECONDS bounds the run
fuzz target seconds="30":
    cargo +nightly fuzz run {{target}} -- -max_total_time={{seconds}}

# Build the project
build:
    cargo build

# Build release version
build-release:
    cargo build --release

# Run the interactive example
run:
    cargo run --features theme --example interactive

# Run basic example
example-basic:
    cargo run --example basic

# Run interactive example
example-interactive:
    cargo run --features theme --example interactive

# Run half_stars example
example-half-stars:
    cargo run --example half_stars

# Run symbols example
example-symbols:
    cargo run --example symbols

# Run styles example
example-styles:
    cargo run --example styles

# Run custom example
example-custom:
    cargo run --example custom

# Run alignment example
example-alignment:
    cargo run --example alignment

# Run all examples
examples: example-basic example-interactive example-half-stars example-symbols example-styles example-custom example-alignment

# Run tests
test:
    cargo test --all-features

# Test the nushell scripts
test-scripts:
    nu scripts/test_scripts.nu

# Run tests with coverage
test-coverage:
    cargo llvm-cov --all-features --html --output-dir coverage

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
    cargo clippy --all-features -- -D warnings

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

# tui-rating lives on GitHub (origin) and the Gitea instances: Microlab, Starscream and
# nexus-lab. Every *-all recipe loops over REMOTES, skips remotes this checkout does not have,
# and keeps going when one fails. Recipes that move history by force say so: be sure!

# Fail fast (5 s) instead of hanging when a remote host is unreachable.
export GIT_SSH_COMMAND := "ssh -o ConnectTimeout=5 -o BatchMode=yes"

# Unreachable remotes warn and are skipped; real rejections still fail the recipe.
PUSHFN := 'gpush() { out=$(git push "$@" 2>&1); rc=$?; if [ $rc -ne 0 ] && printf "%s" "$out" | grep -Eqi "timed out|could not resolve|connection refused|no route to host|network is unreachable"; then echo "⚠️  $r unreachable - NOT pushed (skipped)"; return 0; fi; [ -n "$out" ] && printf "%s\n" "$out"; return $rc; }'

REMOTES := "origin gitea-microlab gitea-starscream gitea-nexus-lab"

# ── push ──

# Push the current branch to GitHub (origin)
push:
    git push origin "$(git branch --show-current)"

# Force-push the current branch to GitHub (origin). No lease!
push-force:
    git push --force origin "$(git branch --show-current)"

# Push the current branch to Gitea Microlab
push-gitea-microlab:
    git push gitea-microlab "$(git branch --show-current)"

# Force-push the current branch to Gitea Microlab. No lease!
push-gitea-microlab-force:
    git push --force gitea-microlab "$(git branch --show-current)"

# Push the current branch to Gitea Starscream
push-gitea-starscream:
    git push gitea-starscream "$(git branch --show-current)"

# Force-push the current branch to Gitea Starscream. No lease!
push-gitea-starscream-force:
    git push --force gitea-starscream "$(git branch --show-current)"

# Push the current branch to Gitea (nexus-lab instance)
push-gitea-nexus-lab:
    git push gitea-nexus-lab "$(git branch --show-current)"

# Force-push the current branch to Gitea (nexus-lab instance). No lease!
push-gitea-nexus-lab-force:
    git push --force gitea-nexus-lab "$(git branch --show-current)"

# Push the current branch to every remote (continues on failure)
push-all:
    #!/usr/bin/env sh
    {{PUSHFN}}
    failed=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured)"; continue; }
        gpush "$r" "$(git branch --show-current)" || failed="$failed $r"
    done
    if [ -n "$failed" ]; then echo "⚠️  Failed to push to:$failed"; exit 1; fi
    echo "✅ Pushed to every configured remote"

# Force-push the current branch to every remote (continues on failure). No lease!
push-all-force:
    #!/usr/bin/env sh
    {{PUSHFN}}
    failed=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured)"; continue; }
        gpush --force "$r" "$(git branch --show-current)" || failed="$failed $r"
    done
    if [ -n "$failed" ]; then echo "⚠️  Failed to force-push to:$failed"; exit 1; fi
    echo "✅ Force-pushed to every configured remote"

# ── pull ──

# Pull the current branch from GitHub (origin)
pull:
    git pull origin "$(git branch --show-current)"

# Pull the current branch from Gitea Microlab
pull-gitea-microlab:
    git pull gitea-microlab "$(git branch --show-current)"

# Pull the current branch from Gitea Starscream
pull-gitea-starscream:
    git pull gitea-starscream "$(git branch --show-current)"

# Pull the current branch from Gitea (nexus-lab instance)
pull-gitea-nexus-lab:
    git pull gitea-nexus-lab "$(git branch --show-current)"

# Pull the current branch from every remote (continues on failure)
pull-all:
    #!/usr/bin/env sh
    failed=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured)"; continue; }
        git pull "$r" "$(git branch --show-current)" || failed="$failed $r"
    done
    if [ -n "$failed" ]; then echo "⚠️  Failed to pull from:$failed"; exit 1; fi
    echo "✅ Pulled from every configured remote"

# ── tags ──

# Push all tags to GitHub
push-tags:
    git push origin --tags

# Force-push all tags to GitHub (moves existing tags). Be sure!
push-tags-force:
    git push --force origin --tags

# Push all tags to every remote (continues on failure)
push-tags-all:
    #!/usr/bin/env sh
    {{PUSHFN}}
    failed=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured)"; continue; }
        gpush "$r" --tags || failed="$failed $r"
    done
    if [ -n "$failed" ]; then echo "⚠️  Failed to push tags to:$failed"; exit 1; fi
    echo "✅ Tags pushed to every configured remote"

# Force-push all tags to every remote (continues on failure; moves existing tags). Be sure!
push-tags-all-force:
    #!/usr/bin/env sh
    {{PUSHFN}}
    failed=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured)"; continue; }
        gpush --force "$r" --tags || failed="$failed $r"
    done
    if [ -n "$failed" ]; then echo "⚠️  Failed to force-push tags to:$failed"; exit 1; fi
    echo "✅ Tags force-pushed to every configured remote"

# ── release ──
# Pushing a vX.Y.Z tag starts the Release workflow on whichever host receives it
# (GitHub: .github/workflows/release.yml, Gitea: .gitea/workflows/release.yml).

# Bump version, then push to GitHub (branch + tag): triggers the Release workflow
release version: (bump version)
    @echo "Pushing release v{{version}} to GitHub…"
    git push --follow-tags origin "$(git branch --show-current)"
    @echo "✅ Release v{{version}} pushed: the Release workflow will trigger automatically."

# Bump version, then push to Gitea Microlab only
release-gitea-microlab version: (bump version)
    git push --follow-tags gitea-microlab "$(git branch --show-current)"
    @echo "✅ Release v{{version}} live on Gitea Microlab."

# Bump version, then push to Gitea (nexus-lab instance) only
release-gitea-nexus-lab version: (bump version)
    git push --follow-tags gitea-nexus-lab "$(git branch --show-current)"
    @echo "✅ Release v{{version}} live on Gitea (nexus-lab instance)."

# Bump version, then push to Gitea Starscream only
release-gitea-starscream version: (bump version)
    git push --follow-tags gitea-starscream "$(git branch --show-current)"
    @echo "✅ Release v{{version}} live on Gitea Starscream."

# Bump version, then push to every remote (continues on failure)
release-all version: (bump version)
    #!/usr/bin/env sh
    {{PUSHFN}}
    failed=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured)"; continue; }
        gpush --follow-tags "$r" "$(git branch --show-current)" || failed="$failed $r"
    done
    if [ -n "$failed" ]; then echo "⚠️  Release failed to push to:$failed"; exit 1; fi
    echo "✅ Release v{{version}} pushed to every configured remote"

# Push the latest commit and its tags to every remote, no bump (continues on failure)
push-release-all: check-all
    #!/usr/bin/env sh
    {{PUSHFN}}
    failed=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured)"; continue; }
        gpush --follow-tags "$r" "$(git branch --show-current)" || failed="$failed $r"
    done
    if [ -n "$failed" ]; then echo "⚠️  Failed to push to:$failed"; exit 1; fi
    echo "✅ Latest commit and tags pushed to every configured remote"

# Force-push the latest commit and its tags to every remote, no bump. Be sure!
push-release-all-force: check-all
    #!/usr/bin/env sh
    {{PUSHFN}}
    failed=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured)"; continue; }
        gpush --force --follow-tags "$r" "$(git branch --show-current)" || failed="$failed $r"
    done
    if [ -n "$failed" ]; then echo "⚠️  Failed to force-push to:$failed"; exit 1; fi
    echo "✅ Latest commit and tags force-pushed to every configured remote"

# Short name for push-release-all
alias release-push := push-release-all

# Manually re-trigger the Release workflow for an existing tag via the gh CLI
release-retrigger version:
    @command -v gh >/dev/null 2>&1 || { echo "❌ GitHub CLI (gh) not found. Install from https://cli.github.com"; exit 1; }
    @echo "Manually dispatching the Release workflow for tag v{{ version }}…"
    gh workflow run release.yml -f tag=v{{ version }}
    @echo "✅ Dispatched. Progress: https://github.com/$(gh repo view --json nameWithOwner -q .nameWithOwner)/actions"

# ── force-sync Gitea with GitHub (main + tags) ──

# Force-sync Gitea Microlab with GitHub
sync-gitea-microlab:
    git push gitea-microlab "$(git branch --show-current)" --force
    git push gitea-microlab --tags --force
    @echo "✅ Gitea Microlab force-synced with GitHub."

# Force-sync Gitea Starscream with GitHub
sync-gitea-starscream:
    git push gitea-starscream "$(git branch --show-current)" --force
    git push gitea-starscream --tags --force
    @echo "✅ Gitea Starscream force-synced with GitHub."

# Force-sync Gitea (nexus-lab instance) with GitHub
sync-gitea-nexus-lab:
    git push gitea-nexus-lab "$(git branch --show-current)" --force
    git push gitea-nexus-lab --tags --force
    @echo "✅ Gitea (nexus-lab instance) force-synced with GitHub."

# Force-sync every configured Gitea instance with GitHub (continues on failure)
sync-all-gitea:
    #!/usr/bin/env sh
    failed=""
    for r in gitea-microlab gitea-starscream gitea-nexus-lab; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured)"; continue; }
        git push "$r" "$(git branch --show-current)" --force && git push "$r" --tags --force || failed="$failed $r"
    done
    if [ -n "$failed" ]; then echo "⚠️  Failed to sync:$failed"; exit 1; fi
    echo "✅ Every configured Gitea instance force-synced with GitHub."

# Show configured remotes
remotes:
    @echo "Configured git remotes:"
    @git remote -v

# Add the remotes this repo is expected to have (nexus-lab: `git remote add gitea-nexus-lab <url>`)
setup-remotes:
    #!/usr/bin/env sh
    git remote get-url origin >/dev/null 2>&1 || git remote add origin git@github.com:sorinirimies/tui-rating.git
    git remote get-url gitea-microlab >/dev/null 2>&1 || git remote add gitea-microlab ssh://git@192.168.1.204:30009/sorin/tui-rating.git
    git remote get-url gitea-starscream >/dev/null 2>&1 || git remote add gitea-starscream gitea@192.168.1.44:sorin/tui-rating.git
    git remote -v

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

# Run the VHS tape for the styles demo
vhs-styles:
    @echo "Running VHS tape for styles..."
    vhs examples/vhs/styles.tape
    @echo "✅ Demo generated at examples/vhs/output/styles.gif"

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

# Render a single tape by name (e.g. just vhs-tape styles)
vhs-tape name:
    #!/usr/bin/env sh
    if [ -f "examples/vhs/{{ name }}.tape" ]; then
        vhs "examples/vhs/{{ name }}.tape" && echo "✅ Done."
    else
        echo "❌ Tape not found: {{ name }}.tape"
        just vhs-list
        exit 1
    fi

# List all VHS tapes
vhs-list:
    @ls examples/vhs/*.tape | sed 's|.*/||; s|\.tape||' | sed 's/^/  /'

# Remove generated demo GIFs
vhs-clean:
    @rm -f examples/vhs/output/*.gif
    @echo "✅ Removed generated GIFs"

# Run all VHS tapes (automatically discovers all .tape files)
vhs-all:
    @echo "🎬 Running automated VHS tape generation..."
    nu scripts/generate_all_tapes.nu

# Run all VHS tapes manually
vhs-all-manual: vhs-basic vhs-interactive vhs-half-stars vhs-symbols vhs-styles vhs-custom vhs-alignment
    @echo "✅ All demo GIFs generated!"
