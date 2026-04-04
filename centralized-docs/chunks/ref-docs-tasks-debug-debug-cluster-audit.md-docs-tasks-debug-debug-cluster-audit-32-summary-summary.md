---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#32-summary
chunk_level: summary
chunk_type: prose
heading: Event batching
token_count: 60
summary: ## Event batching Both `log` and `webhook` backends support batching. Below is a list of available flags specific to each backend. By default, batching and throttling are **enabled** for the...
---

## Event batching
Both `log` and `webhook` backends support batching. Below is a list of
available flags specific to each backend.
By default, batching and throttling are **enabled** for the `webhook` backend and **disabled** for the `log` backend.