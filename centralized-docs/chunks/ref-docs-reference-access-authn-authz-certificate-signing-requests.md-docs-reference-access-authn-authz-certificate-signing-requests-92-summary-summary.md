---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#92-summary
chunk_level: summary
chunk_type: prose
heading: Cluster trust bundles
token_count: 78
summary: make these objects public within the cluster, so that they work as intended. If you do not have permission to list cluster trust bundles by default in your cluster, you can impersonate a service...
---

make these objects public within the cluster, so that they work as intended.
If you do not have permission to list cluster trust bundles by default in your
cluster, you can impersonate a service account you have access to in order to
see available ClusterTrustBundles:
```
`kubectl get clustertrustbundles --as='system:serviceaccount:mynamespace:default'
`
```