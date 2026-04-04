---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#18-summary
chunk_level: summary
chunk_type: prose
heading: Audit policy
token_count: 126
summary: # Log \"pods/log\", \"pods/status\" at Metadata level - level: Metadata resources: - group: \"\" resources: [\"pods/log\", \"pods/status\"] # Don't log requests to a configmap called \"controller-leader\" -...
---

# Log "pods/log", "pods/status" at Metadata level
- level: Metadata
resources:
- group: ""
resources: ["pods/log", "pods/status"]
# Don't log requests to a configmap called "controller-leader"
- level: None
resources:
- group: ""
resources: ["configmaps"]
resourceNames: ["controller-leader"]
# Don't log watch requests by the "system:kube-proxy" on endpoints or services
- level: None
users: ["system:kube-proxy"]
verbs: ["watch"]
resources:
- group: "" # core API group