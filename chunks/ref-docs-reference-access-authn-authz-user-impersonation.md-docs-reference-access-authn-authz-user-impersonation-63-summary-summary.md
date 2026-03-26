---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#63-summary
chunk_level: summary
chunk_type: prose
heading: Auditing
token_count: 107
summary: ``` `{ \"kind\": \"Event\", \"apiVersion\": \"audit.k8s.io/v1\", \"user\": { \"username\": \"system:serviceaccount:default:my-controller\" }, \"impersonatedUser\": { \"username\": \"jane.doe@example.com\" },...
---

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