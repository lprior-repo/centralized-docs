---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#7-standard
chunk_level: standard
chunk_type: prose
heading: API objects
token_count: 360
summary: ### Role and ClusterRole An RBAC *Role* or *ClusterRole* contains rules that represent a set of permissions. Permissions are purely additive (there are no \"deny\" rules). A Role always sets...
---

### Role and ClusterRole
An RBAC *Role* or *ClusterRole* contains rules that represent a set of permissions.
Permissions are purely additive (there are no "deny" rules).
A Role always sets permissions within a particular [namespace](/docs/concepts/overview/working-with-objects/namespaces);
when you create a Role, you have to specify the namespace it belongs in.
ClusterRole, by contrast, is a non-namespaced resource. The resources have different names (Role
and ClusterRole) because a Kubernetes object always has to be either namespaced or not namespaced;
it can't be both.
ClusterRoles have several uses. You can use a ClusterRole to:
1. define permissions on namespaced resources and be granted access within individual namespace(s)
2. define permissions on namespaced resources and be granted access across all namespaces
3. define permissions on cluster-scoped resources
If you want to define a role within a namespace, use a Role; if you want to define
a role cluster-wide, use a ClusterRole.
#### Role example
Here's an example Role in the "default" namespace that can be used to grant read access to
[pods](/docs/concepts/workloads/pods/):
[`access/simple-role.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/simple-role.yaml)![](/images/copycode.svg "Copy access/simple-role.yaml to clipboard")
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
namespace: default
name: pod-reader
rules:
- apiGroups: [""] # "" indicates the core API group
resources: ["pods"]
verbs: ["get", "watch", "list"]
`
```