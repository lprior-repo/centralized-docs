---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#41-summary
chunk_level: summary
chunk_type: prose
heading: Parameter tuning
token_count: 121
summary: Parameters should be set to accommodate the load on the API server. For example, if kube-apiserver receives 100 requests each second, and each request is audited only on `ResponseStarted` and...
---

Parameters should be set to accommodate the load on the API server.
For example, if kube-apiserver receives 100 requests each second, and each request is audited only
on `ResponseStarted` and `ResponseComplete` stages, you should account for ≅200 audit
events being generated each second. Assuming that there are up to 100 events in a batch,
you should set throttling level at least 2 queries per second. Assuming that the backend can take up to
5 seconds to write events, you should set the buffer size to hold up to 5 seconds of events;