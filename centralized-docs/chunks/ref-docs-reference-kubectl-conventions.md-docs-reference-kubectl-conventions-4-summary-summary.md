---
doc_id: ref/docs-reference-kubectl-conventions.md/docs-reference-kubectl-conventions
chunk_id: ref/docs-reference-kubectl-conventions.md/docs-reference-kubectl-conventions#4-summary
chunk_level: summary
chunk_type: prose
heading: Using `kubectl` in Reusable Scripts
token_count: 100
summary: * For `kubectl edit`, the `scale` subresource is not supported. If you use `--subresource` with `kubectl edit` and specify `scale` as the subresource, the command will error out. * The API contract...
---

* For `kubectl edit`, the `scale` subresource is not supported. If you use `--subresource` with
`kubectl edit` and specify `scale` as the subresource, the command will error out.
* The API contract against a subresource is identical to a full resource. While updating the
`status` subresource to a new value, keep in mind that the subresource could be potentially
reconciled by a controller to a different value.## Best Practices