---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#100-summary
chunk_level: summary
chunk_type: table
heading: Default roles and role bindings
token_count: 115
summary: |Default ClusterRole|Default ClusterRoleBinding|Description| |**system:kube-scheduler**|**system:kube-scheduler** user|Allows access to the resources required by the...
---

|Default ClusterRole|Default ClusterRoleBinding|Description|
|**system:kube-scheduler**|**system:kube-scheduler** user|Allows access to the resources required by the [scheduler](/docs/reference/command-line-tools-reference/kube-scheduler/) component.|
|**system:volume-scheduler**|**system:kube-scheduler** user|Allows access to the volume resources required by the kube-scheduler component.|
|**system:kube-controller-manager**|**system:kube-controller-manager** user|Allows access to the resources required by the