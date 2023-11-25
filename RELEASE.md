# Release

  * [ ] Update `CHANGELOG.md` with version and publication date.
  * Within `/packages`, do the following:
    * [ ] Update version in the top-level `Cargo.toml`.
    * [ ] Run tests: `cargo test --all-features`.
    * [ ] Run linting: `cargo clippy --all-features`.
    * [ ] Run fmt: `cargo fmt --check`.
    * [ ] Run doc: `cargo doc`.
  * [ ] Update the `swagger.yml` specification to match the new version.
  * [ ] Stage changes: `git add packages/Cargo.lock packages/Cargo.toml
    CHANGELOG.md swagger.yml`.
  * [ ] Create git commit: `git commit -m "release: bumps version to v0.1.0"`.
  * [ ] Create git tag: `git tag v0.1.0`.
  * [ ] Push release: `git push && git push --tags`.
  * [ ] Go to the Releases page in Github
    * [ ] Create a Release for this tag.
    * [ ] Copy the notes from the `CHANGELOG.md` file.
  * [ ] Update all relevant wiki pages.
    * Commit the updates with this message: `release: updates wiki page for v0.1.0`