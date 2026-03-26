---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#10-standard
chunk_level: standard
chunk_type: prose
heading: Constrained Impersonation
token_count: 432
summary: #### Example: Node agent impersonating the associated node This is a common pattern for node agents (like CNI plugins) that need to read pods on their node without having cluster-wide pod access. ```...
---

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