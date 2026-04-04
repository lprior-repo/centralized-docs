---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#9-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 124
summary: * where was it observed? * from where was it initiated? * to where was it going? Audit records begin their lifecycle inside the...
---

* where was it observed?
* from where was it initiated?
* to where was it going?
Audit records begin their lifecycle inside the
[kube-apiserver](/docs/reference/command-line-tools-reference/kube-apiserver/)
component. Each request on each stage
of its execution generates an audit event, which is then pre-processed according to
a certain policy and written to a backend. The policy determines what's recorded
and the backends persist the records. The current backend implementations
include logs files and webhooks.
Each request can be recorded with an associated *stage*. The defined stages are: