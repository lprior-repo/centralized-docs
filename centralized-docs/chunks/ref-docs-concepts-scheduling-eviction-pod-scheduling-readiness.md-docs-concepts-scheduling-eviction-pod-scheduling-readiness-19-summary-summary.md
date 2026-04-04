---
doc_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness
chunk_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness#19-summary
chunk_level: summary
chunk_type: prose
heading: Mutable Pod scheduling directives
token_count: 45
summary: 1. For `.spec.nodeSelector`, only additions are allowed. If absent, it will be allowed to be set. 2. For `spec.affinity.nodeAffinity`, if nil, then setting anything is allowed.
---

1. For `.spec.nodeSelector`, only additions are allowed. If absent, it will be allowed to be set.
2. For `spec.affinity.nodeAffinity`, if nil, then setting anything is allowed.