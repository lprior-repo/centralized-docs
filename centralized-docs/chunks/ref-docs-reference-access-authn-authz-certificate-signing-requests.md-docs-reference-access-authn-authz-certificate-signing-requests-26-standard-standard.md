---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#26-standard
chunk_level: standard
chunk_type: prose
heading: Cluster trust bundles
token_count: 277
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
They are primarily intended for cluster configuration use cases.
Each signer-unlinked ClusterTrustBundle is an independent object, in contrast to the
customary grouping behavior of signer-linked ClusterTrustBundles.
Signer-unlinked ClusterTrustBundles have no `attest` verb requirement.
Instead, you control access to them directly using the usual mechanisms,
such as role-based access control.
To distinguish them from signer-linked ClusterTrustBundles, the names of
signer-unlinked ClusterTrustBundles **must not** contain a colon (`:`).
### Accessing ClusterTrustBundles from pods
FEATURE STATE:
`Kubernetes v1.33 [beta]`(disabled by default)
The contents of ClusterTrustBundles can be injected into the container filesystem, similar to ConfigMaps and Secrets.
See the [clusterTrustBundle projected volume source](/docs/concepts/storage/projected-volumes/#clustertrustbundle) for more details.