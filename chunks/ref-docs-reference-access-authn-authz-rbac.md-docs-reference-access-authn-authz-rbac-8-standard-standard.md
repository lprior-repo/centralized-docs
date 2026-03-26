---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#8-standard
chunk_level: standard
chunk_type: prose
heading: API objects
token_count: 335
summary: #### ClusterRole example A ClusterRole can be used to grant the same permissions as a Role. Because ClusterRoles are cluster-scoped, you can also use them to grant access to: * cluster-scoped...
---

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