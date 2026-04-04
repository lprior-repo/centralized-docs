---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#4-detailed
chunk_level: detailed
chunk_type: prose
heading: API objects
token_count: 836
summary: ## API objects The RBAC API declares four kinds of Kubernetes object: *Role*, *ClusterRole*, *RoleBinding* and *ClusterRoleBinding*. You can describe or amend the RBAC...
---

## API objects
The RBAC API declares four kinds of Kubernetes object: *Role*, *ClusterRole*,
*RoleBinding* and *ClusterRoleBinding*. You can describe or amend the RBAC
[objects](/docs/concepts/overview/working-with-objects/#kubernetes-objects)
using tools such as `kubectl`, just like any other Kubernetes object.
#### Caution:
These objects, by design, impose access restrictions. If you are making changes
to a cluster as you learn, see
[privilege escalation prevention and bootstrapping](#privilege-escalation-prevention-and-bootstrapping)
to understand how those restrictions can prevent you making some changes.
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
#### ClusterRole example
A ClusterRole can be used to grant the same permissions as a Role.
Because ClusterRoles are cluster-scoped, you can also use them to grant access to:
* cluster-scoped resources (like [nodes](/docs/concepts/architecture/nodes/))
* non-resource endpoints (like `/healthz`)
* namespaced resources (like Pods), across all namespaces
For example: you can use a ClusterRole to allow a particular user to run
`kubectl get pods --all-namespaces`
Here is an example of a ClusterRole that can be used to grant read access to
[secrets](/docs/concepts/configuration/secret/) in any particular namespace,
or across all namespaces (depending on how it is [bound](#rolebinding-and-clusterrolebinding)):
[`access/simple-clusterrole.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/simple-clusterrole.yaml)![](/images/copycode.svg "Copy access/simple-clusterrole.yaml to clipboard")
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
# "namespace" omitted since ClusterRoles are not namespaced
name: secret-reader
rules:
- apiGroups: [""]
# at the HTTP level, the name of the resource for accessing Secret
# objects is "secrets"
resources: ["secrets"]
verbs: ["get", "watch", "list"]
`
```
The name of a Role or a ClusterRole object must be a valid
[path segment name](/docs/concepts/overview/working-with-objects/names/#path-segment-names).