---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#26-standard
chunk_level: standard
chunk_type: table
heading: Default roles and role bindings
token_count: 332
summary: ### Core component roles |Default ClusterRole|Default ClusterRoleBinding|Description| |**system:kube-scheduler**|**system:kube-scheduler** user|Allows access to the resources required by the...
---

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