---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#96-summary
chunk_level: summary
chunk_type: prose
heading: Cluster trust bundles
token_count: 71
summary: . Signer-linked ClusterTrustBundles will typically be consumed in workloads by a combination of a [field selector](/docs/concepts/overview/working-with-objects/field-selectors/) on the signer name,...
---

.
Signer-linked ClusterTrustBundles will typically be consumed in workloads
by a combination of a
[field selector](/docs/concepts/overview/working-with-objects/field-selectors/) on the signer name, and a separate
[label selector](/docs/concepts/overview/working-with-objects/labels/#label-selectors).