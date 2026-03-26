---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#13-standard
chunk_level: standard
chunk_type: prose
heading: API objects
token_count: 424
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