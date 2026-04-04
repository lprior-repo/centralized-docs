---
doc_id: ref/docs-reference-kubectl-conventions.md/docs-reference-kubectl-conventions
chunk_id: ref/docs-reference-kubectl-conventions.md/docs-reference-kubectl-conventions#7-summary
chunk_level: summary
chunk_type: prose
heading: Using `kubectl` in Reusable Scripts
token_count: 114
summary: * Check in the script for an image that is heavily parameterized. * Switch to configuration files checked into source control for features that are needed, but not expressible via `kubectl run`...
---

* Check in the script for an image that is heavily parameterized.
* Switch to configuration files checked into source control for features that are needed, but not expressible via `kubectl run` flags.
You can use the `--dry-run=client` flag to preview the object that would be sent to your cluster, without really submitting it.
### `kubectl apply`
* You can use `kubectl apply` to create or update resources. For more information about using kubectl apply to update resources, see [Kubectl Book](https://kubectl.docs.kubernetes.io).