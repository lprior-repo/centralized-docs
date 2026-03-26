---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#138-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 54
summary: * Apply a manifest file of RBAC objects, removing any extra permissions (in roles) and any extra subjects (in bindings): ``` `kubectl auth reconcile -f my-rbac-rules.yaml --remove-extra-subjects...
---

* Apply a manifest file of RBAC objects, removing any extra permissions (in roles) and any extra subjects (in bindings):
```
`kubectl auth reconcile -f my-rbac-rules.yaml --remove-extra-subjects --remove-extra-permissions
`
```