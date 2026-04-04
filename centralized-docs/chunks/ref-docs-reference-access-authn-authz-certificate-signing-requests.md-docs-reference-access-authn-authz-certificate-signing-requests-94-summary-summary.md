---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#94-summary
chunk_level: summary
chunk_type: prose
heading: Cluster trust bundles
token_count: 100
summary: * To create or update a signer-linked ClusterTrustBundle, you must be permitted to **attest** on the signer (custom authorization verb `attest`, API group `certificates.k8s.io`; resource path...
---

* To create or update a signer-linked ClusterTrustBundle, you must be permitted
to **attest** on the signer (custom authorization verb `attest`,
API group `certificates.k8s.io`; resource path `signers`). You can configure
authorization for the specific resource name
`&lt;signerNameDomain&gt;/&lt;signerNamePath&gt;` or match a pattern such as
`&lt;signerNameDomain&gt;/\*`.