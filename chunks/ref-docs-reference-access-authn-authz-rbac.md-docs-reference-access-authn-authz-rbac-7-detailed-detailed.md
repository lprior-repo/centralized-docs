---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#7-detailed
chunk_level: detailed
chunk_type: prose
heading: API objects
token_count: 868
summary: ### Referring to resources In the Kubernetes API, most resources are represented and accessed using a string representation of their object name, such as `pods` for a Pod. RBAC refers to resources...
---

### Referring to resources
In the Kubernetes API, most resources are represented and accessed using a string representation of
their object name, such as `pods` for a Pod. RBAC refers to resources using exactly the same
name that appears in the URL for the relevant API endpoint.
Some Kubernetes APIs involve a
*subresource*, such as the logs for a Pod. A request for a Pod's logs looks like:
```
`GET /api/v1/namespaces/{namespace}/pods/{name}/log
`
```
In this case, `pods` is the namespaced resource for Pod resources, and `log` is a
subresource of `pods`. To represent this in an RBAC role, use a slash (`/`) to
delimit the resource and subresource. To allow a subject to read `pods` and
also access the `log` subresource for each of those Pods, you write:
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
namespace: default
name: pod-and-pod-logs-reader
rules:
- apiGroups: [""]
resources: ["pods", "pods/log"]
verbs: ["get", "list"]
`
```
You can also refer to resources by name for certain requests through the `resourceNames` list.
When specified, requests can be restricted to individual instances of a resource.
Here is an example that restricts its subject to only `get` or `update` a
[ConfigMap](/docs/concepts/configuration/configmap/) named `my-configmap`:
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
namespace: default
name: configmap-updater
rules:
- apiGroups: [""]
# at the HTTP level, the name of the resource for accessing ConfigMap
# objects is "configmaps"
resources: ["configmaps"]
resourceNames: ["my-configmap"]
verbs: ["update", "get"]
`
```
#### Note:
You cannot restrict **deletecollection** or top-level **create** requests by resource name.
For **create**, this limitation is because the name of the new object may not be known at authorization time. However, the **create** limitation applies only to top-level resources, not subresources. For example, you can use the `resourceNames` field with `pods/exec`.
If you restrict **list** or **watch** by `resourceName`, clients must include a `metadata.name` field selector in their **list** or **watch** request (that matches the specified `resourceName`)
in order to be authorized.
For example: `kubectl get configmaps --field-selector=metadata.name=my-configmap`
Rather than referring to individual `resources`, `apiGroups`, and `verbs`,
you can use the wildcard `\*` symbol to refer to all such objects.
For `nonResourceURLs`, you can use the wildcard `\*` as a suffix glob match.
For `resourceNames`, an empty set means that everything is allowed.
Here is an example that allows access to perform any current and future action on
all current and future resources in the `example.com` API group.
This is similar to the built-in `cluster-admin` role.
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
namespace: default
name: example.com-superuser # DO NOT USE THIS ROLE, IT IS JUST AN EXAMPLE
rules:
- apiGroups: ["example.com"]
resources: ["\*"]
verbs: ["\*"]
`
```
#### Caution:
Using wildcards in resource and verb entries could result in overly permissive access being granted
to sensitive resources.
For instance, if a new resource type is added, or a new subresource is added,
or a new custom verb is checked, the wildcard entry automatically grants access, which may be undesirable.
The [principle of least privilege](/docs/concepts/security/rbac-good-practices/#least-privilege)
should be employed, using specific resources and verbs to ensure only the permissions required for the
workload to function correctly are applied.