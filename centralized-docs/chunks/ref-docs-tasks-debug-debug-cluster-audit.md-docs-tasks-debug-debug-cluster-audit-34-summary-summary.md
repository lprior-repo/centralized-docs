---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#34-summary
chunk_level: summary
chunk_type: prose
heading: Event batching
token_count: 111
summary: * `--audit-webhook-batch-buffer-size` defines the number of events to buffer before batching. If the rate of incoming events overflows the buffer, events are dropped. The default value is 10000. *...
---

* `--audit-webhook-batch-buffer-size` defines the number of events to buffer before batching.
If the rate of incoming events overflows the buffer, events are dropped. The default value is 10000.
* `--audit-webhook-batch-max-size` defines the maximum number of events in one batch. The default value is 400.
* `--audit-webhook-batch-max-wait` defines the maximum amount of time to wait before unconditionally
batching events in the queue. The default value is 30 seconds.