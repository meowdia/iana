# IANA

Generic IANA Rust registry types and code-gen tooling.

## Generated files

Public paths remain `iana::sdp`, `iana::tls`, etc. Each catalog is enabled by its
Cargo feature (for example, `sdp-parameters`).`.

## Development

Enter the development environment with `nix develop`, then run `just check`
to check generated code, lint, build, test the workspace, and check licensing.
Run `just --list` to see all commands.

- `just iana-fetch`: discover and download all IANA catalog snapshots.
- `just iana-generate`: regenerate Rust modules and features from local snapshots.
- `just iana-check`: verify generated files match local snapshots.
- `just iana-update`: fetch and regenerate.

Fetching requires network access. Generation and checks use local snapshots
and require Rust tooling (including rustfmt).
The consistency check reports missing, stale, or obsolete generated files.

## License

MIT OR Apache-2.0, as inherited from Sphynx. Registry snapshots retain the
IANA/IETF Trust attribution and CC0-1.0 declaration in `REUSE.toml`.
