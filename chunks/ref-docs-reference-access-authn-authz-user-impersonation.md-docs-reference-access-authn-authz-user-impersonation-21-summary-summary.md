---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#21-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 118
summary: `apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRole metadata: name: scopes-and-uid-impersonator rules: # Can impersonate the user \"jane.doe@example.com\" - apiGroups: [\"\"] resources: [\"users\"]...
---

`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: scopes-and-uid-impersonator
rules:
# Can impersonate the user "jane.doe@example.com"
- apiGroups: [""]
resources: ["users"]
verbs: ["impersonate"]
resourceNames: ["jane.doe@example.com"]
# Can impersonate the groups "developers" and "admins"
- apiGroups: [""]
resources: ["groups"]
verbs: ["impersonate"]
resourceNames: ["developers","admins"]