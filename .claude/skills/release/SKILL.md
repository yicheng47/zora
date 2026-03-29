---
name: release
description: Create a new release for Zora
---

# Release Workflow

1. Bump version in `Cargo.toml`
2. Run `cargo check` to verify
3. Run `cargo test` to verify tests pass
4. Create a git commit: `release: vX.Y.Z`
5. Create a git tag: `vX.Y.Z`
6. Push commit and tag: `git push && git push --tags`
7. Wait for CI to build release artifacts
8. Draft release notes on GitHub
