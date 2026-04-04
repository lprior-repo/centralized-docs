---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#11-standard
chunk_level: standard
chunk_type: prose
heading: Event batching
token_count: 163
summary: * `--audit-log-batch-buffer-size` defines the number of events to buffer before batching. If the rate of incoming events overflows the buffer, events are dropped. * `--audit-log-batch-max-size`...
---

* `--audit-log-batch-buffer-size` defines the number of events to buffer before batching.
If the rate of incoming events overflows the buffer, events are dropped.
* `--audit-log-batch-max-size` defines the maximum number of events in one batch.
* `--audit-log-batch-max-wait` defines the maximum amount of time to wait before unconditionally
batching events in the queue.
* `--audit-log-batch-throttle-enable` defines whether batching throttling is enabled.
* `--audit-log-batch-throttle-qps` defines the maximum average number of batches generated
per second.
* `--audit-log-batch-throttle-burst` defines the maximum number of batches generated at the same
moment if the allowed QPS was underutilized previously.