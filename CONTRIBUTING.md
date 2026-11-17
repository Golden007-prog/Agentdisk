# Contributing

Thanks for helping. Starring the repo is a simple way to keep it in your list and helps others find it, but it is never needed to get an issue, help or a review.

The most useful contributions:

1. **Adapters for more agents** — see [docs/adapters.md](docs/adapters.md). Include where you verified the paths (OS, agent version).
2. **Fixes to existing adapter paths** — agents move their data often.
3. **Bug reports with `agentdisk scan --json` output** (remove personal paths first).

## Development

```sh
./scripts/check.sh        # macOS / Linux
.\scripts\check.ps1       # Windows (PowerShell)
```

Both run format, lint, tests, a release build and a smoke test.

The project uses no external crates on purpose: one small, auditable binary.

## Pull requests

Keep each pull request to one change, and run the checks above before you push. Do not edit `CHANGELOG.md` in your pull request; the maintainer adds the entry, with credit to you, when it is merged. This keeps pull requests from conflicting with each other.

## Rules that must not change

- Scanning never writes anything.
- Cleanup is plan → quarantine → purge, and apply/purge require a person at a terminal.
- The MCP server never gets tools that move or delete files.
- When in doubt, classify data as `user-data`.
