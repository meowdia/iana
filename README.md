# IANA

Rust registry types and snapshot tooling extracted from Sphynx. The initial
implementation still targets the IANA SDP Parameters registry; generalization
to other registries is future work.

The library exposes the existing enums through `iana::iana`. Generated code
lives in `src/iana/generated.rs`, the committed XML snapshot in
`iana/snapshots/`, and the fetch/generate/check implementation in
`xtask/src/main.rs`.

## Development

Enter the development environment with `nix develop`, then run `just check`
to check generated code, lint, build, test the workspace, and check licensing.
Run `just --list` to see all commands.

- `just iana-fetch`: download the SDP registry snapshot.
- `just iana-generate`: regenerate Rust enums from the local snapshot.
- `just iana-check`: verify generated code matches the local snapshot.
- `just iana-update`: fetch and regenerate.

Fetching requires network access. Generation and checks use the committed
snapshot and require Rust tooling (including rustfmt) and the `date` command.
Git provides diffs when generated code is stale.

GitHub Actions runs the Nix build, formatting, Clippy, workspace tests,
registry consistency, and REUSE licensing checks. Run `nix flake check`
locally for the Nix checks.

## License

MIT OR Apache-2.0, as inherited from Sphynx. Registry snapshots retain the
IANA/IETF Trust attribution and CC0-1.0 declaration in `REUSE.toml`.
