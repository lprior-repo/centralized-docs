---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#12-standard
chunk_level: standard
chunk_type: prose
heading: API objects
token_count: 454
summary: #### ClusterRoleBinding example To grant permissions across a whole cluster, you can use a ClusterRoleBinding. The following ClusterRoleBinding allows any user in the group \"manager\" to read secrets...
---

#### ClusterRoleBinding example
To grant permissions across a whole cluster, you can use a ClusterRoleBinding.
The following ClusterRoleBinding allows any user in the group "manager" to read
secrets in any namespace.
[`access/simple-clusterrolebinding.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/simple-clusterrolebinding.yaml)![](/images/copycode.svg "Copy access/simple-clusterrolebinding.yaml to clipboard")
```
`apiVersion: rbac.authorization.k8s.io/v1
# This cluster role binding allows anyone in the "manager" group to read secrets in any namespace.
kind: ClusterRoleBinding
metadata:
name: read-secrets-global
subjects:
- kind: Group
name: manager # Name is case sensitive
apiGroup: rbac.authorization.k8s.io
roleRef:
kind: ClusterRole
name: secret-reader
apiGroup: rbac.authorization.k8s.io
`
```
After you create a binding, you cannot change the Role or ClusterRole that it refers to.
If you try to change a binding's `roleRef`, you get a validation error. If you do want
to change the `roleRef` for a binding, you need to remove the binding object and create
a replacement.
There are two reasons for this restriction:
1. Making `roleRef` immutable allows granting someone `update` permission on an existing binding
object, so that they can manage the list of subjects, without being able to change
the role that is granted to those subjects.
2. A binding to a different role is a fundamentally different binding.
Requiring a binding to be deleted/recreated in order to change the `roleRef`
ensures the full list of subjects in the binding is intended to be granted
the new role (as opposed to enabling or accidentally modifying only the roleRef
without verifying all of the existing subjects should be given the new role's
permissions).
The `kubectl auth reconcile` command-line utility creates or updates a manifest file containing RBAC objects,
and handles deleting and recreating binding objects if required to change the role they refer to.
See [command usage and examples](#kubectl-auth-reconcile) for more information.