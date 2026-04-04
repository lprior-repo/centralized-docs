---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#16-summary
chunk_level: summary
chunk_type: prose
heading: Concepts
token_count: 122
summary: * Pods that tolerate the taint with a specified `tolerationSeconds` remain bound for the specified amount of time. After that time elapses, the node lifecycle controller evicts the Pods from the...
---

* Pods that tolerate the taint with a specified `tolerationSeconds` remain
bound for the specified amount of time. After that time elapses, the node
lifecycle controller evicts the Pods from the node.`NoSchedule`No new Pods will be scheduled on the tainted node unless they have a matching
toleration. Pods currently running on the node are **not** evicted.`PreferNoSchedule``PreferNoSchedule` is a "preference" or "soft" version of `NoSchedule`.
The control plane will *try* to avoid placing a Pod that does not tolerate