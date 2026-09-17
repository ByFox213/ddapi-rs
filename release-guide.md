Release guide:
- Push version bump commit
    - Update /Cargo.toml version
- Add version tag with `git tag v0.3.0` and `git push origin --tags`
- Make GitHub release based on new tag
- Release on crates.io with `cargo publish`

PyPI (`ddapi`) release:
- The version is taken from `/Cargo.toml` automatically - bump it only there,
  do not edit `pyproject.toml` version (it is `dynamic`).
- Local setup: `python -m venv .env && source .env/bin/activate && pip install maturin`
- Install dev deps (pytest): `pip install --group dev -e .`
- Local check: `maturin develop && pytest tests/python/`
- Build wheels: `maturin build --release` (artifacts land in `target/wheels/`)
- Publish: `maturin publish` (requires a PyPI token in `MATURIN_PYPI_TOKEN`
  or interactive credentials; run once per release - it builds wheels for
  the current platform and uploads them)