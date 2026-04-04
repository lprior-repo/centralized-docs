---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#64-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 67
summary: ``` `apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRole metadata: name: monitoring aggregationRule: clusterRoleSelectors: - matchLabels: rbac.example.com/aggregate-to-monitoring: \"true\"...
---

```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: monitoring
aggregationRule:
clusterRoleSelectors:
- matchLabels:
rbac.example.com/aggregate-to-monitoring: "true"
rules: [] # The control plane automatically fills in the rules
`
```