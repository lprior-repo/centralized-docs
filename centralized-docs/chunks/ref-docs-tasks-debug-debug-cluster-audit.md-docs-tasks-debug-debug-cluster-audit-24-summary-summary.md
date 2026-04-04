---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#24-summary
chunk_level: summary
chunk_type: prose
heading: Audit backends
token_count: 105
summary: ## Audit backends Audit backends persist audit events to an external storage. Out of the box, the kube-apiserver provides two backends: * Log backend, which writes events into the filesystem *...
---

## Audit backends
Audit backends persist audit events to an external storage.
Out of the box, the kube-apiserver provides two backends:
* Log backend, which writes events into the filesystem
* Webhook backend, which sends events to an external HTTP API
In all cases, audit events follow a structure defined by the Kubernetes API in the
[`audit.k8s.io` API group](/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-Event).