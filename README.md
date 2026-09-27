# iana

Type-safe Rust bindings generator for IANA protocol registries. No allocations, supports `no_std`.

## Usage

Add the crate for your protocol and enable the catalog you need:

```toml
[dependencies]
iana-tls = { version = "0.1.0", features = ["tls-parameters"] }
```

```rust
use iana_tls::tls::TlsContenttype;

assert_eq!(TlsContenttype::APPLICATION_DATA.value(), 23);
assert_eq!(TlsContenttype::new(22).name(), Some("handshake"));
```

## Crates

Registries are grouped by prefix into crates such as `iana-http`, `iana-dns`, and
`iana-tls`.

- One catalog: always available, no catalog feature needed.
- Multiple catalogs: enable the IANA catalog IDs you need; none are enabled by default.
- `metadata`: includes registry details and raw record fields.

The `iana-gen-shared` crate provides shared types and macros.

<details>
<summary>Package and catalog tree</summary>

Standalone crates need no feature flags. For grouped crates, enable the catalogs
listed beneath them.

<!-- BEGIN GENERATED CRATE TREE -->
<!-- END GENERATED CRATE TREE -->

</details>

## Development

Generate the crates before running checks:

```sh
nix develop
just iana-update
just check
```

Use `just iana-generate` to regenerate from cached XML, or `just --list` for all commands.

## License

MIT OR Apache-2.0, IANA data retains the IANA/IETF Trust attribution and CC0-1.0.
