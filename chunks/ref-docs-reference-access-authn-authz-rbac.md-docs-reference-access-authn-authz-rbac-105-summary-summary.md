---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#105-summary
chunk_level: summary
chunk_type: table
heading: Default roles and role bindings
token_count: 128
summary: |Default ClusterRole|Default ClusterRoleBinding|Description| |**system:auth-delegator**|None|Allows delegated authentication and authorization checks. This is commonly used by add-on API servers for...
---

|Default ClusterRole|Default ClusterRoleBinding|Description|
|**system:auth-delegator**|None|Allows delegated authentication and authorization checks.
This is commonly used by add-on API servers for unified authentication and authorization.|
|**system:heapster**|None|Role for the [Heapster](https://github.com/kubernetes/heapster) component (deprecated).|
|**system:kube-aggregator**|None|Role for the [kube-aggregator](https://github.com/kubernetes/kube-aggregator) component.|
|**system:kube-dns**|**kube-dns** service account in the