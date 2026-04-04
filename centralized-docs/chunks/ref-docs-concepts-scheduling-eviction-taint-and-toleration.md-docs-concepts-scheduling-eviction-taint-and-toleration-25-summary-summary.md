---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#25-summary
chunk_level: summary
chunk_type: prose
heading: Numeric comparison operators
token_count: 71
summary: * `Gt` matches when the taint value is greater than the toleration value. * `Lt` matches when the taint value is less than the toleration value. For numeric operators, both the toleration and taint...
---

* `Gt` matches when the taint value is greater than the toleration value.
* `Lt` matches when the taint value is less than the toleration value.
For numeric operators, both the toleration and taint values must be valid integers.
If either value cannot be parsed as an integer, the toleration does not match.