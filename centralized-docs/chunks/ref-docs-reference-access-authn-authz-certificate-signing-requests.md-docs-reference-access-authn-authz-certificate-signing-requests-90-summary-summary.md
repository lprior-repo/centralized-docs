---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#90-summary
chunk_level: summary
chunk_type: prose
heading: Cluster trust bundles
token_count: 128
summary: All ClusterTrustBundle objects have strong validation on the contents of their `trustBundle` field. That field must contain one or more X.509 certificates, DER-serialized, each wrapped in a PEM...
---

All ClusterTrustBundle objects have strong validation on the contents of their
`trustBundle` field. That field must contain one or more X.509 certificates,
DER-serialized, each wrapped in a PEM `CERTIFICATE` block. The certificates
must parse as valid X.509 certificates.
Esoteric PEM features like inter-block data and intra-block headers are either
rejected during object validation, or can be ignored by consumers of the object.
Additionally, consumers are allowed to reorder the certificates in
the bundle with their own arbitrary but stable ordering.
ClusterTrustBundle objects should be considered world-readable within the
cluster. If your cluster uses