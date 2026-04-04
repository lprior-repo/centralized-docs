---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#32-summary
chunk_level: summary
chunk_type: prose
heading: Numeric comparison operators
token_count: 89
summary: * Numeric operators work with all taint effects: `NoSchedule`, `PreferNoSchedule`, and `NoExecute`. * For `PreferNoSchedule` with numeric operators: if a pod's toleration doesn't satisfy the numeric...
---

* Numeric operators work with all taint effects: `NoSchedule`, `PreferNoSchedule`, and `NoExecute`.
* For `PreferNoSchedule` with numeric operators: if a pod's toleration doesn't satisfy the numeric comparison
(e.g., taint value &lt; toleration value when using `Gt`), the scheduler gives the node a lower priority
but may still schedule there if no better options exist.