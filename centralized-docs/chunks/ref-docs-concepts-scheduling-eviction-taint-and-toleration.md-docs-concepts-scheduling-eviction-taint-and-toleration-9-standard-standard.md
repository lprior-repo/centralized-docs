---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#9-standard
chunk_level: standard
chunk_type: prose
heading: Numeric comparison operators
token_count: 245
summary: #### Note: When using numeric comparison operators: * Both the toleration and taint values must be valid signed 64-bit integers (zero leading numbers (e.g., \"0550\") are not allowed). * If a value...
---

#### Note:
When using numeric comparison operators:
* Both the toleration and taint values must be valid signed 64-bit integers
(zero leading numbers (e.g., "0550") are not allowed).
* If a value cannot be parsed as an integer, the toleration does not match.
* Numeric operators work with all taint effects: `NoSchedule`, `PreferNoSchedule`, and `NoExecute`.
* For `PreferNoSchedule` with numeric operators: if a pod's toleration doesn't satisfy the numeric comparison
(e.g., taint value &lt; toleration value when using `Gt`), the scheduler gives the node a lower priority
but may still schedule there if no better options exist.
#### Warning:
Before disabling the `TaintTolerationComparisonOperators` feature gate:
* You should identify all workloads using the `Gt` or `Lt` operators to avoid controller hot-loops.
* Update all workload controller templates to use `Equal` or `Exists` operators instead
* Delete any pending pods that use `Gt` or `Lt` operators
* Monitor the `apiserver\_request\_total` metric for spikes in validation errors