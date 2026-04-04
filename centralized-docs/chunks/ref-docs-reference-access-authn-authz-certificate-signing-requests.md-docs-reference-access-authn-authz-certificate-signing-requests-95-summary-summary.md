---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#95-summary
chunk_level: summary
chunk_type: prose
heading: Cluster trust bundles
token_count: 113
summary: * Signer-linked ClusterTrustBundles **must** be named with a prefix derived from their `spec.signerName` field. Slashes (`/`) are replaced with colons (`:`), and a final colon is appended. This is...
---

* Signer-linked ClusterTrustBundles **must** be named with a prefix derived from
their `spec.signerName` field. Slashes (`/`) are replaced with colons (`:`),
and a final colon is appended. This is followed by an arbitrary name. For
example, the signer `example.com/mysigner` can be linked to a
ClusterTrustBundle `example.com:mysigner:&lt;arbitrary-name&gt;`.
Signer-linked ClusterTrustBundles will typically be consumed in workloads
by a combination of a