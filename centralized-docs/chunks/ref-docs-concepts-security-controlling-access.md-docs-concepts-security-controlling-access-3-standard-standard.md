---
doc_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access
chunk_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access#3-standard
chunk_level: standard
chunk_type: prose
heading: Authorization
token_count: 508
summary: ## Authorization After the request is authenticated as coming from a specific user, the request must be authorized. This is shown as step**2**in the diagram. A request must include the username of...
---

## Authorization
After the request is authenticated as coming from a specific user, the request must
be authorized. This is shown as step**2**in the diagram.
A request must include the username of the requester, the requested action, and
the object affected by the action. The request is authorized if an existing policy
declares that the user has permissions to complete the requested action.
For example, if Bob has the policy below, then he can read pods only in the namespace `projectCaribou`:
```
`{
"apiVersion": "abac.authorization.kubernetes.io/v1beta1",
"kind": "Policy",
"spec": {
"user": "bob",
"namespace": "projectCaribou",
"resource": "pods",
"readonly": true
}
}
`
```
If Bob makes the following request, the request is authorized because he is
allowed to read objects in the `projectCaribou` namespace:
```
`{
"apiVersion": "authorization.k8s.io/v1beta1",
"kind": "SubjectAccessReview",
"spec": {
"resourceAttributes": {
"namespace": "projectCaribou",
"verb": "get",
"group": "unicorn.example.org",
"resource": "pods"
}
}
}
`
```
If Bob makes a request to write (`create` or `update`) to the objects in the
`projectCaribou` namespace, his authorization is denied. If Bob makes a request
to read (`get`) objects in a different namespace such as `projectFish`, then his authorization is denied.
Kubernetes authorization requires that you use common REST attributes to interact
with existing organization-wide or cloud-provider-wide access control systems.
It is important to use REST formatting because these control systems might
interact with other APIs besides the Kubernetes API.
Kubernetes supports multiple authorization modules, such as ABAC mode, RBAC Mode,
and Webhook mode. When an administrator creates a cluster, they configure the
authorization modules that should be used in the API server. If more than one
authorization modules are configured, Kubernetes checks each module, and if
any module authorizes the request, then the request can proceed. If all of
the modules deny the request, then the request is denied (HTTP status code 403).
To learn more about Kubernetes authorization, including details about creating
policies using the supported authorization modules, see [Authorization](/docs/reference/access-authn-authz/authorization/).