---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#24-summary
chunk_level: summary
chunk_type: prose
heading: Numeric comparison operators
token_count: 79
summary: ## Numeric comparison operators FEATURE STATE: `Kubernetes v1.35 [alpha]`(disabled by default) In addition to `Equal` and `Exists`, you can use numeric comparison operators (`Gt` and `Lt`) to match...
---

## Numeric comparison operators
FEATURE STATE:
`Kubernetes v1.35 [alpha]`(disabled by default)
In addition to `Equal` and `Exists`, you can use numeric comparison operators
(`Gt` and `Lt`) to match taints with integer values. This is useful for threshold-based
scheduling, such as matching nodes by reliability level or SLA tier.