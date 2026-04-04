---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#19-summary
chunk_level: summary
chunk_type: prose
heading: Audit policy
token_count: 118
summary: - level: None users: [\"system:kube-proxy\"] verbs: [\"watch\"] resources: - group: \"\" # core API group resources: [\"endpoints\", \"services\"] # Don't log authenticated requests to certain non-resource URL...
---

- level: None
users: ["system:kube-proxy"]
verbs: ["watch"]
resources:
- group: "" # core API group
resources: ["endpoints", "services"]
# Don't log authenticated requests to certain non-resource URL paths.
- level: None
userGroups: ["system:authenticated"]
nonResourceURLs:
- "/api\*" # Wildcard matching.
- "/version"
# Log the request body of configmap changes in kube-system.
- level: Request
resources:
- group: "" # core API group
resources: ["configmaps"]