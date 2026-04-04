---
doc_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors
chunk_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors#9-summary
chunk_level: summary
chunk_type: prose
heading: Chained selectors
token_count: 96
summary: ## Chained selectors As with [label](/docs/concepts/overview/working-with-objects/labels/) and other selectors, field selectors can be chained together as a comma-separated list. This `kubectl`...
---

## Chained selectors
As with [label](/docs/concepts/overview/working-with-objects/labels/) and other selectors, field selectors can be chained together as a comma-separated list. This `kubectl` command selects all Pods for which the `status.phase` does not equal `Running` and the `spec.restartPolicy` field equals `Always`:
```
`kubectl get pods --field-selector=status.phase!=Running,spec.restartPolicy=Always
`
```