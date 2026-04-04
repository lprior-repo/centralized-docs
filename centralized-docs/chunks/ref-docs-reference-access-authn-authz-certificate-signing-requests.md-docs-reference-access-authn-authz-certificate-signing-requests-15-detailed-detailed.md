---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#15-detailed
chunk_level: detailed
chunk_type: prose
heading: Cluster trust bundles
token_count: 859
summary: ## Cluster trust bundles FEATURE STATE: `Kubernetes v1.33 [beta]`(disabled by default) #### Note: In Kubernetes 1.35, you must enable the `ClusterTrustBundle` [feature...
---

## Cluster trust bundles
FEATURE STATE:
`Kubernetes v1.33 [beta]`(disabled by default)
#### Note:
In Kubernetes 1.35, you must enable the `ClusterTrustBundle`
[feature gate](/docs/reference/command-line-tools-reference/feature-gates/)
*and* the `certificates.k8s.io/v1alpha1`
[API group](/docs/concepts/overview/kubernetes-api/#api-groups-and-versioning) in order to use
this API.
A ClusterTrustBundles is a cluster-scoped object for distributing X.509 trust
anchors (root certificates) to workloads within the cluster. They're designed
to work well with the [signer](#signers) concept from CertificateSigningRequests.
ClusterTrustBundles can be used in two modes:
[signer-linked](#ctb-signer-linked) and [signer-unlinked](#ctb-signer-unlinked).
### Common properties and validation
All ClusterTrustBundle objects have strong validation on the contents of their
`trustBundle` field. That field must contain one or more X.509 certificates,
DER-serialized, each wrapped in a PEM `CERTIFICATE` block. The certificates
must parse as valid X.509 certificates.
Esoteric PEM features like inter-block data and intra-block headers are either
rejected during object validation, or can be ignored by consumers of the object.
Additionally, consumers are allowed to reorder the certificates in
the bundle with their own arbitrary but stable ordering.
ClusterTrustBundle objects should be considered world-readable within the
cluster. If your cluster uses [RBAC](/docs/reference/access-authn-authz/rbac/)
authorization, all ServiceAccounts have a default grant that allows them to
**get**, **list**, and **watch** all ClusterTrustBundle objects.
If you use your own authorization mechanism and you have enabled
ClusterTrustBundles in your cluster, you should set up an equivalent rule to
make these objects public within the cluster, so that they work as intended.
If you do not have permission to list cluster trust bundles by default in your
cluster, you can impersonate a service account you have access to in order to
see available ClusterTrustBundles:
```
`kubectl get clustertrustbundles --as='system:serviceaccount:mynamespace:default'
`
```
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
* To create or update a signer-linked ClusterTrustBundle, you must be permitted
to **attest** on the signer (custom authorization verb `attest`,
API group `certificates.k8s.io`; resource path `signers`). You can configure
authorization for the specific resource name
`&lt;signerNameDomain&gt;/&lt;signerNamePath&gt;` or match a pattern such as
`&lt;signerNameDomain&gt;/\*`.
* Signer-linked ClusterTrustBundles **must** be named with a prefix derived from
their `spec.signerName` field. Slashes (`/`) are replaced with colons (`:`),
and a final colon is appended. This is followed by an arbitrary name. For
example, the signer `example.com/mysigner` can be linked to a
ClusterTrustBundle `example.com:mysigner:&lt;arbitrary-name&gt;`.
Signer-linked ClusterTrustBundles will typically be consumed in workloads
by a combination of a
[field selector](/docs/concepts/overview/working-with-objects/field-selectors/) on the signer name, and a separate
[label selector](/docs/concepts/overview/working-with-objects/labels/#label-selectors).