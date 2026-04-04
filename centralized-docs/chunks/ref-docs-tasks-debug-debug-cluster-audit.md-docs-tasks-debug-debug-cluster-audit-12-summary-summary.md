---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#12-summary
chunk_level: summary
chunk_type: prose
heading: Audit policy
token_count: 104
summary: ## Audit policy Audit policy defines rules about what events should be recorded and what data they should include. The audit policy object structure is defined in the [`audit.k8s.io` API...
---

## Audit policy
Audit policy defines rules about what events should be recorded and what data
they should include. The audit policy object structure is defined in the
[`audit.k8s.io` API group](/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-Policy).
When an event is processed, it's
compared against the list of rules in order. The first matching rule sets the
*audit level* of the event. The defined audit levels are: