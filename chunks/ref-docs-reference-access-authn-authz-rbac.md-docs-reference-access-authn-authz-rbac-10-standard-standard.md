---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#10-standard
chunk_level: standard
chunk_type: prose
heading: API objects
token_count: 433
summary: #### RoleBinding examples Here is an example of a RoleBinding that grants the \"pod-reader\" Role to the user \"jane\" within the \"default\" namespace. This allows \"jane\" to read pods in the \"default\"...
---

#### RoleBinding examples
Here is an example of a RoleBinding that grants the "pod-reader" Role to the user "jane"
within the "default" namespace.
This allows "jane" to read pods in the "default" namespace.
[`access/simple-rolebinding-with-role.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/simple-rolebinding-with-role.yaml)![](/images/copycode.svg "Copy access/simple-rolebinding-with-role.yaml to clipboard")
```
`apiVersion: rbac.authorization.k8s.io/v1
# This role binding allows "jane" to read pods in the "default" namespace.
# You need to already have a Role named "pod-reader" in that namespace.
kind: RoleBinding
metadata:
name: read-pods
namespace: default
subjects:
# You can specify more than one "subject"
- kind: User
name: jane # "name" is case sensitive
apiGroup: rbac.authorization.k8s.io
roleRef:
# "roleRef" specifies the binding to a Role / ClusterRole
kind: Role #this must be Role or ClusterRole
name: pod-reader # this must match the name of the Role or ClusterRole you wish to bind to
apiGroup: rbac.authorization.k8s.io
`
```
A RoleBinding can also reference a ClusterRole to grant the permissions defined in that
ClusterRole to resources inside the RoleBinding's namespace. This kind of reference
lets you define a set of common roles across your cluster, then reuse them within
multiple namespaces.
For instance, even though the following RoleBinding refers to a ClusterRole,
"dave" (the subject, case sensitive) will only be able to read Secrets in the "development"
namespace, because the RoleBinding's namespace (in its metadata) is "development".
[`access/simple-rolebinding-with-clusterrole.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/simple-rolebinding-with-clusterrole.yaml)![](/images/copycode.svg "Copy access/simple-rolebinding-with-clusterrole.yaml to clipboard")