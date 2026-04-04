---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#37-summary
chunk_level: summary
chunk_type: prose
heading: Event batching
token_count: 106
summary: * `blocking` - block API server responses on processing each individual event. This is the default mode for the `log` backend. * `blocking-strict` - Same as blocking, but when there is a failure...
---

* `blocking` - block API server responses on processing each individual event. This is the default mode for the `log` backend.
* `blocking-strict` - Same as blocking, but when there is a failure during audit logging at the
RequestReceived stage, the whole request to the kube-apiserver fails.
The following flags are used only in the `batch` mode (batching is **disabled** by default for the `log` backend, and when batching is disabled, all batching-related flags are ignored):