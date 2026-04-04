---
doc_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness
chunk_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness#18-summary
chunk_level: summary
chunk_type: prose
heading: Mutable Pod scheduling directives
token_count: 91
summary: ## Mutable Pod scheduling directives You can mutate scheduling directives of Pods while they have scheduling gates, with certain constraints. At a high level, you can only tighten the scheduling...
---

## Mutable Pod scheduling directives
You can mutate scheduling directives of Pods while they have scheduling gates, with certain constraints.
At a high level, you can only tighten the scheduling directives of a Pod. In other words, the updated
directives would cause the Pods to only be able to be scheduled on a subset of the nodes that it would
previously match. More concretely, the rules for updating a Pod's scheduling directives are as follows: