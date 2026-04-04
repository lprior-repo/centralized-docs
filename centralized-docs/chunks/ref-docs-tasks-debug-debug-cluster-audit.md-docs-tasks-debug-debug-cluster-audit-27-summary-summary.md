---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#27-summary
chunk_level: summary
chunk_type: prose
heading: Audit backends
token_count: 75
summary: * `--audit-log-path` specifies the log file path that log backend uses to write audit events. Not specifying this flag disables log backend. `-` means standard out * `--audit-log-maxage` defined the...
---

* `--audit-log-path` specifies the log file path that log backend uses to write
audit events. Not specifying this flag disables log backend. `-` means standard out
* `--audit-log-maxage` defined the maximum number of days to retain old audit log files
* `--audit-log-maxbackup` defines the maximum number of audit log files to retain