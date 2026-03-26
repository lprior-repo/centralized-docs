---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#149-summary
chunk_level: summary
chunk_type: prose
heading: Write access for EndpointSlices
token_count: 128
summary: Kubernetes clusters created before Kubernetes v1.22 include write access to EndpointSlices (and the now-deprecated Endpoints API) in the aggregated \"edit\" and \"admin\" roles. As a mitigation for...
---

Kubernetes clusters created before Kubernetes v1.22 include write access to
EndpointSlices (and the now-deprecated Endpoints API) in the aggregated "edit" and "admin" roles.
As a mitigation for [CVE-2021-25740](https://github.com/kubernetes/kubernetes/issues/103675),
this access is not part of the aggregated roles in clusters that you create using
Kubernetes v1.22 or later.
Existing clusters that have been upgraded to Kubernetes v1.22 will not be
subject to this change. The [CVE
announcement](https://github.com/kubernetes/kubernetes/issues/103675) includes