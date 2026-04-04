---
doc_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors
chunk_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors#2-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 73
summary: ``` `kubectl get pods --field-selector status.phase=Running ` ``` #### Note: Field selectors are essentially resource *filters*. By default, no selectors/filters are applied, meaning that all...
---

```
`kubectl get pods --field-selector status.phase=Running
`
```
#### Note:
Field selectors are essentially resource *filters*. By default, no selectors/filters are applied, meaning that all resources of the specified type are selected. This makes the `kubectl` queries `kubectl get pods` and `kubectl get pods --field-selector ""` equivalent.