---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#30-summary
chunk_level: summary
chunk_type: prose
heading: Audit backends
token_count: 53
summary: ### Webhook backend The webhook audit backend sends audit events to a remote web API, which is assumed to be a form of the Kubernetes API, including means of authentication. You can configure a...
---

### Webhook backend
The webhook audit backend sends audit events to a remote web API, which is assumed to
be a form of the Kubernetes API, including means of authentication. You can configure
a webhook audit backend using the following kube-apiserver flags: