Release guide:
- Push version bump commit
    - Update /Cargo.toml version
- Add version tag with `git tag v0.3.0` and `git push origin --tags`
- Make GitHub release based on new tag
- Release on crates.io with `cargo publish`