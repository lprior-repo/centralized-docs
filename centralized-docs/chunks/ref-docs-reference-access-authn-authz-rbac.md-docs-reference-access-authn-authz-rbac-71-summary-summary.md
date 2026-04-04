---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#71-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 112
summary: verbs: [\"get\", \"list\", \"watch\", \"create\", \"update\", \"patch\", \"delete\"] --- kind: ClusterRole apiVersion: rbac.authorization.k8s.io/v1 metadata: name: aggregate-cron-tabs-view labels: # Add these...
---

verbs: ["get", "list", "watch", "create", "update", "patch", "delete"]
---
kind: ClusterRole
apiVersion: rbac.authorization.k8s.io/v1
metadata:
name: aggregate-cron-tabs-view
labels:
# Add these permissions to the "view" default role.
rbac.authorization.k8s.io/aggregate-to-view: "true"
rules:
- apiGroups: ["stable.example.com"]
resources: ["crontabs"]
verbs: ["get", "list", "watch"]
`
```