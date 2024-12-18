# Release

  * [ ] Update `CHANGELOG.md` with version and publication date.
  * [ ] Within `/crates`, do the following:
    * [ ] Update version in the top-level `Cargo.toml`.
    * [ ] Run tests: `cargo test --all-features`.
    * [ ] Run linting: `cargo clippy --all-features`.
    * [ ] Run fmt: `cargo fmt --check`.
    * [ ] Run doc: `cargo doc --all-features`.
  * [ ] Search and update the (`v0.1.0`) in the code. In particular,
      * [ ] Update the API version in the main `api.rs`.
      * [ ] Update the example API version in the `/info` example response.
      * [ ] Regenerate the `swagger.yml` specification (`cd crates; cargo run
        --all-features --bin ccdi-spec generate -o ../swagger.yml -f; cd ..`).
  * [ ] Stage changes: `git add crates/Cargo.lock crates/Cargo.toml CHANGELOG.md
    swagger.yml`.
  * [ ] Create git commit: `git commit -m "release: bumps version to v0.1.0"`.
  * [ ] Create git tag: `git tag v0.1.0`.
  * [ ] Push release: `git push && git push --tags`.
  * [ ] Go to the Releases page in Github
    * [ ] Create a Release for this tag.
    * [ ] Copy the notes from the `CHANGELOG.md` file.
  * [ ] Update all relevant wiki pages (`cd crates; cargo run --release --bin
    ccdi-spec wiki subject | pbcopy; cd ..`).
    * Commit the updates with this message: `release: updates wiki page for
      v0.1.0`
