---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#66-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 122
summary: ``` `apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRole metadata: name: monitoring-endpointslices labels: rbac.example.com/aggregate-to-monitoring: \"true\" # the rules below will be added to...
---

```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: monitoring-endpointslices
labels:
rbac.example.com/aggregate-to-monitoring: "true"
# the rules below will be added to the "monitoring" ClusterRole.
rules:
- apiGroups: [""]
resources: ["services", "pods"]
verbs: ["get", "list", "watch"]
- apiGroups: ["discovery.k8s.io"]
resources: ["endpointslices"]
verbs: ["get", "list", "watch"]
`
```