---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#31-summary
chunk_level: summary
chunk_type: prose
heading: Numeric comparison operators
token_count: 77
summary: * Both the toleration and taint values must be valid signed 64-bit integers (zero leading numbers (e.g., \"0550\") are not allowed). * If a value cannot be parsed as an integer, the toleration does not...
---

* Both the toleration and taint values must be valid signed 64-bit integers
(zero leading numbers (e.g., "0550") are not allowed).
* If a value cannot be parsed as an integer, the toleration does not match.
* Numeric operators work with all taint effects: `NoSchedule`, `PreferNoSchedule`, and `NoExecute`.