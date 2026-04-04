---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#25-standard
chunk_level: standard
chunk_type: prose
heading: Cluster trust bundles
token_count: 380
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