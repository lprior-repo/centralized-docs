---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#43-summary
chunk_level: summary
chunk_type: prose
heading: Parameter tuning
token_count: 128
summary: * `apiserver\_audit\_event\_total` metric contains the total number of audit events exported. * `apiserver\_audit\_error\_total` metric contains the total number of events dropped due to an error...
---

* `apiserver\_audit\_event\_total` metric contains the total number of audit events exported.
* `apiserver\_audit\_error\_total` metric contains the total number of events dropped due to an error
during exporting.### Log entry truncation
Both log and webhook backends support limiting the size of events that are logged.
As an example, the following is the list of flags available for the log backend:
* `audit-log-truncate-enabled` whether event and batch truncating is enabled.
* `audit-log-truncate-max-batch-size` maximum size in bytes of the batch sent to the underlying backend.