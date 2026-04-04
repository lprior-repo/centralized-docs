---
doc_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness
chunk_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness#20-summary
chunk_level: summary
chunk_type: prose
heading: Mutable Pod scheduling directives
token_count: 113
summary: 3. If `NodeSelectorTerms` was empty, it will be allowed to be set. If not empty, then only additions of `NodeSelectorRequirements` to `matchExpressions` or `fieldExpressions` are allowed, and no...
---

3. If `NodeSelectorTerms` was empty, it will be allowed to be set.
If not empty, then only additions of `NodeSelectorRequirements` to `matchExpressions`
or `fieldExpressions` are allowed, and no changes to existing `matchExpressions`
and `fieldExpressions` will be allowed. This is because the terms in
`.requiredDuringSchedulingIgnoredDuringExecution.NodeSelectorTerms`, are ORed
while the expressions in `nodeSelectorTerms[].matchExpressions` and
`nodeSelectorTerms[].fieldExpressions` are ANDed.