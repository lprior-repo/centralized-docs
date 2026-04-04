---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#16-summary
chunk_level: summary
chunk_type: prose
heading: Kubelet authentication
token_count: 106
summary: * verb=\*, resource=nodes, subresource=metrics * verb=\*, resource=nodes, subresource=configz * verb=\*, resource=nodes, subresource=healthz * verb=\*, resource=nodes, subresource=pods If [RBAC...
---

* verb=\*, resource=nodes, subresource=metrics
* verb=\*, resource=nodes, subresource=configz
* verb=\*, resource=nodes, subresource=healthz
* verb=\*, resource=nodes, subresource=pods
If [RBAC authorization](/docs/reference/access-authn-authz/rbac/) is used,
enabling this gate also ensure that the builtin `system:kubelet-api-admin` ClusterRole
is updated with permissions to access all the above mentioned subresources.