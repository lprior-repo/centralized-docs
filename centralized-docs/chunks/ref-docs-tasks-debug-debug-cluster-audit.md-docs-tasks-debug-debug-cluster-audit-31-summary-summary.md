---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#31-summary
chunk_level: summary
chunk_type: prose
heading: Audit backends
token_count: 112
summary: * `--audit-webhook-config-file` specifies the path to a file with a webhook configuration. The webhook configuration is effectively a specialized...
---

* `--audit-webhook-config-file` specifies the path to a file with a webhook
configuration. The webhook configuration is effectively a specialized
[kubeconfig](/docs/tasks/access-application-cluster/configure-access-multiple-clusters/).
* `--audit-webhook-initial-backoff` specifies the amount of time to wait after the first failed
request before retrying. Subsequent requests are retried with exponential backoff.
The webhook config file uses the kubeconfig format to specify the remote address of
the service and credentials used to connect to it.