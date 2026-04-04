---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#17-summary
chunk_level: summary
chunk_type: prose
heading: Audit policy
token_count: 126
summary: `apiVersion: audit.k8s.io/v1 # This is required. kind: Policy # Don't generate audit events for all requests in RequestReceived stage. omitStages: - \"RequestReceived\" rules: # Log pod changes at...
---

`apiVersion: audit.k8s.io/v1 # This is required.
kind: Policy
# Don't generate audit events for all requests in RequestReceived stage.
omitStages:
- "RequestReceived"
rules:
# Log pod changes at RequestResponse level
- level: RequestResponse
resources:
- group: ""
# Resource "pods" doesn't match requests to any subresource of pods,
# which is consistent with the RBAC policy.
resources: ["pods"]
# Log "pods/log", "pods/status" at Metadata level
- level: Metadata
resources:
- group: ""