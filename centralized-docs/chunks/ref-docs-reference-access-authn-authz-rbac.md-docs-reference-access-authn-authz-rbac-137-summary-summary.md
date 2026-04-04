---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#137-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 111
summary: * Test applying a manifest file of RBAC objects, displaying changes that would be made: ``` `kubectl auth reconcile -f my-rbac-rules.yaml --dry-run=client ` ``` * Apply a manifest file of RBAC...
---

* Test applying a manifest file of RBAC objects, displaying changes that would be made:
```
`kubectl auth reconcile -f my-rbac-rules.yaml --dry-run=client
`
```
* Apply a manifest file of RBAC objects, preserving any extra permissions (in roles) and any extra subjects (in bindings):
```
`kubectl auth reconcile -f my-rbac-rules.yaml
`
```
* Apply a manifest file of RBAC objects, removing any extra permissions (in roles) and any extra subjects (in bindings):