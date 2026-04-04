---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#13-summary
chunk_level: summary
chunk_type: prose
heading: Audit policy
token_count: 69
summary: * `None` - don't log events that match this rule. * `Metadata` - log events with metadata (requesting user, timestamp, resource, verb, etc.) but not request or response body. * `Request` - log events...
---

* `None` - don't log events that match this rule.
* `Metadata` - log events with metadata (requesting user, timestamp, resource,
verb, etc.) but not request or response body.
* `Request` - log events with request metadata and body but not response body.
This does not apply for non-resource requests.