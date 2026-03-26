---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#6-detailed
chunk_level: detailed
chunk_type: code
heading: Constrained Impersonation
token_count: 945
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
#### Example: Node agent impersonating the associated node
This is a common pattern for node agents (like CNI plugins) that need to read pods on their node
without having cluster-wide pod access.
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: impersonate-associated-node-identity
rules:
- apiGroups: ["authentication.k8s.io"]
resources: ["nodes"]
verbs: ["impersonate:associated-node"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: impersonate-list-pods-on-node
rules:
- apiGroups: [""]
resources: ["pods"]
verbs:
- "impersonate-on:associated-node:list"
- "impersonate-on:associated-node:get"
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
name: node-agent-impersonate-node
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: ClusterRole
name: impersonate-associated-node-identity
subjects:
- kind: ServiceAccount
name: node-agent
namespace: kube-system
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
name: node-agent-impersonate-list-pods
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: ClusterRole
name: impersonate-list-pods-on-node
subjects:
- kind: ServiceAccount
name: node-agent
namespace: kube-system
`
```
The controller would get the node name using the downward API:
```
`env:
- name: MY\_NODE\_NAME
valueFrom:
fieldRef:
fieldPath: spec.nodeName
`
```
Then configure the kubeconfig to impersonate:
```
`kubeConfig, \_ := clientcmd.BuildConfigFromFlags("", "")
kubeConfig.Impersonate = rest.ImpersonationConfig{
UserName: "system:node:" + os.Getenv("MY\_NODE\_NAME"),
}
`
```
### Using constrained impersonation
From a client perspective, using constrained impersonation is identical to using traditional
impersonation. You use the same impersonation headers:
```
`Impersonate-User: jane.doe@example.com
`
```
Or with kubectl:
```
`kubectl get pods -n default --as=jane.doe@example.com
`
```
The difference is entirely in the authorization checks performed by the API server.
### Working with `impersonate` verb
* If you have existing RBAC rules using the `impersonate` verb, they continue
to function when the feature gate is enabled.
* When an impersonation request is made, the API server first checks for
constrained impersonation permissions. If those checks fail, it falls back to checking the
`impersonate` permission.