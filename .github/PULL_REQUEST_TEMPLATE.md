## What and why

<!-- One or two sentences. Link the issue if there is one. -->

## Checks

- [ ] `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test --all` pass (or `./scripts/check.sh` / `.\scripts\check.ps1`)
- [ ] Scanning still never writes; cleanup still goes plan → quarantine → purge
- [ ] New adapter paths: OS and agent version where you verified them are in the PR text

Do not edit `CHANGELOG.md` in your pull request; the maintainer adds the entry, with credit to you, when it is merged. This keeps pull requests from conflicting with each other.
