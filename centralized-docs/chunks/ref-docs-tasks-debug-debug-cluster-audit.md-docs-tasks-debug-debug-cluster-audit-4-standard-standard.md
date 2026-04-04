---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#4-standard
chunk_level: standard
chunk_type: prose
heading: Audit policy
token_count: 319
summary: ## Audit policy Audit policy defines rules about what events should be recorded and what data they should include. The audit policy object structure is defined in the [`audit.k8s.io` API...
---

## Audit policy
Audit policy defines rules about what events should be recorded and what data
they should include. The audit policy object structure is defined in the
[`audit.k8s.io` API group](/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-Policy).
When an event is processed, it's
compared against the list of rules in order. The first matching rule sets the
*audit level* of the event. The defined audit levels are:
* `None` - don't log events that match this rule.
* `Metadata` - log events with metadata (requesting user, timestamp, resource,
verb, etc.) but not request or response body.
* `Request` - log events with request metadata and body but not response body.
This does not apply for non-resource requests.
* `RequestResponse` - log events with request metadata, request body and response body.
This does not apply for non-resource requests.
You can pass a file with the policy to `kube-apiserver`
using the `--audit-policy-file` flag. If the flag is omitted, no events are logged.
Note that the `rules` field **must** be provided in the audit policy file.
A policy with no (0) rules is treated as illegal.
Below is an example audit policy file:
[`audit/audit-policy.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/audit/audit-policy.yaml)![](/images/copycode.svg "Copy audit/audit-policy.yaml to clipboard")