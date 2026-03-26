---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#150-summary
chunk_level: summary
chunk_type: prose
heading: Write access for EndpointSlices
token_count: 104
summary: subject to this change. The [CVE announcement](https://github.com/kubernetes/kubernetes/issues/103675) includes guidance for restricting this access in existing clusters. If you want new clusters to...
---

subject to this change. The [CVE
announcement](https://github.com/kubernetes/kubernetes/issues/103675) includes
guidance for restricting this access in existing clusters.
If you want new clusters to retain this level of access in the aggregated roles,
you can create the following ClusterRole:
[`access/endpoints-aggregated.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/endpoints-aggregated.yaml)![](/images/copycode.svg "Copy access/endpoints-aggregated.yaml to clipboard")