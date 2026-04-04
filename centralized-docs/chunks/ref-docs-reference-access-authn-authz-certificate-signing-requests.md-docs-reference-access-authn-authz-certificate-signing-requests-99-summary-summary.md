---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#99-summary
chunk_level: summary
chunk_type: prose
heading: Cluster trust bundles
token_count: 78
summary: ### Accessing ClusterTrustBundles from pods FEATURE STATE: `Kubernetes v1.33 [beta]`(disabled by default) The contents of ClusterTrustBundles can be injected into the container filesystem, similar to...
---

### Accessing ClusterTrustBundles from pods
FEATURE STATE:
`Kubernetes v1.33 [beta]`(disabled by default)
The contents of ClusterTrustBundles can be injected into the container filesystem, similar to ConfigMaps and Secrets.
See the [clusterTrustBundle projected volume source](/docs/concepts/storage/projected-volumes/#clustertrustbundle) for more details.