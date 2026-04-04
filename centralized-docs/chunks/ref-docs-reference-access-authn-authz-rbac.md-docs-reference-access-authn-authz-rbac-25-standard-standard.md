---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#25-standard
chunk_level: standard
chunk_type: table
heading: Default roles and role bindings
token_count: 459
summary: ``` `metadata: labels: rbac.authorization.k8s.io/aggregate-to-admin: \"true\" rbac.authorization.k8s.io/aggregate-to-edit: \"true\" rbac.authorization.k8s.io/aggregate-to-view: \"true\" ` ``` |Default...
---

```
`metadata:
labels:
rbac.authorization.k8s.io/aggregate-to-admin: "true"
rbac.authorization.k8s.io/aggregate-to-edit: "true"
rbac.authorization.k8s.io/aggregate-to-view: "true"
`
```
|Default ClusterRole|Default ClusterRoleBinding|Description|
|**cluster-admin**|**system:masters** group|Allows super-user access to perform any action on any resource.
When used in a **ClusterRoleBinding**, it gives full control over every resource in the cluster and in all namespaces.
When used in a **RoleBinding**, it gives full control over every resource in the role binding's namespace, including the namespace itself.|
|**admin**|None|Allows admin access, intended to be granted within a namespace using a **RoleBinding**.
If used in a **RoleBinding**, allows read/write access to most resources in a namespace,
including the ability to create roles and role bindings within the namespace.
This role does not allow write access to resource quota or to the namespace itself.
This role also does not allow write access to EndpointSlices in clusters created
using Kubernetes v1.22+. More information is available in the
["Write Access for EndpointSlices" section](#write-access-for-endpoints).
|
|**edit**|None|Allows read/write access to most objects in a namespace.
This role does not allow viewing or modifying roles or role bindings.
However, this role allows accessing Secrets and running Pods as any ServiceAccount in
the namespace, so it can be used to gain the API access levels of any ServiceAccount in
the namespace. This role also does not allow write access to EndpointSlices in
clusters created using Kubernetes v1.22+. More information is available in the
["Write Access for EndpointSlices" section](#write-access-for-endpoints).
|
|**view**|None|Allows read-only access to see most objects in a namespace.
It does not allow viewing roles or role bindings.
This role does not allow viewing Secrets, since reading
the contents of Secrets enables access to ServiceAccount credentials
in the namespace, which would allow API access as any ServiceAccount
in the namespace (a form of privilege escalation).
|