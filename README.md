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

```text
iana
├── iana-gen-shared (shared types and macros)
├── iana-6lowpan-parameters
├── iana-6tisch
├── iana-aaa-parameters
├── iana-abfab-parameters
├── iana-about-uri-tokens
├── iana-acap-registrations
├── iana-access-types
├── iana-ace
├── iana-acl-tls
├── iana-acme
├── iana-acp
├── iana-address-family-numbers
├── iana-adsp-parameters
├── iana-aead-parameters
├── iana-aka-version-namespace
├── iana-alert-urns
├── iana-alto-protocol
├── iana-amtrelay-resource-record
├── iana-ancp
├── iana-aodv-parameters
├── iana-apex
├── iana-arp-parameters
├── iana-as-numbers
├── iana-audio-telephone-event-registry
├── iana-auth-namespaces
├── iana-authentication
│   ├── authentication-cryptographic-protocol-id
│   └── authentication-method-reference-values
├── iana-auto
│   ├── auto-response-parameters
│   └── auto-submitted-keywords
├── iana-babel
├── iana-bandwidth-constraints-model-ids
├── iana-battery-technologies
├── iana-beep-parameters
├── iana-bfcp-parameters
├── iana-bfd-parameters
├── iana-bgp
│   ├── bgp-data-collection-communities-std
│   ├── bgp-extended-communities
│   ├── bgp-ls-parameters
│   ├── bgp-parameters
│   ├── bgp-spf
│   ├── bgp-tunnel-encapsulation
│   └── bgp-well-known-communities
├── iana-bier
├── iana-bmp-parameters
├── iana-bootp-dhcp-parameters
├── iana-bpf-instructions
├── iana-brski-parameters
├── iana-building-blocks
├── iana-bundle
├── iana-c-dns
├── iana-calipso
├── iana-caller-id-plans
├── iana-capability-codes
├── iana-captive-portals
├── iana-capwap-parameters
├── iana-card-parameters
├── iana-cbor
│   ├── cbor-encoded-x509-c509
│   ├── cbor-simple-values
│   └── cbor-tags
├── iana-ccmp-parameters
├── iana-ccnx
├── iana-cddl
├── iana-cdni-parameters
├── iana-cert-rr-types
├── iana-cfm-oam
├── iana-cga-message-types
├── iana-channel-binding-types
├── iana-character-sets
├── iana-charset
│   ├── charset-info
│   └── charset-reg
├── iana-cisco-sla-protocol
├── iana-civic-address-types-registry
├── iana-clue
├── iana-cmp
├── iana-cnrp-parameters
├── iana-coap-eap
├── iana-comp-meth-ids
├── iana-cont-disp
├── iana-content-security-policy-directives
├── iana-cookie-attribute-names
├── iana-cops-parameters
├── iana-core-parameters
├── iana-cose
│   ├── cose
│   └── cose-suit-algorithm-profiles
├── iana-coswid
├── iana-cpim-headers
├── iana-crypto-suites
├── iana-cwt
├── iana-cxtp-parameters
├── iana-dane-parameters
├── iana-dcap-parameters
├── iana-dccp
│   ├── dccp-ccid2-parameters
│   ├── dccp-ccid3-parameters
│   ├── dccp-ccid4-parameters
│   └── dccp-parameters
├── iana-dcep-parameters
├── iana-detnet-ach-flags
├── iana-device-identification
├── iana-dhcpv6-parameters
├── iana-directory-system-names
├── iana-dkim-parameters
├── iana-dlep-parameters
├── iana-dmarc-parameters
├── iana-dncp-registry
├── iana-dns
│   ├── dns-key-rr
│   ├── dns-parameters
│   ├── dns-sec-alg-numbers
│   ├── dns-sshfp-rr-parameters
│   ├── dns-svcb
│   ├── dnskey-flags
│   └── dnssec-nsec3-parameters
├── iana-dots
├── iana-drip
├── iana-ds-rr-types
├── iana-dscp-registry
├── iana-dskpp
├── iana-dsn-types
├── iana-dsr-parameters
├── iana-dssc
├── iana-eap
│   ├── eap-channel-binding-parameters
│   ├── eap-eke
│   ├── eap-fast-parameters
│   ├── eap-gpsk-parameters
│   ├── eap-ikev2-payloads
│   ├── eap-noob
│   ├── eap-numbers
│   ├── eap-pax
│   ├── eap-psk-parameters
│   ├── eap-pwd-parameters
│   └── eap-sake-parameters
├── iana-eappotp-identifiers
├── iana-eapsimaka-numbers
├── iana-ebml
├── iana-ecml-parameters
├── iana-ecmlv2-parameters
├── iana-edhoc
├── iana-email-auth
├── iana-emergency
│   ├── emergency-call-additional-data
│   └── emergency-call-metadata-control-data
├── iana-emsk-parameters
├── iana-enum-services
├── iana-epp
│   ├── epp-extension-role-values
│   ├── epp-extensions
│   └── epp-repository-ids
├── iana-esp-aggfrag-payload
├── iana-ethernet-numbers
├── iana-evpn
├── iana-ex-mobility-subtypes
├── iana-fc-port-types
├── iana-fcast
├── iana-fedfs-parameters
├── iana-flac
├── iana-flow-spec
├── iana-flute-parameters
├── iana-foobar-af-numbers
├── iana-forces
├── iana-ftp-commands-extensions
├── iana-g-ach-parameters
├── iana-gdoi-payloads
├── iana-geo-uri-parameters
├── iana-geolocation-policy
├── iana-geopriv-identifiers
├── iana-gist-parameters
├── iana-gmpls
│   ├── gmpls-sig-parameters
│   └── gmpls-wson
├── iana-gnap
├── iana-gnss
├── iana-grasp-parameters
├── iana-gre-parameters
├── iana-gsakmp-parameters
├── iana-gsmpv3
│   ├── gsmpv3-adapt
│   ├── gsmpv3-event
│   ├── gsmpv3-failure
│   ├── gsmpv3-label
│   ├── gsmpv3-message
│   ├── gsmpv3-model
│   ├── gsmpv3-port
│   ├── gsmpv3-result
│   ├── gsmpv3-service
│   └── gsmpv3-traffic
├── iana-gss
│   ├── gss-api-parameters
│   └── gss-eap-parameters
├── iana-gssapi-service-names
├── iana-gstn-extensions
├── iana-hash-function-text-names
├── iana-held-parameters
├── iana-hip-parameters
├── iana-hoba
│   ├── hoba-device-identifiers
│   ├── hoba-key-identifiers
│   └── hoba-signature-algorithms
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
├── iana-iana
│   ├── iana-as-numbers-special-registry
│   ├── iana-bfd-types
│   ├── iana-bgp-l2-encaps
│   ├── iana-crypt-hash
│   ├── iana-dns-class-rr-type
│   ├── iana-dots-signal-channel
│   ├── iana-hardware
│   ├── iana-http-versions
│   ├── iana-icmpv4-types
│   ├── iana-icmpv6-types
│   ├── iana-if-type
│   ├── iana-igp-algo-types
│   ├── iana-igp-link-attr-apps
│   ├── iana-igp-metric-types
│   ├── iana-ioam-integrity-protection-methods
│   ├── iana-ipv4-special-registry
│   ├── iana-ipv6-ext-types
│   ├── iana-ipv6-special-registry
│   ├── iana-msd-types
│   ├── iana-ospf-functional-cap-bits
│   ├── iana-pseudowire-types
│   ├── iana-routing-types
│   ├── iana-sip-option-tags
│   ├── iana-ssh-encryption-algs
│   ├── iana-ssh-key-exchange-algs
│   ├── iana-ssh-mac-algs
│   ├── iana-ssh-public-key-algs
│   ├── iana-tls-cipher-suite-algs
│   └── iana-tunnel-type
├── iana-iax-parameters
├── iana-icalendar
├── iana-ice
├── iana-icmp-parameters
├── iana-icmpv6-parameters
├── iana-idmef-parameters
├── iana-idxp-options
├── iana-ieee-802-numbers
├── iana-iesg-recognized-organizations
├── iana-igmp-type-numbers
├── iana-igp-parameters
├── iana-ikev2-parameters
├── iana-im-srv-labels
├── iana-imap
│   ├── imap-annotate-extension
│   ├── imap-capabilities
│   ├── imap-jmap-keywords
│   ├── imap-list-extended
│   ├── imap-mailbox-name-attributes
│   ├── imap-metadata
│   ├── imap-response-codes
│   └── imap-threading-algorithms
├── iana-imdn
├── iana-inst-man-values
├── iana-integ-serv
├── iana-internet-date-time-format
├── iana-ioam
│   ├── ioam
│   └── ioam-capabilities
├── iana-iodef
├── iana-iodef2
├── iana-iotp-codes
├── iana-ip
│   ├── ip-over-IEEE1394
│   ├── ip-parameters
│   └── ip-xns-mapping
├── iana-ipfix
├── iana-ipp-registrations
├── iana-ips-protocols
├── iana-ipsec-registry
├── iana-ipseckey-rr-parameters
├── iana-ipv4
│   ├── ipv4-address-space
│   └── ipv4-recovered-address-space
├── iana-ipv6
│   ├── ipv6-address-space
│   ├── ipv6-anycast-addresses
│   ├── ipv6-interface-ids
│   ├── ipv6-multicast-addresses
│   ├── ipv6-parameters
│   ├── ipv6-routeralert-values
│   ├── ipv6-tla-assignments
│   └── ipv6-unicast-address-assignments
├── iana-isakmp-registry
├── iana-iscsi-parameters
├── iana-isis
│   ├── isis-mt-parameters
│   ├── isis-pdu
│   └── isis-tlv-codepoints
├── iana-isns-parameters
├── iana-jmap
├── iana-jms-uri-variants
├── iana-jose
├── iana-jscalendar
├── iana-jscontact
├── iana-jsonpath
├── iana-jwt
├── iana-kerberos
│   ├── kerberos-parameters
│   └── kerberos-v-gss-api
├── iana-keynote
├── iana-keytable
├── iana-kink-parameters
├── iana-l2tp-parameters
├── iana-lang
│   ├── lang-subtags-templates
│   └── lang-tag-apps
├── iana-language
│   ├── language-subtag-registry
│   └── language-subtags-tags-extensions
├── iana-lct-header-extensions
├── iana-ldap-parameters
├── iana-ldp-namespaces
├── iana-leighton-micali-signatures
├── iana-lgr-dispositions
├── iana-link-relations
├── iana-lisp-parameters
├── iana-lmp-parameters
├── iana-loa-profiles
├── iana-locally-served-dns-zones
├── iana-location-type-registry
├── iana-lost-location-profiles
├── iana-ltp-parameters
├── iana-machine-names
├── iana-madcap-parameters
├── iana-mail-encoding
├── iana-managesieve
├── iana-manet-parameters
├── iana-marf-parameters
├── iana-markdown-variants
├── iana-masc-parameters
├── iana-masque
├── iana-matroska
├── iana-mdn
├── iana-media
│   ├── media-control-channel
│   ├── media-feature-tags
│   ├── media-type-structured-suffix
│   ├── media-type-sub-parameters
│   ├── media-types
│   └── media-types-parameters
├── iana-megaco-h248
├── iana-message
│   ├── message-header-types
│   ├── message-headers
│   └── message-store-events
├── iana-method-tokens
├── iana-mgcp
│   ├── mgcp-localconnectionoptions
│   └── mgcp-packages
├── iana-mib-modules
├── iana-mih
├── iana-mikey-payloads
├── iana-milnet-parameters
├── iana-mls
├── iana-mobileip-numbers
├── iana-mobility-parameters
├── iana-mpls
│   ├── mpls-id-type
│   ├── mpls-label-values
│   ├── mpls-lsp-ping-parameters
│   ├── mpls-multi-topology-parameters
│   └── mpls-network-actions
├── iana-mrcpv2-parameters
├── iana-mrt
│   ├── mrt
│   └── mrt-parameters
├── iana-msdp-tlv-values
├── iana-msrp-parameters
├── iana-mta-sts
├── iana-mtqp-options
├── iana-mtrace
├── iana-mud
├── iana-multicast
│   ├── multicast-acquisition
│   ├── multicast-addresses
│   └── multicast-ping
├── iana-named-information
├── iana-netconf-capability-urns
├── iana-netnews-parameters
├── iana-nfsv4
│   ├── nfsv4-device-id-notifications
│   ├── nfsv4-named-attributes
│   ├── nfsv4-path-variables
│   └── nfsv4-recallable-object-types
├── iana-nhrp-types
├── iana-nlpids
├── iana-nntp-parameters
├── iana-norm-parameters
├── iana-notification-capability-parameters
├── iana-novell-sap-numbers
├── iana-nsh
├── iana-nslp-parameters
├── iana-ntp-parameters
├── iana-nts
├── iana-nvo3
├── iana-oauth-parameters
├── iana-olsr-types
├── iana-oncrpc-ids
├── iana-openpgp
├── iana-operating-system-names
├── iana-opes
├── iana-opus-channel-mapping-families
├── iana-os-specific-parameters
├── iana-osi-nsapa-numbers
├── iana-ospf
│   ├── ospf-authentication-codes
│   ├── ospf-dd-packet-flags
│   ├── ospf-lls-tlvs
│   ├── ospf-mt-routing
│   ├── ospf-opaque-types
│   ├── ospf-parameters
│   ├── ospf-sig-alg
│   └── ospf-traffic-eng-tlvs
├── iana-ospfv2-parameters
├── iana-ospfv3
│   ├── ospfv3-authentication-trailer-options
│   └── ospfv3-parameters
├── iana-otp-parameters
├── iana-owamp-parameters
├── iana-pa-tnc-parameters
├── iana-pal-package-types
├── iana-pana-parameters
├── iana-params
├── iana-passport
├── iana-paws
├── iana-pb-tnc-parameters
├── iana-pcap
├── iana-pcep
├── iana-pcim-oids
├── iana-pcp-parameters
├── iana-perc
├── iana-performance-metrics
├── iana-perm-mcast-groupids
├── iana-phbid-codes
├── iana-pidf-lo-civic-address-considerations
├── iana-pim-parameters
├── iana-pkix-parameters
├── iana-pnfs-layout-types
├── iana-pop3-extension-mechanism
├── iana-posh-service-names
├── iana-post-stack-first-nibble
├── iana-power-state-sets
├── iana-ppp-numbers
├── iana-pppoe-parameters
├── iana-ppsp-tp
├── iana-ppspp
├── iana-precis-parameters
├── iana-preemption-namespace
├── iana-pres-srv-labels
├── iana-printer-language-numbers
├── iana-privacy-pass
├── iana-profile-uris
├── iana-pronet80-type-numbers
├── iana-prophet
├── iana-protocol-numbers
├── iana-provisional-standard-media-types
├── iana-proxy-languages
├── iana-psamp-parameters
├── iana-pskc
├── iana-pt
│   ├── pt-eap
│   └── pt-tls-parameters
├── iana-public-data-network-numbers
├── iana-pvds
├── iana-pwe3-parameters
├── iana-qspec
├── iana-quic
├── iana-radius-types
├── iana-rats
├── iana-rdap
│   ├── rdap-asn
│   ├── rdap-dns
│   ├── rdap-extensions
│   ├── rdap-ipv4
│   ├── rdap-ipv6
│   ├── rdap-json-values
│   ├── rdap-object-tags
│   ├── rdap-provider-object-tags
│   ├── rdap-query-purpose-values
│   ├── rdap-reverse-search
│   └── rdap-reverse-search-mapping
├── iana-rddp
├── iana-registrar-ids
├── iana-reload
├── iana-remote-direct-data-placement
├── iana-reputation-parameters
├── iana-restconf-capability-urns
├── iana-rfb
├── iana-rid
├── iana-rift
├── iana-rip-types
├── iana-rmt-fec-parameters
├── iana-rohc
│   ├── rohc-pro-ids
│   └── rohc-sub-ids
├── iana-rolie
├── iana-roughtime
├── iana-route-distinguisher-types
├── iana-rpc
│   ├── rpc-authentication-numbers
│   ├── rpc-netids
│   └── rpc-program-numbers
├── iana-rpcbind-transport-parameters
├── iana-rpki
├── iana-rpl
│   ├── rpl
│   └── rpl-routing-metric-constraint
├── iana-rserpool-parameters
├── iana-rsip-parameters
├── iana-rsvp
│   ├── rsvp-parameters
│   ├── rsvp-te-oam
│   └── rsvp-te-parameters
├── iana-rtcp
│   ├── rtcp-xr-block-types
│   └── rtcp-xr-sdp-parameters
├── iana-rtfm
├── iana-rtp-parameters
├── iana-rtsp-parameters
├── iana-rtspv2-parameters
├── iana-rue
├── iana-s
│   ├── s-naptr-parameters
│   └── s-pmsi-parameters
├── iana-safi-namespace
├── iana-sam-baseline
├── iana-sasl-mechanisms
├── iana-scep
├── iana-scim
├── iana-scsp-numbers
├── iana-sctp-parameters
├── iana-sdf
├── iana-sdp
│   ├── sdp-parameters
│   └── sdp-security-descriptions
├── iana-sdxf-parameters
├── iana-sec-ext-enum
├── iana-secevent
├── iana-security
│   ├── security-label-format-selection
│   └── security-txt-fields
├── iana-segment-routing
├── iana-senml
├── iana-service
│   ├── service-codes
│   ├── service-function-chaining-service-function-types
│   └── service-names-port-numbers
├── iana-sfc-active-oam
├── iana-sframe
├── iana-sgmp-vendor-specific-codes
├── iana-shim6
├── iana-sieve
│   ├── sieve-environment-items
│   ├── sieve-extensions
│   └── sieve-notification
├── iana-sig-alg-numbers
├── iana-sigcomp-namespace
├── iana-sigtran-adapt
├── iana-sip
│   ├── sip-clf-parameters
│   ├── sip-events
│   ├── sip-parameters
│   ├── sip-precond-types
│   ├── sip-priv-values
│   └── sip-table
├── iana-slp-da-service
├── iana-smi-numbers
├── iana-smtp
│   ├── smtp
│   └── smtp-enhanced-status-codes
├── iana-snmp-number-spaces
├── iana-snoop-datalink-types
├── iana-socks-methods
├── iana-software-id
├── iana-special
│   ├── special-registry
│   └── special-use-domain-names
├── iana-spf-parameters
├── iana-spi-numbers
├── iana-sppf-orgidtype-namespace
├── iana-srtp-protection
├── iana-ssh-parameters
├── iana-stamp-tlv-types
├── iana-starttls-validation-result-types
├── iana-stringprep-profiles
├── iana-stun-parameters
├── iana-suit
├── iana-sun-rpc-numbers
├── iana-svrloc
│   ├── svrloc-cryptographic-bsd
│   ├── svrloc-error-numbers
│   ├── svrloc-extensions
│   ├── svrloc-function-ids
│   └── svrloc-templates
├── iana-syslog-parameters
├── iana-tcp
│   ├── tcp-convert-protocol-parameters
│   ├── tcp-header-flags
│   └── tcp-parameters
├── iana-te
│   ├── te-link-capabilities
│   └── te-types
├── iana-teap-parameters
├── iana-teep
├── iana-tel-uri-parameters
├── iana-telnet-options
├── iana-terminal-type-names
├── iana-tesla-parameters
├── iana-text-directory-registrations
├── iana-tls
│   ├── tls-ech-configuration-extensions
│   ├── tls-extensiontype-values
│   └── tls-parameters
├── iana-token-binding-protocol
├── iana-top-level-media-types
├── iana-trailer-types
├── iana-trans
├── iana-transfer-encodings
├── iana-trill-parameters
├── iana-trip-parameters
├── iana-tsig-algorithm-names
├── iana-tunnel-setup-protocol
├── iana-twamp-parameters
├── iana-tzdist
│   ├── tzdist-actions
│   └── tzdist-identifiers
├── iana-uaddr-formats
├── iana-udp
├── iana-udpstp
├── iana-ule-next-headers
├── iana-uri-schemes
├── iana-urlauth
│   ├── urlauth-access-ids
│   └── urlauth-authorization-mechanism-registry
├── iana-urn
│   ├── urn-namespaces
│   └── urn-serviceid-labels
├── iana-uuid
├── iana-vcard-elements
├── iana-vcdiff-comp-ids
├── iana-version
│   ├── version-numbers
│   └── version-symbols
├── iana-vnc-uri
├── iana-vot
├── iana-wave-avi-codec-registry
├── iana-webauthn
├── iana-webfinger
├── iana-webpush-parameters
├── iana-websocket
├── iana-well-known-uris
├── iana-wesp-flags
├── iana-whip
├── iana-x25-type-numbers
├── iana-xcap-parameters
├── iana-xml
│   ├── xml-ns-provided-by
│   ├── xml-registry
│   └── xml-security-uris
├── iana-xmlers
├── iana-xmss-extended-hash-based-signatures
├── iana-xns-protocol-types
└── iana-yang
    ├── yang-geographic-location
    ├── yang-module-tags
    ├── yang-parameters
    └── yang-sid
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
