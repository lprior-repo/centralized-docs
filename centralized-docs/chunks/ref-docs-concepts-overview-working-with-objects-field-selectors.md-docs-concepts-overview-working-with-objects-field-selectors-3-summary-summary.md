---
doc_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors
chunk_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors#3-summary
chunk_level: summary
chunk_type: prose
heading: Supported fields
token_count: 108
summary: ## Supported fields Supported field selectors vary by Kubernetes resource type. All resource types support the `metadata.name` and `metadata.namespace` fields. Using unsupported field selectors...
---

## Supported fields
Supported field selectors vary by Kubernetes resource type. All resource types support the `metadata.name` and `metadata.namespace` fields. Using unsupported field selectors produces an error. For example:
```
`kubectl get ingress --field-selector foo.bar=baz
`
```
```
`Error from server (BadRequest): Unable to find "ingresses" that match label selector "", field selector "foo.bar=baz": "foo.bar" is not a known field selector: only "metadata.name", "metadata.namespace"
`
```