---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#97-summary
chunk_level: summary
chunk_type: prose
heading: Cluster trust bundles
token_count: 89
summary: ### Signer-unlinked ClusterTrustBundles Signer-unlinked ClusterTrustBundles have an empty `spec.signerName` field, like this: ``` `apiVersion: certificates.k8s.io/v1alpha1 kind: ClusterTrustBundle...
---

### Signer-unlinked ClusterTrustBundles
Signer-unlinked ClusterTrustBundles have an empty `spec.signerName` field, like this:
```
`apiVersion: certificates.k8s.io/v1alpha1
kind: ClusterTrustBundle
metadata:
name: foo
spec:
# no signerName specified, so the field is blank
trustBundle: "&lt;... PEM data ...&gt;"
`
```