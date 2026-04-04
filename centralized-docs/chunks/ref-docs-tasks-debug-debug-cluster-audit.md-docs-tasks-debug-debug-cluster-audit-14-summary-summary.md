---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#14-summary
chunk_level: summary
chunk_type: prose
heading: Audit policy
token_count: 102
summary: * `RequestResponse` - log events with request metadata, request body and response body. This does not apply for non-resource requests. You can pass a file with the policy to `kube-apiserver` using...
---

* `RequestResponse` - log events with request metadata, request body and response body.
This does not apply for non-resource requests.
You can pass a file with the policy to `kube-apiserver`
using the `--audit-policy-file` flag. If the flag is omitted, no events are logged.
Note that the `rules` field **must** be provided in the audit policy file.
A policy with no (0) rules is treated as illegal.
Below is an example audit policy file: