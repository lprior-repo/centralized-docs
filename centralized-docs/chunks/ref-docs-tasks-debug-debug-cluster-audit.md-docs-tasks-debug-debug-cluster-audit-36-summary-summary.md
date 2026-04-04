---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#36-summary
chunk_level: summary
chunk_type: prose
heading: Event batching
token_count: 70
summary: * `--audit-log-mode` defines the buffering strategy. One of the following: * `batch` - buffer events and asynchronously process them in batches. Batching is not recommended for the `log` backend. *...
---

* `--audit-log-mode` defines the buffering strategy. One of the following:
* `batch` - buffer events and asynchronously process them in batches. Batching is not recommended for the `log` backend.
* `blocking` - block API server responses on processing each individual event. This is the default mode for the `log` backend.