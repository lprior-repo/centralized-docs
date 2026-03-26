---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#9-standard
chunk_level: standard
chunk_type: prose
heading: Constrained Impersonation
token_count: 340
summary: #### Example: Impersonate a node To allow `node-impersonator` ServiceAccount in `default` namespace impersonating a node named `mynode` to get and list pods: ``` `apiVersion:...
---

#### Example: Impersonate a node
To allow `node-impersonator` ServiceAccount in `default` namespace impersonating
a node named `mynode` to get and list pods:
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: impersonate-node-sa
rules:
- apiGroups: ["authentication.k8s.io"]
resources: ["nodes"]
resourceNames: ["mynode"]
verbs: ["impersonate:arbitrary-node"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: impersonate-list-pods
rules:
- apiGroups: [""]
resources: ["pods"]
verbs:
- "impersonate-on:arbitrary-node:list"
- "impersonate-on:arbitrary-node:get"
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
name: impersonate-node-sa
namespace: default
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: ClusterRole
name: impersonate-node-sa
subjects:
- kind: ServiceAccount
name: node-impersonator
namespace: default
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
name: impersonate-list-pods
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: ClusterRole
name: impersonate-list-pods
subjects:
- kind: ServiceAccount
name: node-impersonator
namespace: default
`
```