# iana

Type-safe Rust bindings generator for IANA protocol registries. No allocations, supports `no_std`.

Generation focuses on registries used by media servers: media delivery in general,
session protocols, authentication, and CDN integration, please feel free to add any iana registry to the list or publish it yourself.

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

```text
iana
├── iana-gen-shared (shared types and macros)
├── iana-acme
├── iana-address-family-numbers
├── iana-aead-parameters
├── iana-amtrelay-resource-record
├── iana-audio-telephone-event-registry
├── iana-authentication
│   ├── authentication-cryptographic-protocol-id
│   └── authentication-method-reference-values
├── iana-cdni-parameters
├── iana-cert-rr-types
├── iana-channel-binding-types
├── iana-character-sets
├── iana-comp-meth-ids
├── iana-cont-disp
├── iana-content-security-policy-directives
├── iana-cookie-attribute-names
├── iana-dane-parameters
├── iana-dcep-parameters
├── iana-dns
│   ├── dns-key-rr
│   ├── dns-parameters
│   ├── dns-sec-alg-numbers
│   ├── dns-sshfp-rr-parameters
│   ├── dns-svcb
│   ├── dnskey-flags
│   └── dnssec-nsec3-parameters
├── iana-ds-rr-types
├── iana-dscp-registry
├── iana-ebml
├── iana-flac
├── iana-hash-function-text-names
├── iana-hpke
├── iana-http
│   ├── http-alt-svc-parameters
│   ├── http-authentication-control-parameters
│   ├── http-authschemes
│   ├── http-cache-directives
│   ├── http-cache-status
│   ├── http-dig-alg
│   ├── http-digest-hash-alg
│   ├── http-fields
│   ├── http-message-signature
│   ├── http-methods
│   ├── http-parameters
│   ├── http-priority
│   ├── http-problem-types
│   ├── http-proxy-status
│   ├── http-status-codes
│   ├── http-upgrade-tokens
│   ├── http-warn-codes
│   ├── http2-parameters
│   └── http3-parameters
├── iana-ice
├── iana-internet-date-time-format
├── iana-ipseckey-rr-parameters
├── iana-jose
├── iana-jwt
├── iana-language
│   ├── language-subtag-registry
│   └── language-subtags-tags-extensions
├── iana-link-relations
├── iana-locally-served-dns-zones
├── iana-matroska
├── iana-media
│   ├── media-control-channel
│   ├── media-feature-tags
│   ├── media-type-structured-suffix
│   ├── media-type-sub-parameters
│   ├── media-types
│   └── media-types-parameters
├── iana-mls
├── iana-ntp-parameters
├── iana-oauth-parameters
├── iana-opus-channel-mapping-families
├── iana-passport
├── iana-pkix-parameters
├── iana-protocol-numbers
├── iana-quic
├── iana-rtcp
│   ├── rtcp-xr-block-types
│   └── rtcp-xr-sdp-parameters
├── iana-rtp-parameters
├── iana-rtsp-parameters
├── iana-rtspv2-parameters
├── iana-sctp-parameters
├── iana-sdp
│   ├── sdp-parameters
│   └── sdp-security-descriptions
├── iana-service
│   ├── service-codes
│   ├── service-function-chaining-service-function-types
│   └── service-names-port-numbers
├── iana-sframe
├── iana-sig-alg-numbers
├── iana-sip
│   ├── sip-clf-parameters
│   ├── sip-events
│   ├── sip-parameters
│   ├── sip-precond-types
│   ├── sip-priv-values
│   └── sip-table
├── iana-special
│   ├── special-registry
│   └── special-use-domain-names
├── iana-srtp-protection
├── iana-stun-parameters
├── iana-tcp
│   ├── tcp-convert-protocol-parameters
│   ├── tcp-header-flags
│   └── tcp-parameters
├── iana-tel-uri-parameters
├── iana-tls
│   ├── tls-ech-configuration-extensions
│   ├── tls-extensiontype-values
│   └── tls-parameters
├── iana-top-level-media-types
├── iana-trans
├── iana-tsig-algorithm-names
├── iana-udp
├── iana-uri-schemes
├── iana-uuid
├── iana-wave-avi-codec-registry
├── iana-webpush-parameters
├── iana-websocket
├── iana-well-known-uris
└── iana-whip
```

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
