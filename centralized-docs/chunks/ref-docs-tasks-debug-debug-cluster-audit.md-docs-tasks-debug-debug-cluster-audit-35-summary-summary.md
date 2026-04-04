---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#35-summary
chunk_level: summary
chunk_type: prose
heading: Event batching
token_count: 120
summary: * `--audit-webhook-batch-throttle-enable` defines whether batching throttling is enabled. Throttling is enabled by default. * `--audit-webhook-batch-throttle-qps` defines the maximum average number...
---

* `--audit-webhook-batch-throttle-enable` defines whether batching throttling is enabled. Throttling is enabled by default.
* `--audit-webhook-batch-throttle-qps` defines the maximum average number of batches generated
per second. The default value is 10.
* `--audit-webhook-batch-throttle-burst` defines the maximum number of batches generated at the same
moment if the allowed QPS was underutilized previously. The default value is 15.
* `--audit-log-mode` defines the buffering strategy. One of the following: