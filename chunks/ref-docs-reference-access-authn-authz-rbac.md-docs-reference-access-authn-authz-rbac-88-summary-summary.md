---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#88-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 86
summary: ### API discovery roles Default cluster role bindings authorize unauthenticated and authenticated users to read API information that is deemed safe to be publicly accessible (including...
---

### API discovery roles
Default cluster role bindings authorize unauthenticated and authenticated users to read API information
that is deemed safe to be publicly accessible (including CustomResourceDefinitions).
To disable anonymous unauthenticated access, add `--anonymous-auth=false` flag to
the API server configuration.
To view the configuration of these roles via `kubectl` run:
```
`kubectl get clusterroles system:discovery -o yaml
`
```