# Development

## Releasing a new version

Pushing a tag matching `v[0-9]+.*` triggers `.github/workflows/rust.yml`, which:

1. Creates a GitHub release.
2. Builds and uploads binaries for `x86_64-unknown-linux-{gnu,musl}`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, and `universal-apple-darwin`.
3. Bumps the Homebrew formula in [`kolja/homebrew-loriini`](https://github.com/kolja/homebrew-loriini).
4. Bumps the AUR package [`loriini-bin`](https://aur.archlinux.org/packages/loriini-bin).

### Steps

```bash
# 1. Bump the version
$EDITOR Cargo.toml          # update `version = "X.Y.Z"`
cargo check                 # refreshes Cargo.lock
git add Cargo.toml Cargo.lock
git commit -m "Release vX.Y.Z"
git push

# 2. Tag and push
git tag vX.Y.Z
git push --tags
```

Watch the run at https://github.com/kolja/loriini/actions. The graph is:

```
create-release → upload-assets (×5 targets) → bump-homebrew
                                            ↘ bump-aur
```

If `bump-homebrew` or `bump-aur` fails, the GitHub release and binaries are unaffected — fix the issue and re-run the failed job from the Actions UI. No need to re-tag.

## Repository secrets

The release workflow needs two secrets on this repo (Settings → Secrets and variables → Actions → Repository secrets):

- **`TAP_TOKEN`** — fine-grained PAT scoped to `kolja/homebrew-loriini` with Contents: Read & Write. Used to push the formula bump.
- **`AUR_SSH_PRIVATE_KEY`** — a dedicated ed25519 private key whose public half is registered on the AUR account. Used by `KSXGitHub/github-actions-deploy-aur` to push to `ssh://aur@aur.archlinux.org/loriini-bin.git`.

