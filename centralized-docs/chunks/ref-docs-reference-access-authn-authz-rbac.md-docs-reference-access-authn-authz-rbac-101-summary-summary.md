---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#101-summary
chunk_level: summary
chunk_type: table
heading: Default roles and role bindings
token_count: 121
summary: |**system:kube-controller-manager**|**system:kube-controller-manager** user|Allows access to the resources required by the [controller...
---

|**system:kube-controller-manager**|**system:kube-controller-manager** user|Allows access to the resources required by the [controller manager](/docs/reference/command-line-tools-reference/kube-controller-manager/) component.
The permissions required by individual controllers are detailed in the [controller roles](#controller-roles).|
|**system:node**|None|Allows access to resources required by the kubelet, **including read access to all secrets, and write access to all pod status objects**.
You should use the [Node authorizer](/docs/reference/access-authn-authz/node/) and