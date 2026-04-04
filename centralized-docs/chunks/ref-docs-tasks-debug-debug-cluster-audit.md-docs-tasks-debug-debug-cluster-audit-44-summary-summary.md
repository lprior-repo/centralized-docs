---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#44-summary
chunk_level: summary
chunk_type: prose
heading: Parameter tuning
token_count: 90
summary: * `audit-log-truncate-max-batch-size` maximum size in bytes of the batch sent to the underlying backend. * `audit-log-truncate-max-event-size` maximum size in bytes of the audit event sent to the...
---

* `audit-log-truncate-max-batch-size` maximum size in bytes of the batch sent to the underlying backend.
* `audit-log-truncate-max-event-size` maximum size in bytes of the audit event sent to the underlying backend.
By default truncate is disabled in both `webhook` and `log`, a cluster administrator should set
`audit-log-truncate-enabled` or `audit-webhook-truncate-enabled` to enable the feature.