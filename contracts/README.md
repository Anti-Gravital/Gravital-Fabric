# Versioned contracts

Fabric publishes contracts as data. No crate in this repository or in
Anti-Gravital imports a crate from the other; shared types are generated from
these schemas when a consumer needs them (RFC-0001, ADR-0001).

## Layout

```
contracts/
|-- README.md
`-- fabric-node-manifest/
    |-- v1.schema.json          JSON Schema 2020-12, manifest_version 1.0.0
    `-- example-manifest.json   synthetic example that validates against v1
```

## Rules

1. A contract is versioned by directory and by an explicit `*_version` field in
   the data. Adding a field is a minor change; removing or renaming a field is a
   new major version directory.
2. A contract change requires an RFC in the product that owns it. Generated
   code never becomes the source of truth.
3. JSON Schema is used for declarative manifests, Protobuf for messages and
   OpenAPI for HTTP surfaces, as defined in the architecture master.
4. Examples are synthetic. Real host output and its evidence live in
   `docs/audits/`, not in the contract directory.
5. This first contract covers only the local node manifest of RFC-0008. It is
   not a control-plane registration contract and must not be read as one.
