---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#70-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 123
summary: `apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRole metadata: name: aggregate-cron-tabs-edit labels: # Add these permissions to the \"admin\" and \"edit\" default roles....
---

`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: aggregate-cron-tabs-edit
labels:
# Add these permissions to the "admin" and "edit" default roles.
rbac.authorization.k8s.io/aggregate-to-admin: "true"
rbac.authorization.k8s.io/aggregate-to-edit: "true"
rules:
- apiGroups: ["stable.example.com"]
resources: ["crontabs"]
verbs: ["get", "list", "watch", "create", "update", "patch", "delete"]
---
kind: ClusterRole