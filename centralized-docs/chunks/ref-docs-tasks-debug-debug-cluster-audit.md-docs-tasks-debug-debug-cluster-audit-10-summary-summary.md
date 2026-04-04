---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#10-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 104
summary: * `RequestReceived` - The stage for events generated as soon as the audit handler receives the request, and before it is delegated down the handler chain. * `ResponseStarted` - Once the response...
---

* `RequestReceived` - The stage for events generated as soon as the audit
handler receives the request, and before it is delegated down the handler
chain.
* `ResponseStarted` - Once the response headers are sent, but before the
response body is sent. This stage is only generated for long-running requests
(e.g. watch).
* `ResponseComplete` - The response body has been completed and no more bytes
will be sent.
* `Panic` - Events generated when a panic occurred.