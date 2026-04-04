---
doc_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors
chunk_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors#10-summary
chunk_level: summary
chunk_type: prose
heading: Multiple resource types
token_count: 61
summary: ## Multiple resource types You can use field selectors across multiple resource types. This `kubectl` command selects all Statefulsets and Services that are not in the `default` namespace: ```...
---

## Multiple resource types
You can use field selectors across multiple resource types. This `kubectl` command selects all Statefulsets and Services that are not in the `default` namespace:
```
`kubectl get statefulsets,services --all-namespaces --field-selector metadata.namespace!=default
`
```