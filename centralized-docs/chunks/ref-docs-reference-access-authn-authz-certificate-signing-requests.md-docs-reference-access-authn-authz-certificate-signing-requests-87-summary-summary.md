---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#87-summary
chunk_level: summary
chunk_type: prose
heading: Cluster trust bundles
token_count: 128
summary: In Kubernetes 1.35, you must enable the `ClusterTrustBundle` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/) *and* the `certificates.k8s.io/v1alpha1` [API...
---

In Kubernetes 1.35, you must enable the `ClusterTrustBundle`
[feature gate](/docs/reference/command-line-tools-reference/feature-gates/)
*and* the `certificates.k8s.io/v1alpha1`
[API group](/docs/concepts/overview/kubernetes-api/#api-groups-and-versioning) in order to use
this API.
A ClusterTrustBundles is a cluster-scoped object for distributing X.509 trust
anchors (root certificates) to workloads within the cluster. They're designed
to work well with the [signer](#signers) concept from CertificateSigningRequests.