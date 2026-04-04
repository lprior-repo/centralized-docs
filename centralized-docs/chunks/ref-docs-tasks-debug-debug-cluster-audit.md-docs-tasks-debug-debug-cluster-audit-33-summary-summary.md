---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#33-summary
chunk_level: summary
chunk_type: prose
heading: Event batching
token_count: 111
summary: * `--audit-webhook-mode` defines the buffering strategy. One of the following: * `batch` - buffer events and asynchronously process them in batches. This is the default mode for the `webhook`...
---

* `--audit-webhook-mode` defines the buffering strategy. One of the following:
* `batch` - buffer events and asynchronously process them in batches. This is the default mode for the `webhook` backend.
* `blocking` - block API server responses on processing each individual event.
* `blocking-strict` - Same as blocking, but when there is a failure during audit logging at the
RequestReceived stage, the whole request to the kube-apiserver fails.
The following flags are used only in the `batch` mode: