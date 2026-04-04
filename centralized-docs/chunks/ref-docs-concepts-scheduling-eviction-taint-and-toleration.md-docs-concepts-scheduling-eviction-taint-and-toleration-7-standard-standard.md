---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#7-standard
chunk_level: standard
chunk_type: prose
heading: Numeric comparison operators
token_count: 150
summary: ## Numeric comparison operators FEATURE STATE: `Kubernetes v1.35 [alpha]`(disabled by default) In addition to `Equal` and `Exists`, you can use numeric comparison operators (`Gt` and `Lt`) to match...
---

## Numeric comparison operators
FEATURE STATE:
`Kubernetes v1.35 [alpha]`(disabled by default)
In addition to `Equal` and `Exists`, you can use numeric comparison operators
(`Gt` and `Lt`) to match taints with integer values. This is useful for threshold-based
scheduling, such as matching nodes by reliability level or SLA tier.
* `Gt` matches when the taint value is greater than the toleration value.
* `Lt` matches when the taint value is less than the toleration value.
For numeric operators, both the toleration and taint values must be valid integers.
If either value cannot be parsed as an integer, the toleration does not match.