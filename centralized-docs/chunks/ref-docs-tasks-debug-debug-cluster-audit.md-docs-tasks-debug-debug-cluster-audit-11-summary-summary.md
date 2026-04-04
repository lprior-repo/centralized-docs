---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#11-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 96
summary: #### Note: The configuration of an [Audit Event configuration](/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-Event) is different from the...
---

#### Note:
The configuration of an
[Audit Event configuration](/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-Event)
is different from the
[Event](/docs/reference/generated/kubernetes-api/v1.35/#event-v1-core)
API object.
The audit logging feature increases the memory consumption of the API server
because some context required for auditing is stored for each request.
Memory consumption depends on the audit logging configuration.