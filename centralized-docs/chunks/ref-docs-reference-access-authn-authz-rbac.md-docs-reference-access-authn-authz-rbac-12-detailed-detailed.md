---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#12-detailed
chunk_level: detailed
chunk_type: table
heading: Default roles and role bindings
token_count: 927
summary: ### User-facing roles Some of the default ClusterRoles are not `system:` prefixed. These are intended to be user-facing roles. They include super-user roles (`cluster-admin`), roles intended to be...
---

### User-facing roles
Some of the default ClusterRoles are not `system:` prefixed. These are intended to be user-facing roles.
They include super-user roles (`cluster-admin`), roles intended to be granted cluster-wide
using ClusterRoleBindings, and roles intended to be granted within particular
namespaces using RoleBindings (`admin`, `edit`, `view`).
User-facing ClusterRoles use [ClusterRole aggregation](#aggregated-clusterroles) to allow admins to include
rules for custom resources on these ClusterRoles. To add rules to the `admin`, `edit`, or `view` roles, create
a ClusterRole with one or more of the following labels:
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
### Core component roles
|Default ClusterRole|Default ClusterRoleBinding|Description|
|**system:kube-scheduler**|**system:kube-scheduler** user|Allows access to the resources required by the [scheduler](/docs/reference/command-line-tools-reference/kube-scheduler/) component.|
|**system:volume-scheduler**|**system:kube-scheduler** user|Allows access to the volume resources required by the kube-scheduler component.|
|**system:kube-controller-manager**|**system:kube-controller-manager** user|Allows access to the resources required by the [controller manager](/docs/reference/command-line-tools-reference/kube-controller-manager/) component.
The permissions required by individual controllers are detailed in the [controller roles](#controller-roles).|
|**system:node**|None|Allows access to resources required by the kubelet, **including read access to all secrets, and write access to all pod status objects**.
You should use the [Node authorizer](/docs/reference/access-authn-authz/node/) and [NodeRestriction admission plugin](/docs/reference/access-authn-authz/admission-controllers/#noderestriction) instead of the system:node role, and allow granting API access to kubelets based on the Pods scheduled to run on them.
The system:node role only exists for compatibility with Kubernetes clusters upgraded from versions prior to v1.8.
|
|**system:node-proxier**|**system:kube-proxy** user|Allows access to the resources required by the [kube-proxy](/docs/reference/command-line-tools-reference/kube-proxy/) component.|