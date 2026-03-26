---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#136-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 123
summary: ### `kubectl auth reconcile` Creates or updates `rbac.authorization.k8s.io/v1` API objects from a manifest file. Missing objects are created, and the containing namespace is created for namespaced...
---

### `kubectl auth reconcile`
Creates or updates `rbac.authorization.k8s.io/v1` API objects from a manifest file.
Missing objects are created, and the containing namespace is created for namespaced objects, if required.
Existing roles are updated to include the permissions in the input objects,
and remove extra permissions if `--remove-extra-permissions` is specified.
Existing bindings are updated to include the subjects in the input objects,
and remove extra subjects if `--remove-extra-subjects` is specified.
Examples:
* Test applying a manifest file of RBAC objects, displaying changes that would be made: