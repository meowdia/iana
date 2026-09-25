# IANA

Generic IANA Rust registry types and code-gen tooling.

## Generated files

Public paths remain `iana::sdp`, `iana::tls`, etc. Each catalog is enabled by its
Cargo feature (for example, `sdp-parameters`).

## Development

Enter the development environment with `nix develop`, then run `just check`
to lint, build, test the workspace, and check licensing.
Run `just --list` to see all commands.

- `just iana-fetch`: discover and download all IANA catalogs.
- `just iana-generate`: regenerate Rust modules and features from cached XML.
- `just iana-check`: verify generated files match cached XML.
- `just iana-update`: fetch and regenerate.

## License

MIT OR Apache-2.0, as inherited from Sphynx. Embedded registry data retains the
IANA/IETF Trust attribution and CC0-1.0 declaration in `REUSE.toml`.
