---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#93-summary
chunk_level: summary
chunk_type: prose
heading: Cluster trust bundles
token_count: 117
summary: ### Signer-linked ClusterTrustBundles Signer-linked ClusterTrustBundles are associated with a *signer name*, like this: ``` `apiVersion: certificates.k8s.io/v1alpha1 kind: ClusterTrustBundle...
---

### Signer-linked ClusterTrustBundles
Signer-linked ClusterTrustBundles are associated with a *signer name*, like this:
```
`apiVersion: certificates.k8s.io/v1alpha1
kind: ClusterTrustBundle
metadata:
name: example.com:mysigner:foo
spec:
signerName: example.com/mysigner
trustBundle: "&lt;... PEM data ...&gt;"
`
```
These ClusterTrustBundles are intended to be maintained by a signer-specific
controller in the cluster, so they have several security features: