---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#9-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 116
summary: * The node must be using [cgroup v2](/docs/concepts/architecture/cgroups/).## Understanding PSI Metrics Pressure Stall Information (PSI) metrics are provided for three resources: CPU, memory, and...
---

* The node must be using [cgroup v2](/docs/concepts/architecture/cgroups/).## Understanding PSI Metrics
Pressure Stall Information (PSI) metrics are provided for three resources: CPU, memory, and I/O. They are categorized into two main types of pressure: `some` and `full`.
* **`some`**: This value indicates that some tasks (one or more) are stalled on a resource. For example, if some tasks are waiting for I/O, this metric will increase. This can be an early indicator of resource contention.