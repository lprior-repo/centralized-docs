---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#39-summary
chunk_level: summary
chunk_type: prose
heading: Event batching
token_count: 59
summary: * `--audit-log-batch-throttle-qps` defines the maximum average number of batches generated per second. * `--audit-log-batch-throttle-burst` defines the maximum number of batches generated at the same...
---

* `--audit-log-batch-throttle-qps` defines the maximum average number of batches generated
per second.
* `--audit-log-batch-throttle-burst` defines the maximum number of batches generated at the same
moment if the allowed QPS was underutilized previously.