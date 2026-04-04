---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#33-summary
chunk_level: summary
chunk_type: prose
heading: Numeric comparison operators
token_count: 95
summary: #### Warning: Before disabling the `TaintTolerationComparisonOperators` feature gate: * You should identify all workloads using the `Gt` or `Lt` operators to avoid controller hot-loops. * Update all...
---

#### Warning:
Before disabling the `TaintTolerationComparisonOperators` feature gate:
* You should identify all workloads using the `Gt` or `Lt` operators to avoid controller hot-loops.
* Update all workload controller templates to use `Equal` or `Exists` operators instead
* Delete any pending pods that use `Gt` or `Lt` operators
* Monitor the `apiserver\_request\_total` metric for spikes in validation errors