# iana-gen

Type-safe, allocation-free Rust bindings for IANA protocol registries.

## Usage

Enable the catalogs you need through their IANA IDs:

```toml
[dependencies]
iana-gen = { version = "0.1.0", features = ["sdp-parameters", "tls-parameters"] }
```

The Rust import use: `iana_gen::sdp` and `iana_gen::tls`.

```rust
use iana_gen::tls::TlsContenttype;

assert_eq!(TlsContenttype::APPLICATION_DATA.value(), 23);
assert_eq!(TlsContenttype::new(22).name(), Some("handshake"));
```

## Development

Enter the development environment with `nix develop`, then run `just check`
to lint, build, test the workspace, and check licensing.
Run `just --list` to see all commands.

- `just iana-fetch`: discover and download all IANA catalogs.
- `just iana-generate`: regenerate Rust modules and features from cached XML.
- `just iana-check`: verify generated files match cached XML.
- `just iana-update`: fetch and regenerate.

## License

MIT OR Apache-2.0, IANA data retains the IANA/IETF Trust attribution and CC0-1.0.
