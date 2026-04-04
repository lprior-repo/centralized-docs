---
doc_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors
chunk_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors#8-summary
chunk_level: summary
chunk_type: prose
heading: Supported operators
token_count: 121
summary: ## Supported operators You can use the `=`, `==`, and `!=` operators with field selectors (`=` and `==` mean the same thing). This `kubectl` command, for example, selects all Kubernetes Services that...
---

## Supported operators
You can use the `=`, `==`, and `!=` operators with field selectors (`=` and `==` mean the same thing). This `kubectl` command, for example, selects all Kubernetes Services that aren't in the `default` namespace:
```
`kubectl get services --all-namespaces --field-selector metadata.namespace!=default
`
```
#### Note:
[Set-based operators](/docs/concepts/overview/working-with-objects/labels/#set-based-requirement)
(`in`, `notin`, `exists`) are not supported for field selectors.