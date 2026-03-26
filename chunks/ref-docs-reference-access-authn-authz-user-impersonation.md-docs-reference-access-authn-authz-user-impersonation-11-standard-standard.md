---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#11-standard
chunk_level: standard
chunk_type: prose
heading: What's next
token_count: 445
summary: ### Using constrained impersonation From a client perspective, using constrained impersonation is identical to using traditional impersonation. You use the same impersonation headers: ```...
---

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
## Auditing
An audit event is logged for each impersonation request to help track how impersonation is used.
When a request uses constrained impersonation, the audit event includes `authenticationMetadata`
object with an `impersonationConstraint` field that indicates which constrained impersonation verb
was used to authorize the request.
Example audit event:
```
`{
"kind": "Event",
"apiVersion": "audit.k8s.io/v1",
"user": {
"username": "system:serviceaccount:default:my-controller"
},
"impersonatedUser": {
"username": "jane.doe@example.com"
},
"authenticationMetadata": {
"impersonationConstraint": "impersonate:user-info"
},
"verb": "list",
"objectRef": {
"resource": "pods",
"namespace": "default"
}
}
`
```
The `impersonationConstraint` value indicates which mode was used (for example, `impersonate:user-info`,
`impersonate:associated-node`). The specific action (for example, `list`) can be determined from the
`verb` field in the audit event.
## What's next
* Read about [RBAC authorization](/docs/reference/access-authn-authz/rbac/)
* Understand [Kubernetes authentication](/docs/reference/access-authn-authz/authentication/)