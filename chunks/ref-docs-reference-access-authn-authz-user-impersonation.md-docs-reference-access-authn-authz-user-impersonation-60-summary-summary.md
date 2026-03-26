---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#60-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 91
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